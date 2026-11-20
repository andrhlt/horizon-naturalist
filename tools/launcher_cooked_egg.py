"""Compare live egg cooking, consumption and the missing pre-port registration."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time

from cooked_egg_reference import capture_cooking


def capture(version, reference, baseline, recipes_only, consumption_only):
    with tempfile.TemporaryDirectory(prefix="nat-cooked-egg-", ignore_cleanup_errors=True) as folder:
        home = Path(folder)
        socket = home / "control.sock"
        script = home / "crafting.json"
        env = {**os.environ, "HORIZON_TICK_SCRIPT": str(script)}

        def ctl(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=75, env=env)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def step(ticks):
            command(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        def food_state():
            player = ctl("state")["player"]
            return player["food"], player["saturation"]

        def craft(product, egg, expected):
            size = 3 if product == "cake" else 2
            ingredients = (["milk_bucket", "milk_bucket", "milk_bucket", "sugar", egg,
                            "sugar", "wheat", "wheat", "wheat"] if size == 3 else
                           ["pumpkin", "sugar", egg])
            command("clear @s")
            for i, item in enumerate(ingredients):
                command(f"item replace entity @s hotbar.{i} with {item}")
            if size == 3:
                ctl("interact", 8, 102, 8, "north")
            else:
                ctl("open-inventory")
            clicks = [slot for i in range(len(ingredients)) for slot in (i, 100 + i)]
            clicks.extend((200, 0))
            steps = {tick: [f"click-slot {slot}"]
                     for tick, slot in zip(range(3, 3 + 3 * len(clicks), 3), clicks, strict=True)}
            close = max(steps) + 3
            steps[close] = ["close"]
            case = f"{product}/{egg}"
            script.write_text(json.dumps({"name": case, "ticks": close + 3, "steps": steps}))
            done = script.with_suffix(".done")
            done.unlink(missing_ok=True)
            script.with_suffix(".go").touch()
            deadline = time.monotonic() + 12
            while time.monotonic() < deadline and not done.exists():
                time.sleep(0.05)
            assert done.exists() and done.read_text() == case, case
            inventory = " ".join(command("data get entity @s Inventory"))
            item = f'"minecraft:{product}"'
            assert (f"id: {item}" in inventory) == bool(expected), (case, inventory)
            print(f"Live crafting {case}", bool(expected), flush=True)

        def consume():
            command("difficulty normal")
            command("gamerule minecraft:natural_regeneration false")
            command("gamemode survival @s")
            command("effect give @s minecraft:hunger 30 100 true")
            for _ in range(16):
                ctl("wait", 10)
                if food_state()[0] <= 12:
                    break
            command("effect clear @s minecraft:hunger")
            start_food, start_saturation = food_state()
            assert start_food <= 14, (start_food, start_saturation)
            command("item replace entity @s weapon.mainhand with naturalist:cooked_egg")
            ctl("look-at", 6.5, 108, 11.5)
            ctl("mouse", "right", "down")
            ctl("wait", 42)
            ctl("mouse", "right", "up")
            end_food, end_saturation = food_state()
            delta = [end_food - start_food, end_saturation - start_saturation]
            print("Consumed food and saturation", delta, flush=True)
            assert delta == reference["food"][:2], (delta, reference["food"])


        def craft_recipes():
            command("setblock 8 102 8 minecraft:crafting_table")
            command("tp @s 8.5 102 6.5")
            command("gamemode survival @s")
            for case, expected in reference["recipes"].items():
                product, ingredient = case.split("/")
                craft(product, ingredient, [] if baseline and ingredient.startswith("naturalist:") else expected)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist Cooked Egg", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            if consumption_only:
                command("tp @s 6.5 102 9.5")
                ctl("wait", 20)
                command("fill 2 101 2 15 101 15 minecraft:stone")
                command("fill 2 102 2 15 108 15 minecraft:air")
                command("tp @s 6.5 102 9.5")
                consume()
                return
            if recipes_only:
                command("tp @s 8.5 102 6.5")
                ctl("wait", 25)
                command("fill 2 101 2 15 101 15 minecraft:stone")
                command("fill 2 102 2 15 108 15 minecraft:air")
                craft_recipes()
                return
            if baseline:
                rows = command("give @s naturalist:cooked_egg")
                print(rows, flush=True)
                assert any("Unknown item" in row for row in rows), rows
                return
            command("tp @s 7.5 102 8.5")
            ctl("wait", 25)
            cooked = capture_cooking(command, step, reference["egg_tag"])
            print(json.dumps(cooked, indent=2), flush=True)
            assert cooked == reference["cooking"], (reference["cooking"], cooked)

            if "hide" in reference:
                for station, x in (("furnace", 7), ("smoker", 6)):
                    command(f'setblock {x} 102 11 minecraft:{station}{{Items:[{{Slot:0b,id:"naturalist:hide",count:1}},'
                            '{Slot:1b,id:"minecraft:coal",count:1}]}')
            command("setblock 8 102 9 minecraft:campfire")
            command("tp @s 6.5 102 9.5")
            command("item replace entity @s weapon.mainhand with naturalist:duck_egg")
            ctl("look-at", 8.5, 102.35, 9.5)
            hit = ctl("looking-at")["block"]
            assert hit["block"] == "minecraft:campfire", hit
            ctl("mouse", "right", "click")
            ctl("wait", 3)
            placed = command("data get block 8 102 9 Items[0].id")
            assert any("naturalist:duck_egg" in line for line in placed), placed
            step(210)
            rows = command('execute if entity @e[type=minecraft:item,nbt={Item:{id:"naturalist:cooked_egg"}}] run say campfire-cooked')
            assert any("campfire-cooked" in row for row in rows), rows
            print("Campfire cooked a real placed Duck Egg", flush=True)

            if "hide" in reference:
                actual = {}
                marker = " following block data: "
                for station, x in (("furnace", 7), ("smoker", 6)):
                    rows = command(f"data get block {x} 102 11 Items[{{Slot:2b}}].id")
                    actual[station] = next((row.split(marker, 1)[1].strip('"') for row in rows if marker in row), None)
                assert actual == reference["hide"], (actual, reference["hide"])
                print("Live Hide furnace and smoker", actual, flush=True)
            command("tick unfreeze")
            consume()

            if "recipes" in reference:
                craft_recipes()
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--baseline", action="store_true")
    parser.add_argument("--recipes-only", action="store_true")
    parser.add_argument("--consumption-only", action="store_true")
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()), args.baseline, args.recipes_only, args.consumption_only)
