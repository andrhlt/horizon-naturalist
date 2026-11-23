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
    result = {"artifact": version.name, "world": world, "stone_support": [11, 91, 5],
              "placement_crosshair": placement_crosshair, "stacking_crosshair": stacking_crosshair,
              "eggs_placed": 4, "hatched_count": len(offspring), "hatchling_ages": ages,
              "headless": True, "random_ticks": 4096}
    print(json.dumps(result, indent=2), flush=True)
    if len(sys.argv) > 2:
        Path(sys.argv[2]).write_text(json.dumps(result, indent=2) + "\n")
finally:
    client.close()
    subprocess.run([LAUNCHER, "worlds", "delete", "--instance", "horizon", world],
                   capture_output=True, text=True, env=client.env)
