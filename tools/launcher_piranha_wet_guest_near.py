"""Measure Piranha attacks on a submerged LAN player with a dry player nearby."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
ctl = version / "horizon-ctl"
game = version / "horizon-client"
resources = version / ".local/horizon-client/26.3"
swap = "--swap" in sys.argv[2:]
wet_z = float(next((flag.removeprefix("--wet-z=") for flag in sys.argv[2:] if flag.startswith("--wet-z=")), "13.5"))


def call(socket, *args):
    result = subprocess.run(
        [ctl, "--socket", socket, *map(str, args)], capture_output=True, check=True, text=True, timeout=60,
    )
    return json.loads(result.stdout.splitlines()[-1])


with tempfile.TemporaryDirectory(prefix="nat-lan-piranha-target-", ignore_cleanup_errors=True) as tmp:
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
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist piranha nearest wet player", "--creative",
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
        run("gamerule minecraft:natural_regeneration false")
        run("difficulty normal")
        run("tick freeze")
        run("fill 2 91 2 26 91 22 minecraft:stone")
        run("fill 2 92 2 26 100 22 minecraft:air")
        run("fill 2 100 2 26 100 22 minecraft:stone")
        run("fill 5 92 5 21 93 19 minecraft:water")
        run("setblock 8 93 10 minecraft:glass")
        run("setblock 23 93 10 minecraft:glass")
        run("gamemode survival NatHost")
        run("gamemode survival NatGuest")
        dry_name, wet_name = ("NatGuest", "NatHost") if swap else ("NatHost", "NatGuest")
        dry, wet = (guest, host) if swap else (host, guest)
        run(f"effect give {wet_name} minecraft:water_breathing 60 0 true")
        target = "@e[type=naturalist:piranha,tag=wet_guest,limit=1]"

        def sample(name, near_x):
            run(f"tp {dry_name} {near_x} 94 10.5")
            run(f"tp {wet_name} 8.5 92 {wet_z}")
            run(f"effect give {wet_name} minecraft:instant_health 1 5 true")
            guest("wait", 8)
            rows = run("summon naturalist:piranha 8.5 92 10.5 {NoGravity:1b,Rotation:[0.0f,0.0f],Tags:[wet_guest]}")
            assert any("summoned" in row.lower() for row in rows), rows
            times = []
            for _ in range(10):
                run("tick step 10")
                host("wait", 13)
                guest("wait", 3)
                times.append(wet("state")["player"]["health"])
            result = {"health": times, "dry_air": dry("state")["player"]["air"],
                      "wet_air": wet("state")["player"]["air"],
                      "piranha": host("entities", 30, "naturalist:piranha")["entities"]}
            print(name, json.dumps(result), flush=True)
            submerged = wet("state")["player"]["block_pos"]
            assert "water" in str(wet("block", *submerged)), (submerged, result)
            run(f"kill {target}")
            run("tick step 22")
            host("wait", 25)
            return times

        blocked = sample(f"dry {dry_name} closer", 8.5)
        positive = sample(f"wet {wet_name} closer", 23.5)
        assert any(value < 20.0 for value in positive), positive
        if "--baseline" in sys.argv[2:]:
            assert all(value == 20.0 for value in blocked), blocked
        else:
            assert any(value < 20.0 for value in blocked), blocked
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
