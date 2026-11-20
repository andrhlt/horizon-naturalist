"""Exercise the deer standing-player and crouching-player movement in Horizon."""
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile
import time

ctl, game, resources, mod, fixture = map(Path, sys.argv[1:])
reference = json.loads(fixture.read_text())["player_avoidance"]
with tempfile.TemporaryDirectory(prefix="naturalist-player-avoidance-") as tmp:
    d = Path(tmp)
    sock = d/"avoid.sock"

    def call(*args):
        out = subprocess.check_output([ctl,"--socket",sock,*map(str,args)], text=True)
        return json.loads(out.splitlines()[-1])

    def command(value):
        return call("command","/"+value)["chat"]

    try:
        call("launch","--game",game,"--resources",resources,"--home",d,"--game-dir",d,
             "--world","Deer Player Avoidance","--creative","--test-terrain",
             "--render-distance",4,"--dev-mod",mod,"--wait")
        command("tp @s 8.5 102 6.5")
        call("wait",35)
        command("fill 0 100 0 25 100 25 minecraft:stone")
        command("fill 0 101 0 25 105 25 minecraft:air")
        command("gamerule minecraft:spawn_mobs false")
        command("difficulty normal")
        command("time set noon")
        command("gamemode survival")
        command("item replace entity @s weapon.mainhand with minecraft:air")
        results = {}
        for crouching in (True,False):
            command("tp @s 8.5 101 8.5")
            call("key","shift","down" if crouching else "up")
            call("wait",8)
            command("tick freeze")
            trials=[]
            for _ in range(4):
                command("kill @e[tag=avoidance_subject]")
                command("summon naturalist:deer 8.5 101 10.5 {Tags:[avoidance_subject]}")
                command("tick step 30")
                time.sleep(1.65)
                entities = call("entities",20,"naturalist:deer")["entities"]
                assert entities
                x,_,z=entities[0]["pos"]
                trials.append(round(math.hypot(x-8.5,z-8.5),3))
            results["sneaking" if crouching else "standing"]=trials
            command("tick unfreeze")
        print(json.dumps(results))
        observed = {
            "standing": max(results["standing"]) > 3.3,
            "sneaking": sorted(results["sneaking"])[2] >= 2.5,
        }
        assert observed == reference, (observed, reference)
    finally:
        if sock.exists():
            call("quit")
