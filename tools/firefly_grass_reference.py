import argparse
import json
from pathlib import Path

import deer_behavior
import java_reference
from firefly_grass_arena import capture, stepper

parser = argparse.ArgumentParser()
for field in ("server", "java_home", "output"):
    parser.add_argument(field, type=Path)
parser.add_argument("--scene", choices=["noon", "canopy"], default="noon")
parser.add_argument("--interval", type=int, choices=[1, 4], default=4)
args = parser.parse_args()
args.variants = args.grazing = False
args.behavior = True
def run(command, _):
    command("firefly_grass_fixture start")
    try:
        result = capture(command, stepper(command), scene=args.scene, interval=args.interval)
        result.update(next(
            json.loads(row.split("firefly_grass_packets ", 1)[1])
            for row in command("firefly_grass_fixture result")
            if "firefly_grass_packets " in row
        ))
        result["reference_light"] = next(
            json.loads(row.split("firefly_grass_light ", 1)[1])
            for row in command("firefly_grass_fixture light")
            if "firefly_grass_light " in row
        )
        return result
    finally:
        command("firefly_grass_fixture stop")


deer_behavior.capture = run
with deer_behavior.reference_probe(args.server, args.java_home, Path(__file__).parent / "reference" / "FireflyGrassProbe.java"):
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", f"firefly-grass-{args.scene}-fixture"])
