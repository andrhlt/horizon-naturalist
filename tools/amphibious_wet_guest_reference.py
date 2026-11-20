"""Capture pinned Java Alligator and Hippo nearest-player selections."""
import argparse
from functools import partial
import json
import time
from pathlib import Path

import java_reference


def capture(command, _fixture_number, *, shallow, night, nest):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 31 31")
    command("tick freeze")
    command("difficulty normal")
    command("time set midnight" if night else "time set noon")
    command("fill 2 91 2 26 91 22 minecraft:stone")
    command("fill 2 92 2 26 100 22 minecraft:air")
    command("fill 2 100 2 26 100 22 minecraft:stone")
    top, shelf, near_x = (93, 93, 6) if shallow else (94, 95, 8)
    command(f"fill 5 92 5 21 {top} 19 minecraft:water")
    command(f"setblock {near_x} {shelf} 10 minecraft:glass")
    command(f"setblock 23 {shelf} 10 minecraft:glass")
    if nest:
        command("setblock 6 93 11 minecraft:sand")
        command("setblock 6 94 11 naturalist:alligator_egg")
    command("tick step 20")
    time.sleep(1.5)
    marker = "amphibious_wet_guest_fixture "
    operation = "amphibious_shallow_wet_guest_fixture" if shallow else "amphibious_wet_guest_fixture"
    return next(json.loads(row.split(marker, 1)[1]) for row in command(operation) if marker + "{" in row)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    parser.add_argument("--shallow", action="store_true")
    parser.add_argument("--night", action="store_true")
    parser.add_argument("--nest", action="store_true")
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = partial(capture, shallow=args.shallow, night=args.night, nest=args.nest)
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "AmphibiousWetGuestProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "amphibious-wet-guest"])
