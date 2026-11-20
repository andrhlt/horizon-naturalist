import argparse
import json
from pathlib import Path

from deer_behavior import reference_probe
from java_reference import run_server


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("tick freeze")
    prefix = "natural_group_age_fixture "
    rows = command("natural_group_age_fixture")
    return next(json.loads(row.split(prefix, 1)[1]) for row in rows if prefix + "{" in row)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    import deer_behavior
    deer_behavior.capture = capture
    with reference_probe(args.server, args.java_home, "NaturalGroupAgeProbe.java"):
        run_server(args, ["sh", "run.sh", "nogui", "--world", "natural-group-age"])
