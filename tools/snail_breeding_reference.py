"""Trace live Java Snail breeding in the same all-stone arena as Horizon."""
import argparse
import json
from pathlib import Path
import time

import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 15 15")
    command("tick freeze")
    command("fill 0 99 0 15 99 15 minecraft:stone")
    command("fill 0 100 0 15 107 15 minecraft:air")
    parents = (
        (7.48961601217193, 8.515575981742105, -0.006390146355735238, 0.009585219533602859, 573),
        (8.995862190386188, 8.482069491673474, -0.0020689048069067718, -0.011034158970169444, 590),
    )
    fields = ("Pos", "Motion", "Age", "InLove", "HasEgg", "OnGround", "ClimbNormalX", "ClimbNormalY", "ClimbNormalZ")
    marker = "following entity data: "

    def sample(label):
        target = f"@e[tag=breeding_{label},limit=1]"
        return {
            field: next((row.split(marker, 1)[1] for row in command(f"data get entity {target} {field}")
                         if marker in row), None)
            for field in fields
        }

    types = ("naturalist:snail", "minecraft:cow" if args.mixed else "naturalist:snail")
    for label, kind, (x, z, vx, vz, love) in zip(("a", "b"), types, parents, strict=True):
        command(f"summon {kind} {x} 100 {z} {{Age:0,InLove:{love},OnGround:1b,"
                f"Motion:[{vx}d,0.0d,{vz}d],Tags:[breeding_{label}]}}")
    trace = []
    for ticks in range(0, 121, 15):
        if ticks:
            command("tick sprint 15")
            time.sleep(0.45)
        row = {"ticks": ticks, "parents": [sample(label) for label in ("a", "b")]}
        print("JAVA SNAILS", json.dumps(row), flush=True)
        trace.append(row)
    carries_egg = any(parent["HasEgg"] in ("1b", "true") for row in trace for parent in row["parents"])
    assert carries_egg != args.mixed, trace
    assert all(int(parent["InLove"] or 0) > 0 for parent in trace[-1]["parents"]) == args.mixed, trace
    return trace


parser = argparse.ArgumentParser(description=__doc__)
for name in ("server", "java_home", "output"):
    parser.add_argument(name, type=Path)
parser.add_argument("--mixed", action="store_true")
args = parser.parse_args()
args.variants = args.grazing = False
args.behavior = True
java_reference.deer_behavior.capture = capture
java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "snail-breeding-mixed-parity-fixture" if args.mixed else "snail-breeding-parity-fixture"])
