"""Capture the Java variant selector's ordered outcomes and remaining level RNG state."""
import argparse
import json
from pathlib import Path
import time

import deer_behavior
from java_reference import run_server

BIOMES = ("forest", "taiga", "beach", "warm_ocean")
SEEDS = (42, -82190113, 884783206602988932)


def capture(command, _):
    marker = "naturalist_variant_rng "
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    time.sleep(1.0)
    command("tick freeze")
    result = {}
    for biome in BIOMES:
        command(f"fillbiome 4 89 4 11 96 11 minecraft:{biome}")
        sample = {}
        for seed in SEEDS:
            fields = next(json.loads(line.split(marker, 1)[1])
                          for line in command(f"naturalist_variant_rng {seed}")
                          if marker + "{" in line)
            assert fields["biome"] == f"minecraft:{biome}", fields["biome"]
            sample[str(seed)] = fields
        result[biome] = sample
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "VariantRngProbe.java"):
        run_server(args, ["sh", "run.sh", "nogui", "--world", "variant-rng-fixture"])
