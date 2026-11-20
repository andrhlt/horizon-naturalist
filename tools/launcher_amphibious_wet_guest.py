"""Measure physical player contact with Shark and Whale detached hitboxes in LAN."""
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import re

version = Path(sys.argv[1])
species = sys.argv[2]
options = set(sys.argv[3:])
owner_control = "--wet-owner" in options
assert not owner_control or species == "hippo"
wet_z = float(next((flag.removeprefix("--wet-z=") for flag in sys.argv[3:] if flag.startswith("--wet-z=")), "16.5"))
assert species in ("alligator", "hippo")
wet_top = 93 if "--shallow" in sys.argv[3:] else 94
platform = wet_top if wet_top == 93 else 95
dry_near_x = 6.5 if wet_top == 93 else 8.5
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
        run("time set midnight" if "--night" in sys.argv[3:] else "time set noon")
        run("difficulty normal")
        run("gamerule minecraft:natural_regeneration false")
        run("tick freeze")
        run("fill 2 91 2 26 91 22 minecraft:stone")
        run("fill 2 92 2 26 100 22 minecraft:air")
        run("fill 2 100 2 26 100 22 minecraft:stone")
        run(f"fill 5 92 5 21 {wet_top} 19 minecraft:water")
        run(f"setblock {int(dry_near_x)} {platform} 10 minecraft:glass")
        run(f"setblock 23 {platform} 10 minecraft:glass")
        if "--nest" in options:
            assert species == "alligator" and platform == 93
            run("setblock 6 93 11 minecraft:sand")
            egg = run("setblock 6 94 11 naturalist:alligator_egg")
            assert any("changed" in row.lower() for row in egg), egg
        run("gamemode survival NatHost")
        run("gamemode survival NatGuest")
        target = f"@e[type=naturalist:{species},tag=wet_guest,limit=1]"

        def speed():
            rows = run(f"attribute {target} minecraft:movement_speed get")
            return float(next(m[1] for line in rows if (m := re.search(r" is ([0-9.]+)", line))))

        swapped = "--swap" in sys.argv[2:]
        dry_name, dry_client = ("NatGuest", guest) if swapped else ("NatHost", host)
        wet_name, wet_client = ("NatHost", host) if swapped else ("NatGuest", guest)

        def sample(name, near_x):
            run(f"tp {dry_name} {near_x} {platform + 1} 10.5")
            run(f"tp {wet_name} 8.5 92 {wet_z}")
            run(f"effect give {wet_name} minecraft:instant_health 1 5 true")
            run(f"effect give {wet_name} minecraft:water_breathing 60 0 true")
            wet_client("wait", 8)
            assert "water" in str(wet_client("block", 8, 92, int(wet_z))), wet_client("state")
            before_dry = dry_client("state")["player"]
            assert before_dry["air"] == 300 and before_dry["block_pos"][1] >= platform + 1, before_dry
            fields = "NoGravity:1b,Rotation:[0.0f,0.0f],Tags:[wet_guest]"
            if owner_control:
                owner = next(row.split("following entity data: ", 1)[1] for row in run(f"data get entity {wet_name} UUID")
                             if "following entity data: " in row)
                fields += f",Owner:{owner},Sitting:0b,FollowingOwner:0b"
            rows = run(f"summon naturalist:{species} 8.5 92 10.5 {{{fields}}}")
            assert any("summoned" in row.lower() for row in rows), rows
            times = []
            health = []
            dry_health = []
            for _ in range(10):
                run("tick step 10")
                host("wait", 13)
                guest("wait", 3)
                times.append(speed())
                health.append(wet_client("state")["player"]["health"])
                dry_health.append(dry_client("state")["player"]["health"])
                if ("--positive" in options and health[-1] < 20.0) or ("--nest" in options and dry_health[-1] < 20.0):
                    break
            result = {"speed": times, "dry_air": dry_client("state")["player"]["air"],
                      "wet_air": wet_client("state")["player"]["air"],
                      "dry_pos": dry_client("state")["player"]["block_pos"],
                      "dry_start": before_dry["block_pos"],
                      "health": health, "dry_health": dry_health,
                      "mob": host("entities", 30, f"naturalist:{species}")["entities"]}
            print(name, json.dumps(result), flush=True)
            if not ("--nest" in options and dry_health[-1] < 20.0):
                assert result["dry_air"] == 300 and result["dry_pos"][1] >= platform + 1, result
            run(f"kill {target}")
            run("tick step 22")
            host("wait", 25)
            return health

        blocked = sample("dry player closer", dry_near_x)
        if "--nest" in options:
            if "--expect-nest" in options:
                assert all(value == 20.0 for value in blocked), blocked
            sys.exit(0)
        positive = sample("wet player closer", 23.5)
        paid_damage = lambda history: any(value < 20.0 for value in history)
        if owner_control:
            assert not paid_damage(blocked) and not paid_damage(positive), (blocked, positive)
        elif "--positive" in options:
            assert paid_damage(blocked) and paid_damage(positive), (blocked, positive)
        elif "--negative" in options:
            assert not paid_damage(blocked) and paid_damage(positive), (blocked, positive)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
