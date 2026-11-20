import argparse
import json
from pathlib import Path

import deer_behavior
import java_reference
from dragonfly_contact_arena import capture, stepper, vanilla_entry

parser = argparse.ArgumentParser()
for field in ("server", "java_home", "output"):
    parser.add_argument(field, type=Path)
args = parser.parse_args()
args.variants = args.grazing = False
args.behavior = True
def run(cmd, _):
    wait = stepper(cmd)

    def report():
        return next(json.loads(row.split("dragonfly_splash ", 1)[1])
                    for row in cmd("dragonfly_splash_fixture report")
                    if "dragonfly_splash " in row)

    cmd("dragonfly_splash_fixture clear")
    motion = capture(cmd, wait)
    dragonfly = report()
    cmd("dragonfly_splash_fixture clear")
    return {"motion": motion, "dragonfly": dragonfly, "vanilla": vanilla_entry(cmd, wait, report)}


deer_behavior.capture = run
with deer_behavior.reference_probe(args.server, args.java_home, Path(__file__).parent / "reference" / "DragonflySplashProbe.java"):
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "dragonfly-contact-fixture"])
