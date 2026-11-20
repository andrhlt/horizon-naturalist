"""Record the pinned Piranha's school capacity, retaliation, and water spawning."""
import argparse
import json
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 16 0")
    command("difficulty normal")
    command("tick freeze")
    command("setblock 8 33 8 minecraft:water")
    command("setblock 8 64 8 minecraft:water")
    command("setblock 7 33 8 minecraft:air")
    command("fill 4 101 8 18 104 14 minecraft:water")
    marker = "piranha_fixture "
    for row in command("piranha_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("Piranha fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "PiranhaProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "piranha-overworld-fixture"])
