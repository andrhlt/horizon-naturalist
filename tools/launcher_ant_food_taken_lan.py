"""Verify owned Ant capture/release by a headless LAN guest."""
import argparse, hashlib, json, os, shutil, subprocess, tempfile
from pathlib import Path
from prepare import JAR_SHA

def capture(version, jar):
    assert hashlib.sha512(jar.read_bytes()).hexdigest() == JAR_SHA
    with tempfile.TemporaryDirectory(prefix="nat-ant-capture-lan-", ignore_cleanup_errors=True) as work:
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
            (home/"horizon-mods.txt").write_text("mod.naturalist:true\n")
            shutil.copy2(jar,home/"jars"/f"{JAR_SHA}.jar")
            ctl(side,"launch","--headless","--game",version/"horizon-client","--resources",version/".local/horizon-client/26.3","--home",home,"--game-dir",home,"--render-distance",4,*args)
        try:
            launch("host","--world","Ant LAN","--test-terrain","--creative","--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 7.5 100 6.5");ctl("host","wait",25)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 108 15 minecraft:air")
            port=ctl("host","lan","open",0,"commands")["port"]
            launch("guest")
            ctl("guest","connect",f"127.0.0.1:{port}","AntGuest")
            ctl("guest","wait-for","loaded","--timeout",55)
            cmd("tp AntGuest 7.5 100 6.5");cmd("tp @s 12.5 100 6.5")
            cmd("gamemode survival AntGuest");ctl("host","wait",20)
            owner=data("AntGuest","UUID");assert owner
            cmd("gamerule minecraft:mob_drops false")
            for tame in (False, True):
                cmd("kill @e[type=naturalist:ant]");cmd("kill @e[type=minecraft:item]")
                cmd("clear AntGuest");cmd("tp AntGuest 7.5 100 6.5");ctl("host","wait",60)
                ant_id=1 if not tame else 11;food_id=5 if not tame else 15
                ant_uuid=f"[I;{ant_id},2,3,4]";food_uuid=f"[I;{food_id},6,7,8]"
                owned=f",Owner:{owner}" if tame else ""
                cmd(f'summon minecraft:item 7.5 100.55 8.5 {{UUID:{food_uuid},Ant:{ant_uuid},Item:{{id:"minecraft:apple",count:1}},PickupDelay:0s,NoGravity:1b}}')
                cmd(f"summon naturalist:ant 7.5 100 8.5 {{UUID:{ant_uuid},CarriedFood:{food_uuid},NoAI:1b,Tags:[food_ant{int(tame)}],HillCooldown:1200{owned}}}")
                ant=f"@e[tag=food_ant{int(tame)},limit=1]"
                assert data(ant,"CarriedFood")
                cmd("tp AntGuest 7.5 100 8.5")
                for step in range(30):
                    ctl("host","wait",4)
                    if data(ant,"CarriedFood") is None: break
                assert data(ant,"CarriedFood") is None,("not taken",tame)
                anger=data(ant,"angry_at")
                print(json.dumps({"tame":tame,"anger":anger,"guest_uuid":owner}),flush=True)
                assert anger==(None if tame else owner),(tame,anger,owner)
            print("PASS",flush=True)
        finally:
            for side in ("guest","host"):
                if (root/f"{side}.sock").exists():ctl(side,"quit")
if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("version",type=Path);p.add_argument("jar",type=Path);a=p.parse_args();capture(a.version,a.jar)
