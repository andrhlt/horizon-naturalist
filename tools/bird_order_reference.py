"""Capture Naturalist 2.0.7 Bird flock and section order observations on an isolated Java server."""
import argparse
import json
from pathlib import Path
import time

import deer_behavior
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 31 15")
    time.sleep(1.0)
    command("tick freeze")
    command("fill 0 92 0 31 92 15 minecraft:stone")
    command("fill 0 93 0 31 98 15 minecraft:air")
    prefix = "bird_fixture_order "
    return next(json.loads(row.split(prefix, 1)[1])
                for row in command("bird_fixture_order") if prefix in row)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "BirdOrderProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "bird-order-fixture"])
