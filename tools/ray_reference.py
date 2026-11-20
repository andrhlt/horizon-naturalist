"""Record the pinned Ray's retaliation, poison and bucket color."""
import argparse
import json
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 16 0")
    command("difficulty normal")
    command("tick freeze")
    command("fill 5 90 5 13 94 12 minecraft:water")
    marker = "ray_fixture "
    for row in command("ray_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("Ray fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "RayProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "ray-overworld-fixture"])
