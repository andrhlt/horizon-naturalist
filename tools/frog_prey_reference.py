"""Capture the pinned Java frog diet and Firefly hit effect headlessly."""
import argparse
from pathlib import Path

import deer_behavior
import java_reference


def capture(command, _):
    prefix = "frog_prey_fixture "
    actual = {}
    for row in command("frog_prey_fixture"):
        if prefix in row:
            entity, edible, hit, glowing = row.split(prefix, 1)[1].split()
            actual[entity] = {"edible": edible == "true", "hit": hit == "true", "glowing_ticks": int(glowing)}
    assert actual.keys() == {"naturalist:firefly", "naturalist:dragonfly"}, actual
    return actual


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "FrogPreyProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "naturalist-frog-mechanics"])
