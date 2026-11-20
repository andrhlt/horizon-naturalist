"""Verify Shark target selection with a wet and a dry player in a stock headless LAN world."""
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import re

version = Path(sys.argv[1])
ctl = version / "horizon-ctl"
game = version / "horizon-client"
resources = version / ".local/horizon-client/26.3"


def call(socket, *args):
    result = subprocess.run(
        [ctl, "--socket", socket, *map(str, args)], capture_output=True, check=True, text=True, timeout=60,
    )
    return json.loads(result.stdout.splitlines()[-1])


with tempfile.TemporaryDirectory(prefix="nat-lan-shark-target-", ignore_cleanup_errors=True) as tmp:
    root = Path(tmp)
    h, g = root / "host.sock", root / "guest.sock"
    host = lambda *args: call(h, *args)
    guest = lambda *args: call(g, *args)
    run = lambda line: host("command", line)["chat"]
    try:
        home, guest_home = root / "host", root / "guest"
        home.mkdir()
        guest_home.mkdir()
        host("launch", "--headless", "--game", game, "--resources", resources, "--home", home,
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist shark nearest wet player", "--creative",
             "--test-terrain", "--render-distance", 4, "--wait")
        run("gamerule minecraft:spawn_mobs false")
        run("tp @s 8.5 92 6.5")
        host("wait-ready")
        status = host("lan", "open", 0, "commands")
        guest("launch", "--headless", "--game", game, "--resources", resources, "--home", guest_home,
             "--game-dir", guest_home, "--name", "NatGuest", "--server", f"127.0.0.1:{status['port']}",
             "--render-distance", 4)
        for _ in range(60):
            if g.exists() and guest("state")["screen"] == "playing" and any(
                actor["name"] == "NatGuest" and actor["loaded"] for actor in host("lan", "status")["guests"]
            ):
                break
            time.sleep(0.5)
        else:
            raise RuntimeError("LAN guest did not finish joining")
        run("kill @e[type=!minecraft:player]")
        run("time set midnight")
        run("difficulty normal")
        run("tick freeze")
        run("fill 2 91 2 26 91 22 minecraft:stone")
        run("fill 2 92 2 26 100 22 minecraft:air")
        run("fill 2 100 2 26 100 22 minecraft:stone")
        run("fill 5 92 5 21 94 19 minecraft:water")
        run("setblock 8 95 10 minecraft:glass")
        run("setblock 23 95 10 minecraft:glass")
        run("gamemode survival NatHost")
        run("gamemode survival NatGuest")
        target = "@e[type=naturalist:great_white_shark,tag=wet_guest,limit=1]"

        def speed():
            rows = run(f"attribute {target} minecraft:movement_speed get")
            return float(next(m[1] for line in rows if (m := re.search(r" is ([0-9.]+)", line))))

        swapped = "--swap" in sys.argv[2:]
        dry_name, dry_client = ("NatGuest", guest) if swapped else ("NatHost", host)
        wet_name, wet_client = ("NatHost", host) if swapped else ("NatGuest", guest)

        def sample(name, near_x):
            run(f"tp {dry_name} {near_x} 96 10.5")
            run(f"tp {wet_name} 8.5 92 16.5")
            run(f"effect give {wet_name} minecraft:instant_health 1 4 true")
            for _ in range(8):
                wet_client("wait", 4)
                wet_position = wet_client("state")["player"]["position"]
                dry_position = dry_client("state")["player"]["position"]
                if math.dist(wet_position, (8.5, 92, 16.5)) < 0.5 and math.dist(dry_position, (near_x, 96, 10.5)) < 0.5:
                    break
            else:
                raise AssertionError((name, wet_position, dry_position))
            assert host("block", 8, 93, 16)["block"] == "minecraft:water"
            assert host("block", math.floor(near_x), 97, 10)["block"] == "minecraft:air"
            rows = run("summon naturalist:great_white_shark 8.5 92 10.5 {NoGravity:1b,Rotation:[0.0f,0.0f],Tags:[wet_guest]}")
            assert any("summoned" in row.lower() for row in rows), rows
            times = []
            for _ in range(4):
                run("tick step 10")
                host("wait", 13)
                guest("wait", 3)
                times.append(speed())
            result = {"speed": times, "dry_air": dry_client("state")["player"]["air"],
                      "wet_air": wet_client("state")["player"]["air"],
                      "start_position": {"wet": wet_position, "dry": dry_position},
                      "shark": host("entities", 30, "naturalist:great_white_shark")["entities"]}
            print(name, json.dumps(result), flush=True)
            run(f"kill {target}")
            run("tick step 22")
            host("wait", 25)
            return times

        blocked = sample("dry player closer", 8.5)
        positive = sample("wet player closer", 23.5)
        assert any(value > 1.0 for value in positive), positive
        if "--baseline" in sys.argv[2:]:
            assert all(value == 1.0 for value in blocked), blocked
        else:
            assert any(value > 1.0 for value in blocked), blocked
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
