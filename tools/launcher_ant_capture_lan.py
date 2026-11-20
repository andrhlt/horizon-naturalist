"""Verify owned Ant capture/release by a headless LAN guest."""
import argparse, hashlib, json, os, shutil, subprocess, tempfile
from pathlib import Path
from prepare import JAR_SHA

def capture(version, jar):
    assert hashlib.sha512(jar.read_bytes()).hexdigest() == JAR_SHA
    with tempfile.TemporaryDirectory(prefix="nat-ant-capture-lan-") as work:
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
            cmd(f'summon naturalist:ant 7.5 100 8.5 {{Tags:[net_ant],Owner:{owner},NoAI:1b,Health:1.5f,Variant:"naturalist:ant",CustomName:"Beacon"}}')
            ant="@e[tag=net_ant,limit=1]";before=data(ant,"UUID");assert before
            cmd("item replace entity AntGuest weapon.mainhand with naturalist:capture_net");ctl("host","wait",8)
            mobs=ctl("guest","entities",8,"naturalist:ant")["entities"];assert len(mobs)==1,mobs
            ctl("guest","look-at","entity",mobs[0]["id"]);ctl("guest","mouse","right","click");ctl("host","wait",8)
            item='Inventory[{id:"naturalist:ant"}].components."minecraft:custom_data"'
            assert data(ant,"UUID") is None
            assert data("AntGuest",item+".Owner")==owner
            assert data("AntGuest",item+".Tame")=="1b"
            assert data("AntGuest",item+".UUID") is None
            ctl("guest","select","naturalist:ant");ctl("guest","look-at-block",7,99,8);ctl("guest","mouse","right","click");ctl("host","wait",8)
            released="@e[type=naturalist:ant,x=7.5,y=100,z=8.5,distance=..1,limit=1]"
            after=data(released,"UUID");assert after and after!=before,(before,after)
            assert data(released,"Owner")==owner
            assert data(released,"Health")=="1.5f"
            assert data(released,"Variant")=='"naturalist:ant"'
            assert data(released,"FromHand")=="1b"
            print(json.dumps({"guest_capture_release":True,"owner_retained":True,"new_uuid":True,"health_variant_hand_traits":True}),flush=True)
        finally:
            for side in ("guest","host"):
                if (root/f"{side}.sock").exists():ctl(side,"quit")
if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("version",type=Path);p.add_argument("jar",type=Path);a=p.parse_args();capture(a.version,a.jar)
