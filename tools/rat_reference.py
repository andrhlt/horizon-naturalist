"""Measure pinned Rat taming, capture and physical crop-to-container work."""
import argparse
import json
from pathlib import Path
import time
import java_reference


def read(command, case):
    for row in command(f"rat_fixture {case}"):
        if "rat_fixture {" in row:
            return json.loads(row.split("rat_fixture ", 1)[1])
    raise RuntimeError(f"Rat fixture emitted no {case} result")


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:advance_time false")
    command("gamerule minecraft:mob_griefing false")
    command("gamerule minecraft:random_tick_speed 0")
    command("forceload add 0 0 15 15")
    command("tick freeze")
    command("time set 1000")
    command("weather clear")
    command("fill 2 91 2 15 91 14 minecraft:stone")
    command("fill 2 92 2 15 98 14 minecraft:air")
    result = read(command, "setup")
    command("tick sprint 420")
    time.sleep(1.0)
    result["work_no_grief"] = read(command, "observe")
    command("gamerule minecraft:mob_griefing true")
    command("tick sprint 420")
    time.sleep(1.0)
    result["work_griefing"] = read(command, "observe")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "RatProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "rat-parity-fixture"])
