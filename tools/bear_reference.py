"""Capture the original Java bears behavior without a client."""
import argparse
import json
import time
from pathlib import Path

from deer_behavior import reference_probe
from java_reference import run_server


def capture(command, _):
    command("forceload add 0 0")
    command("time set 7000")
    time.sleep(2.0)
    for row in command("naturalist_fixture_bear"):
        if "naturalist_bear {" in row:
            return json.loads(row.split("naturalist_bear ", 1)[1])
    raise RuntimeError("Java server did not report the bear fixture")


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
    with reference_probe(options.server, options.java_home, "BearProbe.java"):
        run_server(options, ["sh", "run.sh", "nogui", "--world", "bear-fixture"])
