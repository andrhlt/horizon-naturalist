import argparse
import json
from pathlib import Path
import time

import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import deer_behavior
from java_reference import run_server


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    time.sleep(2)
    command("fillbiome 4 96 4 12 104 12 minecraft:forest")
    command("fill 7 99 7 10 99 10 minecraft:grass_block")
    command("fill 7 100 7 10 104 10 minecraft:air")
    command("tick freeze")
    prefix = "Naturalist Deer entry "
    actions = {
        "summon": "summon naturalist:deer 8.5 100 8.5",
        "summon_nbt": "summon naturalist:deer 8.5 100 8.5 {NoAI:1b}",
        "egg": "naturalist_fixture_deer_entry egg",
    }
    output = {}
    for seed in (7198391, 22684254, 8430129, 15485863, 708329, 123456789):
        cases = {}
        for mode, action in actions.items():
            seeded = command(f"naturalist_fixture_deer_entry seed {seed}")
            assert any("Deer entry seeded" in row for row in seeded), seeded
            result = command(action)
            rows = command("naturalist_fixture_deer_entry observe")
            data = next((json.loads(row.split(prefix,1)[1]) for row in rows if prefix in row), None)
            assert data is not None, (mode, result, rows)
            cases[mode] = data
        output[seed] = cases
    return output



if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "DeerEntryProbe.java"):
        run_server(args, ["sh", "run.sh", "nogui", "--world", "deer-entry-reference"])
