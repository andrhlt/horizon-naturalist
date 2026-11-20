"""Read the Java experience orbs from actual Java Naturalist mob deaths for actual block steps."""
from argparse import ArgumentParser
import json
from pathlib import Path
import time

import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:mob_drops true")
    command("forceload add 0 0")
    time.sleep(2.0)
    command("difficulty normal")
    command("fill 6 149 6 11 149 11 minecraft:stone")
    key = "naturalist_fixture_xp "
    return next(json.loads(line.split(key, 1)[1]) for line in command(key.strip()) if key + "{" in line)


if __name__ == "__main__":
    parser = ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "MobExperienceProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "mob-xp-fully-loaded-fixture"])
