import argparse
import json
from pathlib import Path

import deer_behavior
import java_reference
from firefly_fixed_light_arena import arena
from firefly_grass_arena import stepper

parser = argparse.ArgumentParser()
for field in ("server", "java_home", "output"):
    parser.add_argument(field, type=Path)
args = parser.parse_args()
args.variants = args.grazing = False
args.behavior = True


def run(command, _):
    step = stepper(command)
    arena(command, step)
    rows = []
    for _ in range(15):
        step(10)
        rows.append(next(json.loads(row.split("firefly_fixed_light ", 1)[1])
                         for row in command("firefly_fixed_light_fixture")
                         if "firefly_fixed_light " in row))
    return rows


deer_behavior.capture = run
with deer_behavior.reference_probe(args.server, args.java_home, Path(__file__).parent / "reference" / "FireflyFixedLightProbe.java"):
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "firefly-fixed-light-fixture"])
