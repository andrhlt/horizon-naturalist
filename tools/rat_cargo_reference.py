"""Save, reopen, and kill isolated Rats with known cargo on pinned Java."""
import argparse
from pathlib import Path
import re



RAT = "@e[type=naturalist:rat,tag=rat_cargo,limit=1]"
WORLD = "rat-cargo-parity-fixture"


def data(command, target, field):
    prefix = " following entity data: "
    return next((row.split(prefix, 1)[1] for row in command(f"data get entity {target} {field}")
                 if prefix in row), None)


def count_item_list(tag):
    stacks = re.findall(r'\{([^{}]+)\}', tag or "")
    items = {}
    for stack in stacks:
        name = re.search(r'id: "([a-z:_]+)"', stack)
        count = re.search(r'count: (\d+)', stack)
        if name and count:
            items[name[1]] = items.get(name[1], 0) + int(count[1])
    return items


def snapshot(command):
    station = data(command, RAT, "Workstation")
    return {
        "station": int(station.rstrip("Ll")) if station is not None else None,
        "cargo": count_item_list(data(command, RAT, "CarriedItems")),
        "from_hand": data(command, RAT, "FromHand") == "1b",
        "following": data(command, RAT, "FollowingOwner") == "1b",
        "variant": (data(command, RAT, "Variant") or "").strip('"'),
    }


def prepare(command):
    command("tick freeze")
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:random_tick_speed 0")
    command("gamerule minecraft:mob_drops false")
    command("fill 2 91 2 15 91 14 minecraft:stone")
    command("fill 2 92 2 15 98 14 minecraft:air")
    command("setblock 9 92 5 minecraft:chest")
    command(f"kill {RAT}")
    command("kill @e[type=minecraft:item,x=10,y=92,z=5,distance=..3]")
    station = (9 << 38) | (5 << 12) | 92
    command('summon naturalist:rat 10.5 92 5.5 '
            f'{{NoAI:1b,FromHand:1b,FollowingOwner:0b,Workstation:{station}L,'
            'Variant:"naturalist:brown",Tags:[rat_cargo],'
            'CarriedItems:[{id:"minecraft:diamond",count:2},{id:"minecraft:wheat",count:3}]}')
    result = snapshot(command)
    assert result == {"station": station, "cargo": {"minecraft:diamond": 2, "minecraft:wheat": 3},
                      "from_hand": True, "following": False, "variant": "naturalist:brown"}, result
    command("save-all flush")
    return result


def reopen(command):
    command("tick freeze")
    result = {"loaded": snapshot(command)}
    assert result["loaded"]["cargo"], result
    command(f"kill {RAT}")
    result["spilled_with_mob_drops_disabled"] = {
        item: count_item_list("[" + (data(command,
            '@e[type=minecraft:item,x=10,y=92,z=5,distance=..3,limit=1,'
            f'nbt={{Item:{{id:"{item}"}}}}]', 'Item') or "") + "]").get(item, 0)
        for item in result["loaded"]["cargo"]
    }
    assert result["spilled_with_mob_drops_disabled"] == result["loaded"]["cargo"], result
    return result


def capture(command, _):
    command("forceload add 0 0 15 15")
    return phase(command)


if __name__ == "__main__":
    import java_reference

    parser = argparse.ArgumentParser(description=__doc__)
    for field in ("server", "java_home", "output"):
        parser.add_argument(field, type=Path)
    parser.add_argument("phase", choices=("prepare", "reopen"))
    args = parser.parse_args()
    phase = {"prepare": prepare, "reopen": reopen}[args.phase]
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", WORLD])
