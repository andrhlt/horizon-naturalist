"""Capture pinned Java native particle packets from Whale spray and splash."""
import argparse
import json
import time
from pathlib import Path

import java_reference


def capture(command, _):
    command("forceload add 0 0 15 15")
    time.sleep(1.0)
    marker = "multipart_contact_fixture "
    return next(json.loads(row.split(marker, 1)[1]) for row in command("multipart_contact_fixture") if marker + "{" in row)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "MultipartContactProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "multipart-contact"])
