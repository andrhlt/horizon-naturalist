"""Observe the bird variant pools selected by Naturalist's Java natural-spawn finalizer."""
import argparse
import json
from pathlib import Path
import time

import deer_behavior
import java_reference

BIOMES = ("taiga", "desert", "plains", "forest", "mushroom_fields")


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    time.sleep(1.0)
    command("tick freeze")
    prefix = "bird_variant_fixture "
    result = {}
    for biome in BIOMES:
        command(f"fillbiome 4 89 4 11 96 11 minecraft:{biome}")
        result[biome] = next(json.loads(row.split(prefix, 1)[1])
                             for row in command("bird_variant_fixture 180") if prefix in row)
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "BirdVariantProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "bird-variant-fixture"])
