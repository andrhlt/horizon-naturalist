"""Capture the original Java rhinoceros behavior without a client."""
import argparse
import json
import time
from pathlib import Path

from deer_behavior import reference_probe
from java_reference import run_server


def capture(command, _):
    command("forceload add 0 0")
    command("time set 6000")
    time.sleep(2.0)
    def observations(griefing):
        command(f"gamerule minecraft:mob_griefing {str(griefing).lower()}")
        for row in command("naturalist_fixture_rhino"):
            if "naturalist_rhino {" in row:
                return json.loads(row.split("naturalist_rhino ", 1)[1])
        raise RuntimeError("Java server did not report the Rhino fixture")

    result = observations(True)
    result["no_grief_dandelion_survived"] = not observations(False)["charge_removed_dandelion"]
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("server", type=Path)
    parser.add_argument("java_home", type=Path)
    parser.add_argument("output", type=Path)
    options = parser.parse_args()
    options.variants = False
    options.grazing = False
    options.behavior = True
    import deer_behavior

    deer_behavior.capture = capture
    with reference_probe(options.server, options.java_home, "RhinoProbe.java"):
        run_server(options, ["sh", "run.sh", "nogui", "--world", "rhino-fixture"])
