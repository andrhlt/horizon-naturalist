"""Record the pinned Catfish's attributes and delayed bass predation."""
import argparse
import json
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    command("fill 6 91 6 10 95 10 minecraft:water")
    command("tick freeze")
    marker = "catfish_fixture "
    for row in command("catfish_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("Catfish fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "CatfishProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "catfish-fixture"])
