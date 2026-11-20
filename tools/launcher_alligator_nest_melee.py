"""Compare both live Alligator nest handoffs to the pinned Java melee action fixture."""
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
reference = json.loads(Path(sys.argv[2]).read_text())



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
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist Alligator nest admission", "--creative",
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

        def fixture(action):
            marker = "Naturalist alligator nest fixture "
            return next(json.loads(line.split(marker, 1)[1])
                        for line in run("naturalist_alligator_nest_fixture " + action) if marker in line)

        def step(ticks):
            run(f"tick step {ticks}")
            host("wait", ticks + 2)

        def health(row):
            return (row["guest_health"], row["host_health"]) if swap else (row["host_health"], row["guest_health"])

        role = "wet_host" if swap else "wet_guest"
        if "--idle" in sys.argv[3:]:
            run("setblock 7 93 10 minecraft:glass")
            run("setblock 5 93 11 minecraft:sand")
            rows = run("setblock 5 94 11 naturalist:alligator_egg")
            assert any("changed" in row.lower() for row in rows), rows
            run(f"tp {dry_name} 7.5 94 10.5")
            run(f"tp {wet_name} 8.5 92.3 12.0")
            wet("wait", 5)
            rows = run("summon naturalist:alligator 8.5 92.3 10.5 {NoGravity:1b,Tags:[nest_probe]}")
            assert any("summoned" in row.lower() for row in rows), rows
            run("attribute @e[tag=nest_probe,limit=1] minecraft:movement_speed base set 0")
            before = fixture("seed 0")
            assert before["target"] is None and health(before) == (20.0, 20.0), before
            step(1)
            after = fixture("inspect")
            print("idle Java nest selection", role, json.dumps(after), flush=True)
            assert after["seed"] is None and after["nest_goal"] and not after["swimmer_goal"], after
            assert after["target"] == dry_name and health(after)[1] == 20.0, after
            sys.exit(0)
        output = {}
        for name, expected in reference[role].items():
            outside = name == "ready_outside"
            dry_z = 11.8 if outside else 10.5
            egg_z = int(dry_z) + 1
            run("tag @e[tag=nest_probe] remove nest_probe")
            run("kill @e[type=naturalist:alligator]")
            step(24)
            run("setblock 5 94 11 minecraft:air")
            run("setblock 5 94 12 minecraft:air")
            run(f"setblock 7 93 {int(dry_z)} minecraft:glass")
            run(f"setblock 5 93 {egg_z} minecraft:sand")
            run(f"tp {dry_name} 7.5 94 {dry_z}")
            run(f"tp {wet_name} 8.5 92.3 14.25")
            for player in (wet_name, dry_name):
                run(f"effect give {player} minecraft:instant_health 1 4 true")
            wet("wait", 8)
            rows = run("summon naturalist:alligator 8.5 92.3 10.5 {NoGravity:1b,Rotation:[0.0f,0.0f],Tags:[nest_probe]}")
            assert any("summoned" in row.lower() for row in rows), rows
            run("attribute @e[tag=nest_probe,limit=1] minecraft:movement_speed base set 0")
            for _ in range(30):
                step(4)
                initial = fixture("inspect")
                if initial["swimmer_goal"] and initial["melee_goal"]:
                    break
            print(name, "initial", json.dumps(initial), flush=True)
            assert initial["target"] == wet_name and initial["swimmer_goal"] and initial["melee_goal"], initial
            assert health(initial) == (20.0, 20.0), initial

            # Match Java's one full GoalSelector turn. Native alternates
            # running-goals-only turns with complete scheduler turns.
            fixture("seed 2")
            for _ in range(8):
                step(1)
                aligned = fixture("inspect")
                if aligned["seed"] is None:
                    break
            assert aligned["seed"] is None and health(aligned) == (20.0, 20.0), (name, aligned)
            step(1)
            assert health(fixture("inspect")) == (20.0, 20.0), name

            run(f"tp {wet_name} 8.5 92.3 12.0")
            wet("wait", 5)
            rows = run(f"setblock 5 94 {egg_z} naturalist:alligator_egg")
            assert any("changed" in row.lower() for row in rows), rows
            seed = 2 if name == "roll_rejected" else 0
            before = fixture(f"seed {seed} {expected['cooldown_before']}")
            step(1)
            row = fixture("inspect")
            print(name, "outcome", json.dumps(row), flush=True)
            assert row["seed"] is None and row["melee_cooldown"] == [expected["cooldown_after"], False], (name, row, expected)
            actor = "wet" if row["target"] == wet_name else "dry" if row["target"] == dry_name else "none"
            selection = {"target": actor, "swimmer_goal": row["swimmer_goal"], "nest_goal": row["nest_goal"]}
            assert selection == expected["selection"], (name, selection, expected)
            assert row["melee_goal"] == expected["melee_after"] and row["position"] == [8.5, 92.3, 10.5], (name, row)
            dry_hp, wet_hp = health(row)
            observed = {
                "selection": selection,
                "cooldown_after": row["melee_cooldown"][0],
                "dry_hp": [health(before)[0], dry_hp],
                "wet_hp": [health(before)[1], wet_hp],
            }
            assert observed["dry_hp"] == expected["dry_hp"][::2] and observed["wet_hp"] == expected["wet_hp"][::2], (name, observed, expected)
            output[name] = observed
        print("matched Java " + role + " " + json.dumps(output, sort_keys=True), flush=True)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
