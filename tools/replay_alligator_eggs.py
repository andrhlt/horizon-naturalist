"""Exercise Alligator egg placement and natural hatching headlessly."""
import json
from pathlib import Path
import subprocess
import sys
import time
from launcher_selected_headless import LAUNCHER, SelectedHeadless

version = Path(sys.argv[1])
world = f"APP2129 Alligator Eggs {time.time_ns()}"
socket = Path("/private/tmp") / f"{world.replace(' ', '-')}.sock"
client = SelectedHeadless(version, socket, world, "--creative")
ctl = client.call
cmd = client.command

def step(ticks):
    cmd(f"tick step {ticks}")
    ctl("wait", ticks + 2)

def at(x, y, z, expected):
    return any("Test passed" in line for line in cmd(f"execute if block {x} {y} {z} {expected}"))

def at_dimension(dimension, x, y, z, expected):
    return any("Test passed" in line for line in cmd(
        f"execute in {dimension} if block {x} {y} {z} {expected}"))

def alligator_near(dimension, x, y, z):
    return any("Test passed" in line for line in cmd(
        f"execute in {dimension} positioned {x + 0.5} {y + 0.5} {z + 0.5} "
        "if entity @e[type=naturalist:alligator,distance=..8]"))

try:
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:random_tick_speed 0")
    cmd("tp @s 11.5 92 7.5")
    ctl("wait", 20)
    cmd("tick freeze")
    cmd("fill 4 91 3 16 91 14 minecraft:stone")
    cmd("fill 4 92 3 16 98 14 minecraft:air")
    cmd("tp @s 11.5 92 7.5")
    cmd("item replace entity @s weapon.mainhand with naturalist:alligator_egg 4")
    step(3)
    ctl("look-at", 11.5, 91.99, 5.5)
    placement_crosshair = ctl("looking-at")
    ctl("mouse", "right", "click")
    ctl("wait", 4)
    assert at(11, 92, 5, "naturalist:alligator_egg[hatch=0,eggs=1]"), ctl("block", 11, 92, 5)

    for count in (2, 3, 4):
        ctl("look-at", 11.5, 92.2, 5.5)
        stacking_crosshair = ctl("looking-at")
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        assert at(11, 92, 5, f"naturalist:alligator_egg[eggs={count}]"), ctl("block", 11, 92, 5)
    cmd("time set 21600")
    cmd("gamerule minecraft:random_tick_speed 4096")
    for _ in range(10):
        step(8)
        if at(11, 92, 5, "minecraft:air"):
            break
    else:
        raise AssertionError(ctl("block", 11, 92, 5))
    offspring = ctl("entities", 10, "naturalist:alligator")["entities"]
    assert len(offspring) == 4, offspring
    age_rows = cmd("execute as @e[type=naturalist:alligator,distance=..10] run data get entity @s Age")
    ages = []
    for row in age_rows:
        try:
            ages.append(int(row.rsplit(": ", 1)[1].replace(",", "")))
        except (IndexError, ValueError):
            pass
    assert len(ages) == 4 and all(age <= 0 for age in ages), {"offspring": offspring, "age_rows": age_rows, "ages": ages}

    # Java's AlligatorEggBlock hatches in the level receiving randomTick.
    # Keep the egg far from the Overworld hatch above so a misplaced hatchling
    # cannot satisfy the wrong-dimension assertion.
    cmd("execute in minecraft:the_nether run forceload add 0 0 63 63")
    cmd("execute in minecraft:the_nether run fill 47 91 47 49 91 49 minecraft:stone")
    cmd("execute in minecraft:the_nether run fill 47 92 47 49 98 49 minecraft:air")
    cmd("execute in minecraft:the_nether run setblock 48 92 48 naturalist:alligator_egg[eggs=1]")
    for _ in range(100):
        step(8)
        if at_dimension("minecraft:the_nether", 48, 92, 48, "minecraft:air"):
            break
    else:
        raise AssertionError({"nether_egg": cmd("execute in minecraft:the_nether run data get block 48 92 48")})
    nether_hatchling = alligator_near("minecraft:the_nether", 48, 92, 48)
    wrong_dimension_hatchling = alligator_near("minecraft:overworld", 48, 92, 48)
    assert nether_hatchling and not wrong_dimension_hatchling, {
        "nether_hatchling": nether_hatchling,
        "wrong_dimension_hatchling": wrong_dimension_hatchling,
    }
    result = {"artifact": version.name, "world": world, "stone_support": [11, 91, 5],
              "placement_crosshair": placement_crosshair, "stacking_crosshair": stacking_crosshair,
              "eggs_placed": 4, "hatched_count": len(offspring), "hatchling_ages": ages,
              "nether_hatchling_in_origin_dimension": nether_hatchling,
              "nether_hatchling_absent_from_overworld": not wrong_dimension_hatchling,
              "headless": True, "random_ticks": 4096}
    print(json.dumps(result, indent=2), flush=True)
    if len(sys.argv) > 2:
        Path(sys.argv[2]).write_text(json.dumps(result, indent=2) + "\n")
finally:
    client.close()
    subprocess.run([LAUNCHER, "worlds", "delete", "--instance", "horizon", world],
                   capture_output=True, text=True, env=client.env)
