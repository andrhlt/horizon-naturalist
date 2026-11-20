"""Measure original Naturalist hurt voice level metadata in a headless server."""
import argparse
import json
from pathlib import Path

import deer_behavior
from deer_behavior import reference_probe
from java_reference import run_server


def capture(command, _):
    for row in command("naturalist_fixture_voice_levels"):
        if "naturalist_voice_levels {" in row:
            return json.loads(row.split("naturalist_voice_levels ", 1)[1])
    raise RuntimeError("Java did not report original Naturalist voice levels")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("server", type=Path)
    parser.add_argument("java_home", type=Path)
    parser.add_argument("output", type=Path)
    options = parser.parse_args()
    options.variants = False
    options.grazing = False
    options.behavior = True
    deer_behavior.capture = capture
    with reference_probe(options.server, options.java_home, "VoiceLevelsProbe.java"):
        run_server(options, ["sh", "run.sh", "nogui", "--world", "voice-levels-fixture"])
