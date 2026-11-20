"""Measure original Plush Bear facing and Wild Ones Fox spawn behavior."""
import argparse
import json
from pathlib import Path

import java_reference


def capture(command, _):
    command("forceload add 0 0 15 15")
    command("setblock 8 101 8 minecraft:stone")
    tag = "plush_and_disc_probe "
    for line in command("plush_and_disc_probe"):
        if tag + "{" in line:
            return json.loads(line.split(tag, 1)[1])
    raise RuntimeError("Java Plush Bear and disc fixture did not answer")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "PlushAndDiscProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "plush-disc-parity-fixture"])
