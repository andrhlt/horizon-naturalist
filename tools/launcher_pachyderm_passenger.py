"""Compare actual non-player passenger steering during juvenile parent pursuit."""
import argparse
import json
from pathlib import Path
import time

import launcher_pachyderm_family as controller


def capture(cmd, step):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:advance_time false")
    cmd("gamerule minecraft:mob_drops false")
    cmd("forceload add 0 0 31 31")
    cmd("time set 1000")
    cmd("tick freeze")
    for x in range(0, 32, 8):
        for z in range(0, 32, 8):
            cmd(f"fill {x} 99 {z} {x+7} 99 {z+7} minecraft:stone")
            cmd(f"fill {x} 100 {z} {x+7} 106 {z+7} minecraft:air")

    def data(label, field):
        marker = "following entity data: "
        rows = cmd(f"data get entity @e[tag={label},limit=1] {field}")
        return next((row.split(marker, 1)[1] for row in rows if marker in row), None)

    def numbers(label, field):
        value = data(label, field)
        assert value is not None, (label, field)
        return [float(item.rstrip("fd")) for item in value[1:-1].split(", ")]

    results = {}
    for species in ("elephant", "mammoth"):
        for mode, payload in (
            ("unmounted", ""),
            ("chicken", ',Passengers:[{id:"minecraft:chicken",NoAI:1b,Rotation:[90f,30f],Tags:[passenger_case]}]'),
            ("item", ',Passengers:[{id:"minecraft:item",Item:{id:"minecraft:apple",count:1},Tags:[passenger_case]}]'),
        ):
            cmd("kill @e[tag=child_case]")
            cmd("kill @e[tag=parent_case]")
            cmd("kill @e[tag=passenger_case]")
            step(25)
            cmd(f"summon naturalist:{species} 20.5 100 8.5 {{Age:0,NoAI:1b,OnGround:1b,Tags:[parent_case]}}")
            cmd(f"summon naturalist:{species} 6.5 100 8.5 "
                f"{{Age:-10000,OnGround:1b,Rotation:[0f,0f],Tags:[child_case]{payload}}}")
            name = f"{species}_{mode}"
            samples = []
            for ticks in (0, 1, 2, 4, 8):
                if ticks:
                    step(ticks - samples[-1]["tick"])
                passengers = data("child_case", "Passengers") or ""
                height = 1.61 if species == "elephant" else 2.1
                physical = cmd("execute at @e[tag=child_case,limit=1] "
                               f"positioned ~ ~{height} ~-0.1 "
                               "if entity @e[tag=passenger_case,distance=..0.01,limit=1] "
                               "run time query gametime")
                at_seat = any("The game time is" in line for line in physical)
                samples.append({"tick": ticks, "position": numbers("child_case", "Pos"),
                    "rotation": numbers("child_case", "Rotation"), "at_java_world_seat": at_seat,
                    "attached": ("minecraft:" + mode) in passengers if payload else not passengers,
                    "passenger": numbers("passenger_case", "Pos") if data("passenger_case", "Pos") else None})
            results[name] = samples
            print(name, samples, flush=True)
    return results


def compare(expected, actual):
    for kind, gold in expected.items():
        rows = actual[kind]
        assert all(row["attached"] for row in gold + rows), (kind, gold, rows)
        if kind.endswith("_chicken"):
            for expected_row, observed_row in zip(gold[1:], rows[1:], strict=True):
                assert all(abs(x-y) < 0.01 for x, y in
                           zip(expected_row["position"], observed_row["position"], strict=True)), (kind, gold, rows)
                assert expected_row["rotation"] == observed_row["rotation"], (kind, gold, rows)
                assert expected_row["at_java_world_seat"] and observed_row["at_java_world_seat"], (kind, gold, rows)
        elif kind.endswith("_unmounted"):
            # The unrelated idle goals consume random draws, so only the
            # actual pilot case requires exact motion. This path must remain
            # responsive to the juvenile's parent.
            assert rows[-1]["position"][0] - rows[0]["position"][0] > 0.10, (kind, rows)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--java-home", type=Path)
    parser.add_argument("--reference", type=Path)
    args = parser.parse_args()
    controller.capture = capture
    if args.java_home:
        import java_reference

        class ReferenceArgs:
            behavior = True
            variants = grazing = False
            server, java_home, output = args.root, args.java_home, args.output

        def run(cmd, _):
            def step(ticks):
                cmd(f"tick step {ticks}")
                time.sleep(ticks / 20 + 0.12)
            return capture(cmd, step)

        java_reference.deer_behavior.capture = run
        java_reference.run_server(ReferenceArgs(), ["sh", "run.sh", "nogui", "--world",
                                                    "pachyderm-passenger-selector-fixture"])
    else:
        result = controller.capture_horizon(args.root)
        args.output.write_text(json.dumps(result, indent=2) + "\n")
        if args.reference:
            compare(json.loads(args.reference.read_text()), result)
