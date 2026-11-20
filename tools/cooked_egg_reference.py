"""Capture loaded Java egg ingredients, food values and actual cooking results."""
import argparse
import json
from pathlib import Path
import time


def capture_cooking(command, sprint, ingredients):
    command("gamerule minecraft:spawn_mobs false")
    command("tick freeze")
    command("fill 2 101 2 15 101 15 minecraft:stone")
    command("fill 2 102 2 15 108 15 minecraft:air")
    pairs = {}
    for row, station in enumerate(("furnace", "smoker")):
        for column, egg in enumerate([*ingredients, "minecraft:snowball"]):
            pos = f"{3 + column} 102 {4 + row}"
            command(f'setblock {pos} minecraft:{station}{{Items:[{{Slot:0b,id:"{egg}",count:1}},{{Slot:1b,id:"minecraft:coal",count:1}}]}}')
            pairs[(station, egg)] = pos
    sprint(225)
    results = {}
    marker = " following block data: "
    for (station, egg), pos in pairs.items():
        output = command(f"data get block {pos} Items[{{Slot:2b}}].id")
        results[f"{station}/{egg}"] = next((line.split(marker, 1)[1].strip('"') for line in output if marker in line), None)
    return results


def capture(command, _):
    command("forceload add 0 0 15 15")
    marker = "cooked_egg_probe "
    result = next(json.loads(line.split(marker, 1)[1]) for line in command("cooked_egg_probe") if marker + "{" in line)

    def sprint(ticks):
        command(f"tick sprint {ticks}")
        time.sleep(1)

    result["cooking"] = capture_cooking(command, sprint, result["egg_tag"])
    command('setblock 7 102 9 minecraft:furnace{Items:[{Slot:0b,id:"naturalist:hide",count:1},{Slot:1b,id:"minecraft:coal",count:1}]}')
    command('setblock 6 102 9 minecraft:smoker{Items:[{Slot:0b,id:"naturalist:hide",count:1},{Slot:1b,id:"minecraft:coal",count:1}]}')
    command("setblock 8 102 9 minecraft:air")
    command('setblock 8 102 9 minecraft:campfire{Items:[{Slot:0b,id:"naturalist:duck_egg",count:1}],'
            'CookingTimes:[I;0,0,0,0],CookingTotalTimes:[I;200,0,0,0]}')
    sprint(210)
    rows = command('execute if entity @e[type=minecraft:item,x=8,y=102,z=9,distance=..5,'
                   'nbt={Item:{id:"naturalist:cooked_egg",count:1}}] run say campfire-cooked')
    result["campfire"] = any("campfire-cooked" in row for row in rows)
    assert result["campfire"], rows
    result["hide"] = {}
    for station, x in (("furnace", 7), ("smoker", 6)):
        rows = command(f"data get block {x} 102 9 Items[{{Slot:2b}}].id")
        marker = " following block data: "
        result["hide"][station] = next((row.split(marker, 1)[1].strip('"') for row in rows if marker in row), None)
    assert result["hide"] == {"furnace": "minecraft:leather", "smoker": None}, result["hide"]
    return result


if __name__ == "__main__":
    import java_reference

    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "CookedEggProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "cooked-egg-aux-parity-fixture"])
