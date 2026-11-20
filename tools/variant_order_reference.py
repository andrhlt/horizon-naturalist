"""Dump loaded Java variant registry order and weighted pools for each biome."""
import argparse
import json
from pathlib import Path

import deer_behavior
from java_reference import run_server


def capture(command, _):
    prefix = "naturalist_variant_order "
    reports = (json.loads(line.split(prefix, 1)[1])
               for line in command("naturalist_variant_order") if prefix + "{" in line)
    result = {report.pop("species"): report for report in reports}
    assert "naturalist:crab" in result and "naturalist:bird" in result
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "MobVariantOrderProbe.java"):
        run_server(args, ["sh", "run.sh", "nogui", "--world", "variant-order-fixture"])
