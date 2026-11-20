"""Capture default body dimensions and baby state while loading the original Java mobs."""
import argparse
import json
from pathlib import Path

import deer_behavior
from java_reference import run_server


def capture(command, _):
    marker = "naturalist_mob_age_profile "
    records = (json.loads(row.split(marker, 1)[1]) for row in command("naturalist_mob_age_profile")
               if marker + "{" in row)
    profiles = {record.pop("id"): record for record in records}
    assert "naturalist:deer" in profiles and "naturalist:giant_isopod" in profiles
    return profiles


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "MobAgeProfileProbe.java"):
        run_server(args, ["sh", "run.sh", "nogui", "--world", "mob-age-profile-fixture"])
