"""Read the actual command-spawned profiles in pinned Java."""
import argparse
from pathlib import Path
import re

import deer_behavior
from java_reference import run_server


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    command("tick freeze")
    records = {}
    for case, tag in (("plain", ""), ("nbt", " {Age:0,NoAI:1b,NoGravity:1b,PersistenceRequired:1b}")):
        command(f"summon naturalist:tortoise 6.5 92 6.5{tag}")
        target = "@e[type=naturalist:tortoise,sort=nearest,limit=1]"
        records[case] = {}
        for attribute in ("max_health", "attack_damage"):
            rows = command(f"attribute {target} minecraft:{attribute} base get")
            records[case][attribute] = next(float(match[1]) for row in rows
                                             if (match := re.search(r" is ([-\d.eE+]+)", row)))
        command(f"kill {target}")
        command("tick step 25")
    return records


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    run_server(args, ["sh", "run.sh", "nogui", "--world", "tortoise-profile-reference-fixture"])
