"""Capture the original Java scorpions behavior without a client."""
import argparse
import json
import time
from pathlib import Path

from deer_behavior import reference_probe
from java_reference import run_server


def capture(command, _):
    command("forceload add 0 0")
    command("time set noon")
    time.sleep(2.0)
    for row in command("naturalist_fixture_scorpion"):
        if "naturalist_scorpion {" in row:
            return json.loads(row.split("naturalist_scorpion ", 1)[1])
    raise RuntimeError("Java server did not report the scorpion fixture")


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
    with reference_probe(options.server, options.java_home, "ScorpionProbe.java"):
        run_server(options, ["sh", "run.sh", "nogui", "--world", "scorpion-fixture"])
