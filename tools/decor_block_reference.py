"""Measure decorative block recognition and neighbor rules in pinned Java."""
import argparse
from pathlib import Path
import time


COLORS = ("azure", "verdant", "crimson")
BLOCKS = (
    *(f"{color}_froglass{shape}" for color in COLORS for shape in ("", "_pane")),
    *("shellstone_bricks" if base == "shellstone_brick" and not shape else base + shape
      for base in ("shellstone", "shellstone_brick", "cut_shellstone", "smooth_shellstone")
      for shape in ("", "_slab", "_stairs", "_wall")),
)


def capture_blocks(command, wait):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 15 15")
    wait(35)
    command("fill 1 101 1 14 101 14 minecraft:stone")
    command("fill 1 102 1 14 106 14 minecraft:air")

    def has(pos, block):
        result = command(f"execute if block {pos} {block} run say decor-block-present")
        return any("decor-block-present" in row for row in result)

    placed = {}
    for index, block in enumerate(BLOCKS):
        position = f"{2+index%11} 102 {2+index//11}"
        command(f"setblock {position} naturalist:{block}")
        placed[block] = has(position, f"naturalist:{block}")
    assert all(placed.values()), placed

    control = {}
    for name, connect_field, bare, connected in (
        ("azure_froglass_pane", "east", "false", "true"),
        ("smooth_shellstone_wall", "east", "none", "low"),
    ):
        block = f"naturalist:{name}"
        command(f"setblock 8 102 8 {block}")
        before = has("8 102 8", f"{block}[{connect_field}={bare}]")
        command("setblock 9 102 8 minecraft:stone")
        wait(2)
        joined = has("8 102 8", f"{block}[{connect_field}={connected}]")
        command("setblock 9 102 8 minecraft:air")
        wait(2)
        removed = has("8 102 8", f"{block}[{connect_field}={bare}]")
        control[name] = [before, joined, removed]
    command("setblock 8 102 8 naturalist:shellstone_slab[type=double,waterlogged=false]")
    control["shellstone_slab"] = has("8 102 8", "naturalist:shellstone_slab[type=double,waterlogged=false]")
    command("setblock 8 102 8 naturalist:cut_shellstone_stairs[facing=east,half=top,shape=straight,waterlogged=false]")
    control["cut_shellstone_stairs"] = has("8 102 8", "naturalist:cut_shellstone_stairs[facing=east,half=top,shape=straight,waterlogged=false]")
    loot = {}
    for case, block, tool in (
        ("no_silk", "azure_froglass", "minecraft:diamond_pickaxe"),
        ("silk", "azure_froglass", 'minecraft:diamond_pickaxe[minecraft:enchantments={"minecraft:silk_touch":1}]'),
        ("pane_silk", "azure_froglass_pane", 'minecraft:diamond_pickaxe[minecraft:enchantments={"minecraft:silk_touch":1}]'),
        ("double_slab", "shellstone_slab[type=double]", "minecraft:diamond_pickaxe"),
    ):
        command("kill @e[type=minecraft:item,x=4,y=102,z=9,distance=..3]")
        command(f"setblock 8 102 8 naturalist:{block}")
        command(f"loot spawn 4.5 102 9.5 mine 8 102 8 {tool}")
        key = block.split("[", 1)[0]
        marker = " following entity data: "
        rows = command('data get entity @e[type=minecraft:item,x=4,y=102,z=9,distance=..3,limit=1,'
                       f'nbt={{Item:{{id:"naturalist:{key}"}}}}] Item.count')
        loot[case] = next((int(row.split(marker, 1)[1]) for row in rows if marker in row), 0)
    return {"registered_placed": placed, "physical_controls": control, "block_loot": loot}


if __name__ == "__main__":
    import java_reference

    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = lambda command, _: capture_blocks(command, lambda ticks: time.sleep(ticks / 20))
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "decor-block-parity-fixture"])
