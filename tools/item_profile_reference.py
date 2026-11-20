"""Record all default Naturalist items from the pinned NeoForge registry."""
import argparse
import json
from pathlib import Path

import deer_behavior
import java_reference


def capture(command, _):
    key = "naturalist_fixture_item_profiles"
    marker = key + " {"
    return next(json.loads(row.split(key + " ", 1)[1])
                for row in command(key) if marker in row)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "ItemProfileProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "naturalist-item-profile"])
