"""Exercise all Java Forest Dasher brewing recipes on an actual brewing stand."""
import re
import time

BOTTLES = ("minecraft:potion", "minecraft:splash_potion", "minecraft:lingering_potion")
CASES = (
    ("minecraft:awkward", "naturalist:antler", "naturalist:forest_dasher"),
    ("naturalist:forest_dasher", "minecraft:redstone", "naturalist:long_forest_dasher"),
    ("naturalist:forest_dasher", "minecraft:glowstone_dust", "naturalist:strong_forest_dasher"),
)


def brew(command, x, y, z):
    at = f"{x} {y} {z}"
    output = {}
    for input_id, reagent, expected in CASES:
        command(f"setblock {at} minecraft:air")
        command(f"setblock {at} minecraft:brewing_stand")
        items = []
        for slot, bottle in enumerate(BOTTLES):
            items.append(
                f'{{Slot:{slot}b,id:"{bottle}",count:1,components:'
                f'{{"minecraft:potion_contents":{{potion:"{input_id}"}}}}}}'
            )
        items.append(f'{{Slot:3b,id:"{reagent}",count:1}}')
        items.append('{Slot:4b,id:"minecraft:blaze_powder",count:1}')
        command(f'data merge block {at} {{Items:[{",".join(items)}],Fuel:1}}')
        # Start a real brew. Then advance its active timer while preserving
        # the stand's loaded reagent so both engines perform the recipe.
        command("tick step 3")
        time.sleep(0.25)
        command(f"data merge block {at} {{BrewTime:3}}")
        command("tick step 12")
        time.sleep(0.8)
        rows = command(f"data get block {at} Items")
        line = next((row for row in rows if "following block data" in row), "")
        matches = re.findall(r'potion\s*:\s*"(naturalist:[a-z_]+)"', line)
        assert matches == [expected] * len(BOTTLES), (expected, rows)
        assert all(f'"{bottle}"' in line for bottle in BOTTLES), (expected, rows)
        output[reagent] = dict(zip(BOTTLES, matches))
    return output
