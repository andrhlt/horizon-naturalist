"""Exercise naturally formed Piranha school movement in a clean stock build."""
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


with tempfile.TemporaryDirectory(prefix="nat-lan-piranha-school-", ignore_cleanup_errors=True) as tmp:
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
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist piranha natural school", "--creative",
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
        run("fill 7 92 5 10 93 20 minecraft:water")
        run("fill 6 92 5 6 93 20 minecraft:glass")
        run("fill 11 92 5 11 93 20 minecraft:glass")
        run("fill 7 92 4 10 93 4 minecraft:glass")
        run("fill 7 92 21 10 93 21 minecraft:glass")
        run("setblock 6 93 10 minecraft:glass")
        run("setblock 23 93 10 minecraft:glass")
        run("gamemode creative NatHost")
        run("gamemode creative NatGuest")
        dry_name, wet_name = ("NatGuest", "NatHost") if swap else ("NatHost", "NatGuest")
        dry, wet = (guest, host) if swap else (host, guest)
        run(f"tp {dry_name} 6.5 94 10.5")
        run(f"tp {wet_name} 8.5 92 14.25")
        for tag, z in (("school_one", 10.5), ("school_two", 9.5)):
            reply = run(f"summon naturalist:piranha 8.5 92 {z} {{NoGravity:1b,PersistenceRequired:1b,Rotation:[0.0f,0.0f],Tags:[{tag}]}}")
            assert any("summoned" in row.lower() for row in reply), reply
            run(f"attribute @e[tag={tag},limit=1] minecraft:movement_speed base set 0")
        run("tick step 480")
        # Control wait counts the client loop separately from the frozen server clock.
        host("wait", 420)
        host("wait", 80)
        rows = host("entities", 30, "naturalist:piranha")["entities"]
        assert len(rows) == 2 and all(8.0 < fish["pos"][0] < 9.0 for fish in rows), rows
        run("rotate @e[tag=school_one,limit=1] 0 0")
        run("rotate @e[tag=school_two,limit=1] 0 0")
        run("gamemode survival NatHost")
        run("gamemode survival NatGuest")
        run(f"effect give {wet_name} minecraft:water_breathing 120 0 true")
        run(f"effect give {wet_name} minecraft:resistance 120 0 true")
        run("attribute @e[tag=school_one,limit=1] minecraft:movement_speed base set 1.2")
        run("attribute @e[tag=school_two,limit=1] minecraft:movement_speed base set 1.2")
        states = []
        for _ in range(22):
            run("tick step 10")
            host("wait", 13)
            wet("wait", 3)
            dry_health = dry("state")["player"]["health"]
            wet_health = wet("state")["player"]["health"]
            fish = host("entities", 30, "naturalist:piranha")["entities"]
            states.append({"dry": dry_health, "wet": wet_health,
                           "fish": {row["id"]: row["pos"] for row in fish}})
        print(json.dumps(states), flush=True)
        assert any(row["wet"] < 20.0 for row in states), states
        assert all(row["dry"] == 20.0 for row in states), states
        assert any(len(row["fish"]) == 2 and all(pos[2] > 12 for pos in row["fish"].values()) for row in states), states
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
