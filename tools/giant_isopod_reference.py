"""Record Giant Isopod hiding, toughness, breathing, food, and bucket behavior."""
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
    marker = "giant_isopod_fixture "
    for row in command("giant_isopod_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("GiantIsopod fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "GiantIsopodProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "giant_isopod-overworld-fixture"])
