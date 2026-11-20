"""Exercise the Alligator egg item and natural block random ticking headlessly."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
with tempfile.TemporaryDirectory(prefix="nat-gator-eggs-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "game.sock"

    def ctl(*args):
        p = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                           check=True, capture_output=True, text=True, timeout=75)
        return json.loads(p.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    def at(x, y, z, expected):
        return any("Test passed" in line for line in cmd(f"execute if block {x} {y} {z} {expected}"))

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Alligator Eggs", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("tp @s 11.5 92 7.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 3 16 91 14 minecraft:stone")
        cmd("fill 4 92 3 16 98 14 minecraft:air")
        cmd("tp @s 11.5 92 7.5")
        cmd("item replace entity @s weapon.mainhand with naturalist:alligator_egg 4")
        step(3)
        ctl("look-at", 11.5, 91.99, 5.5)
        print("placement crosshair", ctl("looking-at"), flush=True)
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        assert at(11, 92, 5, "naturalist:alligator_egg[hatch=0,eggs=1]"), ctl("block", 11, 92, 5)

        for count in (2, 3, 4):
            ctl("look-at", 11.5, 92.2, 5.5)
            print("stack crosshair", ctl("looking-at"), flush=True)
            ctl("mouse", "right", "click")
            ctl("wait", 4)
            assert at(11, 92, 5, f"naturalist:alligator_egg[eggs={count}]"), ctl("block", 11, 92, 5)
        print("Actual item placed and stacked four eggs on stone.", flush=True)

        cmd("time set 21600")
        cmd("gamerule minecraft:random_tick_speed 4096")
        for _ in range(10):
            step(8)
            if at(11, 92, 5, "minecraft:air"):
                break
        else:
            raise AssertionError(ctl("block", 11, 92, 5))
        offspring = ctl("entities", 10, "naturalist:alligator")["entities"]
        assert len(offspring) == 4, offspring
        print("Normal daytime block random ticks hatched four living Alligators on stone.", flush=True)
    finally:
        if socket.exists():
            ctl("quit")
