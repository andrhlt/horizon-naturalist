"""Measure live Bird trajectories near a survival player on a flat arena."""
import argparse
import json
from pathlib import Path
import time

import deer_behavior
import java_reference


def capture(command, _, approach):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 31 31")
    command("time set noon")
    command("tick freeze")
    command("fill 0 99 0 31 99 31 minecraft:stone")
    command("fill 0 100 0 31 108 31 minecraft:air")
    combined = []
    for trial in range(12):
        command("kill @e[tag=approach_bird]")
        assert any("fixture bird approach ready" in row for row in command(f"fixture_bird_approach {str(approach).lower()} {trial}"))
        command("tick step 6")
        time.sleep(0.40)
        first = query(command, trial)
        command("tick step 18")
        time.sleep(1.0)
        combined.append([first, query(command, trial)])
    return combined


def query(command, trial):
    marker = "following entity data: "
    raw = next(row.split(marker, 1)[-1] for row in command(f"data get entity @e[tag=approach_bird_{trial},limit=1] Pos")
               if marker in row)
    return [float(value.strip().rstrip("d")) for value in raw[1:-1].split(",")]


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    parser.add_argument("--creative-control", action="store_true")
    args = parser.parse_args()
    args.behavior = True
    args.variants = args.grazing = False
    deer_behavior.capture = lambda command, y: capture(command, y, not args.creative_control)
    with deer_behavior.reference_probe(args.server, args.java_home, str(Path(__file__).parent / "reference" / "BirdFleeProbe.java")):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "bird-approach-fixture"])
