"""Replay the reference deer attribute and state checks in Horizon."""
import argparse
from forest_dasher import brew
import deer_variants
import deer_grazing
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("ctl", "game", "resources", "pack", "fixture"):
        parser.add_argument(name, type=lambda s: Path(s).resolve(strict=True))
    task = parser.add_mutually_exclusive_group()
    task.add_argument("--variants", action="store_true")
    task.add_argument("--grazing", action="store_true")
    args = parser.parse_args()
    reference = json.loads(args.fixture.read_text())
    with tempfile.TemporaryDirectory(prefix="deer-replay-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "game.sock"

        def control(*words):
            cmd = [args.ctl, "--socket", socket, *map(str, words)]
            result = subprocess.run(cmd, text=True, capture_output=True, timeout=75)
            if result.returncode:
                raise RuntimeError(f"{words}: {result.stdout} {result.stderr}")
            return json.loads(result.stdout.splitlines()[-1])

        def command(value):
            return control("command", "/" + value)["chat"]

        def start(world="Deer Fixture", terrain=("--test-terrain",)):
            control("launch", "--headless", "--game", args.game, "--resources", args.resources,
                    "--home", home, "--game-dir", home, "--world", world, "--seed", 49903,
                    "--creative", *terrain, "--render-distance", 4, "--dev-mod", args.pack,
                    "--wait")

        def stop():
            try:
                control("quit")
            except (RuntimeError, subprocess.TimeoutExpired):
                return
            deadline = time.monotonic() + 10
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.1)

        def value(reply):
            return next((r.split("following entity data: ", 1)[1]
                         for r in reply if "following entity data: " in r), None)

        adult = "@e[type=naturalist:deer,tag=fixture_deer,limit=1]"
        baby = "@e[type=naturalist:deer,tag=fixture_fawn,limit=1]"
        try:
            start()
            command("tp @s 8.5 101 8.5")
            if args.grazing:
                control("wait", 35)
                observed = deer_grazing.capture(command, 101)
                assert observed == reference, (reference, observed)
                print("Fawn grazing, grass conversion and growth match Java", flush=True)
                return
            if args.variants:
                worlds = [p.parent for p in (home / "saves").rglob("level.dat")]
                if not worlds:
                    command("save-all")
                    worlds = [p.parent for p in (home / "saves").rglob("level.dat")]
                assert len(worlds) == 1, worlds
                deer_variants.write_pack(worlds[0] / "datapacks" / deer_variants.PACK)
                stop()
                start()
                command("tp @s 8.5 101 8.5")
                if not any(deer_variants.PACK in line for line in command("datapack list enabled")):
                    command(f'datapack enable "file/{deer_variants.PACK}"')
                enabled = command("datapack list enabled")
                assert any(deer_variants.PACK in line for line in enabled), enabled
                control("wait", 35)
                observed = deer_variants.capture(command, 101)
                assert observed == reference, (reference, observed)
                stop()
                start()
                tag = "@e[type=naturalist:deer,tag=variant_explicit,limit=1]"
                assert value(command(f"data get entity {tag} Variant")) == json.dumps(reference["explicit"])
                print("Third-party variant selection, inheritance, and saved identity match Java", flush=True)
                return
            control("wait", 32)
            command("fill 6 100 6 13 100 13 minecraft:stone")
            command("tick freeze")
            # Age may advance briefly before the save; the first measurements
            # are made with the server paused at the same point as Java.
            summon = command("summon naturalist:deer 8.5 101 10.5 {NoAI:1b,Tags:[fixture_deer]}")
            assert any("Summoned" in r for r in summon), summon
            command("summon naturalist:deer 9.5 101 10.5 {Age:-24000,NoAI:1b,Tags:[fixture_fawn]}")
            for key in ("max_health", "movement_speed", "step_height"):
                reply = command(f"attribute {adult} minecraft:{key} base get")
                result = next((re.search(r" is ([0-9.eE+-]+)", line)
                               for line in reply if " is " in line), None)
                assert result and abs(float(result[1]) - reference[key]) < 1e-6, (key, reply)
            for key in ("Health", "Age", "Variant"):
                assert value(command(f"data get entity {adult} {key}")) == reference[key], key
            assert value(command(f"data get entity {baby} Age")) == reference["baby_age"]
            identity = value(command(f"data get entity {adult} UUID"))
            assert identity is not None
            print("Adult and fawn match Java attribute and initial state probes", flush=True)
            stop()
            start()
            assert value(command(f"data get entity {adult} UUID")) == identity
            assert value(command(f"data get entity {adult} Variant")) == reference["Variant"]
            fawn = value(command(f"data get entity {baby} Age"))
            assert fawn is not None and int(fawn) < 0, fawn
            print("Both deer survived reload. Adult UUID and Variant were preserved", flush=True)

            command("gamemode creative")
            command("gamerule minecraft:spawn_mobs false")
            command("difficulty normal")
            command("time set midnight")
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("fill 1 101 1 15 104 15 minecraft:air")
            command("tp @s 8.5 106 8.5")
            command("tick freeze")
            command("summon minecraft:zombie 7.5 101 8.5 {NoAI:1b,Tags:[fixture_threat]}")
            def distance(record):
                x, _, z = map(float, re.findall(r"([-+\d.eE]+)d", record))
                return ((x - 7.5) ** 2 + (z - 8.5) ** 2) ** 0.5

            # The live levels have independent mob RNG state. The outcome
            # to compare is fleeing the threat, rather than the chosen path.
            assert distance(reference["avoidance_position"]) > 3.0
            for _ in range(3):
                command("summon naturalist:deer 9.5 101 8.5 {Tags:[fixture_avoid]}")
                command("tick step 35")
                time.sleep(2.0)
                current = value(command("data get entity @e[tag=fixture_avoid,limit=1] Pos"))
                if current and distance(current) > 3.0:
                    break
                command("kill @e[tag=fixture_avoid]")
            else:
                raise AssertionError(f"Deer did not move away from the zombie: {current}")
            print(f"Deer fled the zombie in both engines (Horizon: {current})", flush=True)

            command("kill @e[tag=fixture_threat]")
            command("kill @e[tag=fixture_avoid]")
            command("kill @e[tag=fixture_couple]")
            command("kill @e[type=naturalist:deer,tag=!fixture_deer,tag=!fixture_fawn]")
            command("summon naturalist:deer 6.5 101 12.5 {Age:0,InLove:600,Tags:[fixture_couple,fixture_mate_1]}")
            command("summon naturalist:deer 7.5 101 12.5 {Age:0,InLove:600,Tags:[fixture_couple,fixture_mate_2]}")
            for index in (1, 2):
                mate = f"@e[tag=fixture_mate_{index},limit=1]"
                assert value(command(f"data get entity {mate} InLove")) == "600", mate
            command("tick step 85")
            time.sleep(4.6)
            child = "@e[type=naturalist:deer,tag=!fixture_couple,tag=!fixture_deer,tag=!fixture_fawn,limit=1]"
            reply = command(f"data get entity {child} Age")
            age = value(reply)
            assert -24000 <= int(reference["bred_age"]) < -23900
            assert age is not None and -24000 <= int(age) < -23900, reply
            print(f"Both engines bred a fawn (Java Age {reference['bred_age']}, Horizon Age {age})", flush=True)

            assert brew(command, 12, 101, 11) == reference["forest_dasher"]
            print("All nine Forest Dasher brewing outcomes match Java", flush=True)

            stop()
            start("Natural Herd", ())
            command("tp @s 126 106 128")
            control("wait", 65)
            deer = control("entities", 72, "naturalist:deer")["entities"]
            assert len(deer) >= 3, deer
            print(f"Found {len(deer)} naturally generated forest deer", flush=True)
        finally:
            stop()


if __name__ == "__main__":
    main()
