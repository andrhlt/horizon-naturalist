"""Capture naturally generated Java Ant Hills around a located savanna."""
import argparse
import json
from pathlib import Path
import re

import deer_behavior
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:random_tick_speed 0")
    starts = [(0, 0), (-2600, -2500), (1400, 3300), (4500, -1300), (-5500, 2100)]
    visited = set()
    for sx, sz in starts:
        located = command(f"execute positioned {sx} 72 {sz} run locate biome minecraft:savanna")
        matches = [re.search(r"\[(-?\d+),\s*(?:~|-?\d+),\s*(-?\d+)\]", row) for row in located]
        coords = next((match for match in matches if match is not None), None)
        if coords is None:
            continue
        x, z = map(int, coords.groups())
        for dx, dz in [(0, 0), (96, 0), (0, 96), (-96, 0), (0, -96)]:
            center = (x + dx, z + dz)
            if center in visited:
                continue
            visited.add(center)
            prefix = "ant_natural "
            rows = command(f"ant_natural {center[0]} {center[1]}")
            record = next(json.loads(row.split(prefix, 1)[1]) for row in rows
                          if prefix in row and '{"seed":' in row)
            if record["blocks"]:
                return record
    raise RuntimeError("No naturally generated Ant Hills in the reference sample")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "AntNaturalProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "ant-natural-fixture"])
