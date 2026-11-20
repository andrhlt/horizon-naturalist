"""Capture Naturalist 2.0.7 Vulture cactus perch on an isolated Java server."""
import argparse
import json
from pathlib import Path
import time

import deer_behavior
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    time.sleep(1.0)
    command("time set midnight")
    command("tick freeze")
    command("fill 0 92 0 15 92 15 minecraft:stone")
    command("fill 0 93 0 15 98 15 minecraft:air")
    command("setblock 8 92 8 minecraft:sand")
    command("setblock 8 93 8 minecraft:cactus")
    command("setblock 8 94 8 minecraft:cactus")
    prefix = "vulture_perch_fixture "
    return next(json.loads(row.split(prefix, 1)[1])
                for row in command("vulture_perch_fixture") if prefix in row)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "VulturePerchProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "vulture-perch-fixture"])
