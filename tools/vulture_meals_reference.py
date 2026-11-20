import argparse
import json
from pathlib import Path

import deer_behavior
import java_reference
from vulture_meals_arena import capture, stepper

parser = argparse.ArgumentParser()
for field in ("server", "java_home", "output"):
    parser.add_argument(field, type=Path)
args = parser.parse_args()
args.variants = args.grazing = False
args.behavior = True
deer_behavior.capture = lambda cmd, _: capture(cmd, stepper(cmd))
java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "vulture-complete-meal-fixture"])
