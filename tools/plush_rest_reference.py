"""Capture bed use with the original Java Plush Bear and paired controls."""
import argparse
from pathlib import Path
import time

import java_reference


def capture(command, _):
    command("forceload add 0 0 15 15")
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:advance_time false")
    command("difficulty normal")
    command("tick freeze")
    command("fill 2 101 2 15 101 15 minecraft:stone")
    command("fill 2 102 2 15 109 15 minecraft:air")
    cases = ("empty_nearby", "main_nearby", "offhand_nearby", "ordinary_item_nearby",
             "inventory_nearby", "day_plush", "empty_no_monsters", "angry_piglin_plush", "calm_piglin_plush", "blocked_plush")
    result = {}
    for name in cases:
        command(f"time set {'noon' if name == 'day_plush' else 'midnight'}")
        command("tick sprint 6")
        time.sleep(0.25)
        marker = f"plush_rest_probe {name} "
        rows = command(f"plush_rest_probe {name}")
        answer = next((row.split(marker, 1)[1] for row in rows if marker in row), None)
        assert answer in ("true", "false"), rows
        result[name] = answer == "true"
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "PlushRestProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "plush-rest-full-parity-fixture"])
