"""Verify owned Duck capture/release by a headless LAN guest."""
import argparse, hashlib, json, os, shutil, subprocess, tempfile
from pathlib import Path
from prepare import JAR_SHA

def capture(version, jar):
    assert hashlib.sha512(jar.read_bytes()).hexdigest() == JAR_SHA
    with tempfile.TemporaryDirectory(prefix="nat-duck-bucket-lan-", ignore_cleanup_errors=True) as work:
        root=Path(work)
        def ctl(side,*args):
            p=subprocess.run([version/"horizon-ctl","--socket",root/f"{side}.sock",*map(str,args)],cwd=root,env={**os.environ,"HORIZON_NO_AUDIO":"1"},capture_output=True,text=True,timeout=80)
            assert p.returncode==0,(args,p.stdout,p.stderr)
            return json.loads(p.stdout.splitlines()[-1])
        def cmd(line):
            r=ctl("host","command",line)
            assert r["ok"],(line,r)
            return r["chat"]
        def data(target,key):
            return next((r.split("following entity data: ",1)[1] for r in cmd(f"data get entity {target} {key}") if "following entity data: " in r),None)
        def launch(side,*args):
            home=root/side; (home/"jars").mkdir(parents=True)
            shutil.copy2(jar,home/"jars"/f"{JAR_SHA}.jar")
            ctl(side,"launch","--headless","--game",version/"horizon-client","--resources",version/".local/horizon-client/26.3","--home",home,"--game-dir",home,"--render-distance",4,*args)
        try:
            launch("host","--world","Duck LAN","--test-terrain","--creative","--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 7.5 100 6.5");ctl("host","wait",25)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 108 15 minecraft:air")
            port=ctl("host","lan","open",0,"commands")["port"]
            launch("guest")
            ctl("guest","connect",f"127.0.0.1:{port}","DuckGuest")
            ctl("guest","wait-for","loaded","--timeout",55)
            cmd("tp DuckGuest 7.5 100 6.5");cmd("tp @s 12.5 100 6.5")
            cmd("gamemode survival DuckGuest");ctl("host","wait",20)
            host_owner=data("@s","UUID");assert host_owner
            cmd('setblock 7 99 8 minecraft:water')
            cmd(f'summon naturalist:duck 7.5 100 8.5 {{Tags:[foreign_duck],Owner:{host_owner},NoAI:1b,NoGravity:1b,Age:-12000}}')
            target="@e[tag=foreign_duck,limit=1]";before=data(target,"UUID");assert before
            cmd("item replace entity DuckGuest weapon.mainhand with naturalist:whistle");cmd("item replace entity DuckGuest weapon.offhand with minecraft:bucket");ctl("host","wait",10)
            mobs=ctl("guest","entities",8,"naturalist:duck")["entities"];assert len(mobs)==1,mobs
            ctl("guest","look-at","entity",mobs[0]["id"])
            print("aim",json.dumps(ctl("guest","looking-at")),flush=True)
            ctl("guest","mouse","right","click");ctl("host","wait",12)
            inv=json.dumps(ctl("guest","inventory"));after=data(target,"UUID")
            print(json.dumps({"inventory":inv,"same_duck":after==before}),flush=True)
            assert after==before and "minecraft:water_bucket" in inv,(after,before,inv)
            ctl("guest","look-at-block",7,99,7,"up");ctl("guest","mouse","right","click");ctl("host","wait",8)
            inv=json.dumps(ctl("guest","inventory"));print(json.dumps({"after_second":inv}),flush=True)
            assert "minecraft:water_bucket" not in inv and 'minecraft:bucket' in inv,inv
        finally:
            for side in ("guest","host"):
                if (root/f"{side}.sock").exists():ctl(side,"quit")
            for log in root.rglob('*.log'):
                lines=[x for x in log.read_text(errors='replace').splitlines() if 'DUCKPET_' in x]
                if lines: print('\n'.join(lines),flush=True)
if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("version",type=Path);p.add_argument("jar",type=Path);a=p.parse_args();capture(a.version,a.jar)
