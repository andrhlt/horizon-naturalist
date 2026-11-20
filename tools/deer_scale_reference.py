"""Read the Java deer's scale, hitbox and eye-height behavior."""
from argparse import ArgumentParser
import json
from pathlib import Path

import java_reference


def capture(command, _):
    key = "naturalist_fixture_deer_scale "
    return next(json.loads(line.split(key, 1)[1]) for line in command(key.strip()) if key + "[" in line)


if __name__ == "__main__":
    parser = ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "DeerScaleProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "deer-scale-fixture"])
