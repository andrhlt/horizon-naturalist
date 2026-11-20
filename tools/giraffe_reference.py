"""Capture Giraffe feeding, taming and actual riding in the pinned Java mod."""
import argparse
import json
from pathlib import Path
import time

import deer_behavior
from java_reference import run_server


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    time.sleep(1.0)
    command("tick freeze")
    result = command("naturalist_giraffe_fixture")
    prefix = "naturalist_giraffe_fixture "
    return next(json.loads(line.split(prefix, 1)[1]) for line in result if prefix + "{" in line)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "GiraffeProbe.java"):
        run_server(args, ["sh", "run.sh", "nogui", "--world", "giraffe-reference-fixture"])
