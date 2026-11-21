"""Verify owned Crab capture/release by a headless LAN guest."""
import argparse, hashlib, json, os, shutil, subprocess, tempfile
from pathlib import Path
from prepare import JAR_SHA

def capture(version, jar):
    assert hashlib.sha512(jar.read_bytes()).hexdigest() == JAR_SHA
    with tempfile.TemporaryDirectory(prefix="nat-crab-capture-lan-", ignore_cleanup_errors=True) as work:
        root=Path(work)
        def ctl(side,*args):
            p=subprocess.run([version/"horizon-ctl","--socket",root/f"{side}.sock",*map(str,args)],cwd=root,env={**os.environ,"HORIZON_HOME":str(root/side),"HORIZON_NO_AUDIO":"1"},capture_output=True,text=True,timeout=80)
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
            launch("host","--world","Duck pet LAN","--test-terrain","--creative","--wait")
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
            owner=data("DuckGuest","UUID");assert owner
            host_owner=data("@s","UUID");assert host_owner
            cmd(f'summon naturalist:duck 7.5 100 8.5 {{Tags:[whistle_duck],Owner:{owner},NoAI:1b,Health:3f,FollowingOwner:1b,Sitting:1b}}')
            crab="@e[tag=whistle_duck,limit=1]"
            cmd("item replace entity DuckGuest weapon.mainhand with naturalist:whistle");ctl("host","wait",8)
            mobs=[m for m in ctl("guest","entities",8,"naturalist:duck")["entities"] if m["health"]>0];assert len(mobs)==1,mobs
            ctl("guest","look-at","entity",mobs[0]["id"]);ctl("guest","mouse","right","click");ctl("host","wait",8)
            first={k:data(crab,k) for k in ("FollowingOwner","Sitting")}
            assert first=={"FollowingOwner":"0b","Sitting":"0b"},first
            ctl("host","wait",25)
            ctl("guest","look-at","entity",mobs[0]["id"]);ctl("guest","mouse","right","click");ctl("host","wait",8)
            second={k:data(crab,k) for k in ("FollowingOwner","Sitting")}
            assert second=={"FollowingOwner":"1b","Sitting":"0b"},second
            cmd(f"kill {crab}")
            cmd(f'summon naturalist:duck 7.5 100 8.5 {{Tags:[whistle_duck],Owner:{host_owner},NoAI:1b,Health:3f,FollowingOwner:1b,Sitting:0b}}')
            ctl("host","wait",25)
            mobs=[m for m in ctl("guest","entities",8,"naturalist:duck")["entities"] if m["health"]>0];assert len(mobs)==1,mobs
            ctl("guest","look-at","entity",mobs[0]["id"]);ctl("guest","mouse","right","click");ctl("host","wait",8)
            nonowner={k:data(crab,k) for k in ("FollowingOwner","Sitting")}
            assert nonowner=={"FollowingOwner":"1b","Sitting":"0b"},nonowner
            cmd(f"kill {crab}")
            cmd(f'summon naturalist:duck 7.5 100 8.5 {{Tags:[whistle_duck],Owner:{owner},NoAI:1b,Health:3f,FollowingOwner:1b,Sitting:0b}}')
            ctl("host","wait",10)
            mobs=[m for m in ctl("guest","entities",8,"naturalist:duck")["entities"] if m["health"]>0];assert len(mobs)==1,mobs
            ctl("host","wait",25)
            ctl("guest","key","shift","down")
            ctl("guest","look-at","entity",mobs[0]["id"]);ctl("guest","mouse","right","click")
            ctl("guest","key","shift","up");ctl("host","wait",8)
            sneaking={k:data(crab,k) for k in ("FollowingOwner","Sitting")}
            assert sneaking=={"FollowingOwner":"1b","Sitting":"1b"},sneaking
            cmd("item replace entity DuckGuest weapon.mainhand with minecraft:air")
            ctl("host","wait",25)
            ctl("guest","look-at","entity",mobs[0]["id"]);ctl("guest","mouse","right","click");ctl("host","wait",8)
            sitting={k:data(crab,k) for k in ("FollowingOwner","Sitting")}
            assert sitting=={"FollowingOwner":"1b","Sitting":"0b"},sitting
            ctl("guest","look-at","entity",mobs[0]["id"]);ctl("guest","mouse","right","click");ctl("host","wait",8)
            later={k:data(crab,k) for k in ("FollowingOwner","Sitting")}
            assert later=={"FollowingOwner":"1b","Sitting":"1b"},later
            print(json.dumps({"first":first,"second":second,"nonowner":nonowner,"sneaking":sneaking}),flush=True)
        finally:
            for log in root.rglob('*.log'):
                for line in log.read_text(errors='replace').splitlines():
                    if 'DUCKPET_' in line: print(log, line,flush=True)
            for side in ("guest","host"):
                if (root/f"{side}.sock").exists():ctl(side,"quit")
if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("version",type=Path);p.add_argument("jar",type=Path);a=p.parse_args();capture(a.version,a.jar)
