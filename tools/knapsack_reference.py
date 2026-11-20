"""Capture Java's loaded Knapsack mechanics through actual item interactions."""
import argparse
import json
from pathlib import Path

import java_reference


def capture(command, _):
    command("forceload add 0 0 15 15")
    command("gamerule minecraft:spawn_mobs false")
    command("tick freeze")
    command("fill 2 101 2 15 101 15 minecraft:stone")
    command("fill 2 102 2 15 109 15 minecraft:air")
    command("setblock 9 102 8 minecraft:stone")
    marker = "knapsack_probe "
    result = next((json.loads(row.split(marker, 1)[1]) for row in command("knapsack_probe") if marker + "{" in row), None)
    assert result is not None
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "KnapsackProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "knapsack-parity-fixture"])
