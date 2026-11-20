"""Record the pinned AquaticSpawn's retaliation, poison and bucket color."""
import argparse
import json
from pathlib import Path
import java_reference


def capture(command, _):
    marker = "aquatic_spawn_fixture "
    for row in command("aquatic_spawn_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("AquaticSpawn fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "AquaticSpawnProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "aquatic_spawn-overworld-fixture"])
