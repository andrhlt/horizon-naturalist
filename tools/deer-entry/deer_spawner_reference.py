import argparse
import json
from pathlib import Path
import time

import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import deer_behavior
from java_reference import run_server


def capture(command, _, empty_potentials=False):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:spawner_blocks_work true")
    command("forceload add 0 0")
    time.sleep(2)
    command("fillbiome 0 96 0 15 104 15 minecraft:forest")
    command("fill 0 97 0 15 97 15 minecraft:grass_block")
    command("fill 0 98 0 15 104 15 minecraft:air")
    command("tick freeze")
    prefix = "Naturalist Deer entry "
    output = {}
    for seed in (7198391, 22684254, 8430129, 15485863, 708329, 123456789):
        cases = {}
        for mode, tag in (("bare", ""), ("explicit", ",NoAI:1b")):
            command("setblock 8 100 8 air")
            spec = ("minecraft:spawner{Delay:0s,MinSpawnDelay:200s,MaxSpawnDelay:200s,SpawnCount:1s,"
                    "MaxNearbyEntities:6s,RequiredPlayerRange:16s,SpawnRange:4s,"
                    'SpawnData:{entity:{id:"naturalist:deer"' + tag + "},"
                    "custom_spawn_rules:{block_light_limit:{min_inclusive:0,max_inclusive:15},"
                    "sky_light_limit:{min_inclusive:0,max_inclusive:15}}}}")
            if empty_potentials:
                spec = spec.replace("MaxNearbyEntities:", "SpawnPotentials:[],MaxNearbyEntities:")
            created = command(f"setblock 8 100 8 {spec}")
            seeded = command(f"naturalist_fixture_deer_entry seed {seed}")
            assert any("Deer entry seeded" in row for row in seeded), seeded
            result = command("naturalist_fixture_deer_entry spawner")
            rows = command("naturalist_fixture_deer_entry observe")
            data = next((json.loads(row.split(prefix, 1)[1]) for row in rows if prefix in row), None)
            cases[mode] = data
            if data is None:
                print((seed, mode, created, result, rows), flush=True)
        output[seed] = cases
    return output



if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    parser.add_argument("--empty-potentials", action="store_true")
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = lambda command, y: capture(command, y, args.empty_potentials)
    with deer_behavior.reference_probe(args.server, args.java_home, "DeerSpawnerProbe.java"):
        run_server(args, ["sh", "run.sh", "nogui", "--world", "deer-spawner-reference"])
