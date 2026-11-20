"""Record the pinned Anglerfish's spawn, hunting cooldown and bucket color."""
import argparse
import time
import json
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 16 0")
    command("difficulty normal")
    command("fill 2 22 2 27 33 14 minecraft:stone")
    command("fill 5 25 5 22 31 11 minecraft:water")
    command("fill 14 23 3 14 32 13 minecraft:stone")
    command("setblock 21 27 8 minecraft:sea_lantern")
    command("fill 5 39 5 12 41 12 minecraft:water")
    command("fill 5 53 5 12 55 12 minecraft:water")
    command("fill 5 90 5 13 94 12 minecraft:water")
    time.sleep(2)
    command("tick freeze")
    marker = "anglerfish_fixture "
    for row in command("anglerfish_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("Anglerfish fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "AnglerfishProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "anglerfish-overworld-fixture"])
