"""Exercise an uninstrumented late nest takeover with two actual LAN players."""
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



def call(socket, *args):
    result = subprocess.run(
        [ctl, "--socket", socket, *map(str, args)], capture_output=True, check=True, text=True, timeout=60,
    )
    return json.loads(result.stdout.splitlines()[-1])


with tempfile.TemporaryDirectory(prefix="nat-lan-alligator-nest-", ignore_cleanup_errors=True) as tmp:
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
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist Alligator natural nest handoff", "--creative",
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
        rules = run("gamerule minecraft:natural_health_regeneration false")
        assert any("false" in result for result in rules), rules
        run("difficulty normal")
        run("tick freeze")
        run("fill 2 91 2 26 91 22 minecraft:stone")
        run("fill 2 92 2 26 100 22 minecraft:air")
        run("fill 2 100 2 26 100 22 minecraft:stone")
        run("fill 5 92 5 21 93 19 minecraft:water")
        run("setblock 23 93 10 minecraft:glass")
        run("gamemode survival NatHost")
        run("gamemode survival NatGuest")
        dry_name, wet_name = ("NatGuest", "NatHost") if swap else ("NatHost", "NatGuest")
        dry, wet = (guest, host) if swap else (host, guest)
        run(f"effect give {wet_name} minecraft:water_breathing 300 0 true")

        run("time set noon")

        run("setblock 7 93 10 minecraft:glass")
        run("setblock 5 93 11 minecraft:sand")
        run(f"tp {dry_name} 7.5 94 10.5")
        run(f"tp {wet_name} 8.5 92.3 12.0")
        wet("wait", 5)
        rows = run("summon naturalist:alligator 8.5 92.3 10.5 {NoGravity:1b,Tags:[nest_probe]}")
        assert any("summoned" in row.lower() for row in rows), rows
        run("attribute @e[tag=nest_probe,limit=1] minecraft:movement_speed base set 0")

        def hp():
            return dry("state")["player"]["health"], wet("state")["player"]["health"]

        def sample():
            run("tick step 6")
            host("wait", 9)
            guest("wait", 3)
            return hp()

        pursuit = []
        for _ in range(24):
            row = sample()
            pursuit.append(row)
            if row[1] < 20:
                break
        print("ordinary swimmer pursuit", json.dumps(pursuit), flush=True)
        assert pursuit[-1] == (20.0, 12.0), pursuit

        run(f"tp {wet_name} 8.5 92.3 14.25")
        wet("wait", 5)
        run(f"effect give {wet_name} minecraft:instant_health 1 4 true")
        wet("wait", 5)
        assert hp() == (20.0, 20.0), hp()
        rows = run("setblock 5 94 11 naturalist:alligator_egg")
        assert any("changed" in row.lower() for row in rows), rows
        takeover = []
        for _ in range(36):
            row = sample()
            takeover.append(row)
            if row[0] < 20:
                break
        print("nest takeover", json.dumps(takeover), flush=True)
        assert takeover[-1] == (12.0, 20.0) and all(row[1] == 20 for row in takeover), takeover
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
