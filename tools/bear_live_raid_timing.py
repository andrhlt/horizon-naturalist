"""Observe actual Black Bear campfire navigation in the pinned headless Java mod."""
import argparse
from pathlib import Path
import time

import deer_behavior
from java_reference import run_server


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:mob_griefing true")
    command("gamerule minecraft:advance_time false")
    command("gamerule minecraft:random_tick_speed 0")
    command("time set 0")
    command("forceload add 0 0")
    time.sleep(2.0)
    command("fill 1 100 1 15 100 15 minecraft:stone")
    command("fill 1 101 1 15 106 15 minecraft:air")
    command("setblock 8 100 8 minecraft:dirt")
    for box in ("6 101 6 11 103 6", "6 101 11 11 103 11", "6 101 7 6 103 10", "11 101 7 11 103 10"):
        command(f"fill {box} minecraft:glass")
    trials = []
    bear = "@e[tag=campfire_fixture,limit=1]"
    command("tick freeze")
    try:
        for z in (9.5, 10.25):
            command(f"kill {bear}")
            command("tick step 25")
            time.sleep(1.5)
            command("kill @e[type=minecraft:item]")
            command("setblock 8 101 8 minecraft:air")
            command("setblock 8 101 8 minecraft:campfire[lit=false]")
            command('data merge block 8 101 8 {Items:[{Slot:0b,id:"minecraft:salmon",count:1},'
                    '{Slot:1b,id:"minecraft:beef",count:1}],CookingTimes:[I;0,0,0,0],'
                    'CookingTotalTimes:[I;600,600,0,0]}')
            before = command("data get block 8 101 8 Items")
            assert "minecraft:salmon" in str(before), before
            command(f"summon naturalist:black_bear 8.5 101 {z} {{Tags:[campfire_fixture],PersistenceRequired:1b}}")
            elapsed = 0
            for steps in (40, 5):
                command(f"tick step {steps}")
                time.sleep(steps / 20 + 0.2)
                elapsed += steps
                after = command("data get block 8 101 8 Items")
                trials.append({"start_z": z, "elapsed": elapsed, "stole_salmon": "minecraft:salmon" not in str(after),
                               "left_beef": "minecraft:beef" in str(after)})
    finally:
        command("tick unfreeze")
    return {"trials": trials}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("server", type=Path)
    parser.add_argument("java_home", type=Path)
    parser.add_argument("output", type=Path)
    options = parser.parse_args()
    options.variants = False
    options.grazing = False
    options.behavior = True
    deer_behavior.capture = capture
    run_server(options, ["sh", "run.sh", "nogui", "--world", "bear-live-raid-timing-fixture"])
