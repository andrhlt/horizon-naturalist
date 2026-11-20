"""Record the pinned Starfish's contact damage, swim pulse and bucket color."""
import argparse
import json
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 16 0")
    command("difficulty normal")
    command("tick freeze")
    command("fill 4 91 3 14 91 15 minecraft:stone")
    command("fill 4 92 3 14 98 15 minecraft:air")
    command("fill 5 92 8 14 95 15 minecraft:water")
    command("fill 5 29 5 12 29 12 minecraft:stone")
    command("fill 5 30 5 12 31 12 minecraft:water")
    command("setblock 8 62 8 minecraft:stone")
    command("setblock 8 63 8 minecraft:water")
    command("setblock 7 92 8 minecraft:water")
    marker = "starfish_fixture "
    for row in command("starfish_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("Starfish fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "StarfishProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "starfish-overworld-fixture"])
