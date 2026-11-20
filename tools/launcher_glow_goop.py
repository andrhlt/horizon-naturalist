"""Exercise the installed Glow Goop through headless player actions and loaded recipes."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile

GOOP = "naturalist:glow_goop"
PLACE_SOUND = "minecraft:block.honey_block.place"
BREAK_SOUND = "minecraft:block.honey_block.break"
TARGET = (8, 100, 8)


def capture(version, java, expect_offhand):
    with tempfile.TemporaryDirectory(prefix="nat-goop-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "goop.sock"

        def ctl(*args):
            response = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                capture_output=True, text=True, timeout=75,
            )
            assert response.returncode == 0, (args, response.stdout, response.stderr)
            return json.loads(response.stdout.splitlines()[-1])

        def cmd(command):
            return ctl("command", command)["chat"]

        def check(pos, block):
            coords = " ".join(map(str, pos))
            return any("Test passed" in row for row in cmd(f"execute if block {coords} {block}"))

        def state(pos=TARGET):
            for count in (1, 2, 3):
                for wet in ("true", "false"):
                    if check(pos, f"{GOOP}[goop={count},waterlogged={wet}]"):
                        return f"{count}:{wet}:{count * 5}"
            return ctl("block", *pos)["block"]

        def held(item, count=1, slot="weapon.mainhand"):
            cmd(f"item replace entity @s {slot} with {item} {count}")
            ctl("wait", 4)

        def drop_counts(expected=True):
            items = []
            rows = []
            for _ in range(8 if expected else 1):
                rows = cmd("execute as @e[type=minecraft:item,distance=..8] run data get entity @s Item")
                for row in rows:
                    item = re.search(r'id: "([^"]+)"', row)
                    count = re.search(r'count: (\d+)', row)
                    if item and count:
                        items.append((item[1], count[1]))
                if items:
                    break
                ctl("wait", 2)
            items.sort()
            if expected and not items:
                status = ctl("status")
                print(json.dumps({"missing_drop": rows, "held": status["held"],
                                  "pickups": status["pickups"], "player": status["player"]}), flush=True)
            cmd("kill @e[type=minecraft:item,distance=..8]")
            return [f"{item}*{count}" for item, count in items]

        def inventory_goop_count():
            rows = cmd("data get entity @s Inventory")
            pattern = r'\{[^{}]*count: (\d+)[^{}]*id: "naturalist:glow_goop"[^{}]*\}'
            return sum(int(count) for row in rows for count in re.findall(pattern, row))

        def aim_down(y=100.98):
            ctl("look-at", 8.5, y, 8.5)

        def click():
            ctl("mouse", "right", "click")
            ctl("wait", 5)

        def mine(before):
            assert (ctl("looking-at").get("block") or {}).get("pos") == list(TARGET), ctl("looking-at")
            ctl("mouse", "left", "down")
            try:
                for _ in range(15):
                    ctl("wait", 2)
                    if not check(TARGET, before):
                        return
                raise AssertionError(("goop layer failed to mine", ctl("block", *TARGET)))
            finally:
                ctl("mouse", "left", "up")
                ctl("wait", 5)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", root,
                "--game-dir", root, "--world", "Glow Goop Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:block_drops true")
            cmd("tp @s 8.5 100 6.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 8.5 100 6.5")
            cmd("gamemode survival @s")
            cmd("clear @s")
            held(GOOP, 5)
            result = {"stacking": []}
            light = []
            place_before = ctl("status")["heard"].get(PLACE_SOUND, 0)
            for click_index in range(4):
                aim_down(100.0 if click_index == 0 else 100.98)
                click()
                result["stacking"].append("true/" + state() + "/" + state((8, 101, 8)))
                light.append(int(ctl("light", *TARGET, *TARGET)["block"], 16))
            remaining = cmd("data get entity @s SelectedItem")
            result["stack_remaining"] = f"{GOOP}*1" if any(GOOP in row and "count: 1" in row for row in remaining) else remaining
            assert result["stacking"] == java["stacking"], result
            assert light == [5, 10, 15, 15], light
            placed = ctl("status")["heard"].get(PLACE_SOUND, 0) - place_before
            assert placed >= 4, (placed, ctl("status")["heard"])
            cmd("setblock 8 101 8 minecraft:air")
            ctl("wait", 4)
            result["main_outline"] = (ctl("looking-at").get("block") or {}).get("pos") == list(TARGET)
            cmd("clear @s")
            ctl("wait", 4)
            result["empty_outline"] = (ctl("looking-at").get("block") or {}).get("pos") == list(TARGET)
            held(GOOP, 1, "weapon.offhand")
            result["off_outline"] = (ctl("looking-at").get("block") or {}).get("pos") == list(TARGET)
            cmd("clear @s")
            held(GOOP)
            cmd("tp @s 8.5 100 5.8")
            ctl("wait", 4)
            aim_down()
            result["mine"] = []
            break_before = ctl("status")["heard"].get(BREAK_SOUND, 0)
            for count in (3, 2, 1):
                mine(f"{GOOP}[goop={count},waterlogged=false]")
                drops = drop_counts()
                result["mine"].append("true/" + state() + "/[" + ", ".join(drops) + "]")
            broken = ctl("status")["heard"].get(BREAK_SOUND, 0) - break_before
            assert broken >= 3, (broken, ctl("status")["heard"])

            cmd(f"setblock 8 100 8 {GOOP}[goop=3]")
            held("minecraft:stone")
            before_goop = inventory_goop_count()
            aim_down(100.0)
            click()
            cmd("tick freeze")
            replacement_drops = drop_counts()
            picked_up = inventory_goop_count() - before_goop
            cmd("tick unfreeze")
            assert picked_up >= 0, picked_up
            if picked_up:
                count = picked_up + sum(int(item.rsplit("*", 1)[1]) for item in replacement_drops
                                        if item.startswith(f"{GOOP}*"))
                replacement_drops = [item for item in replacement_drops if not item.startswith(f"{GOOP}*")]
                replacement_drops.append(f"{GOOP}*{count}")
                replacement_drops.sort()
            result["replace"] = [check(TARGET, "minecraft:stone"), state(), replacement_drops]
            cmd(f"setblock 8 100 8 {GOOP}[goop=3]")
            held("minecraft:cactus")
            aim_down(100.0)
            click()
            still_cactus = ctl("status")["held"]
            item = "minecraft:cactus*1" if still_cactus and still_cactus.get("item") == "minecraft:cactus" else "empty"
            result["failed_replace"] = [check(TARGET, "minecraft:cactus"), state(), drop_counts(), item]

            cmd("setblock 8 100 8 minecraft:water")
            held(GOOP, 3)
            cmd("tp @s 8.5 100 6.5")
            aim_down(100.0)
            click()
            water = [True, state()]
            first_placement = {"state": water[-1], "held": ctl("status")["held"], "looking_at": ctl("looking-at")}
            cmd("tp @s 8.5 100 6.5")
            aim_down()
            click()
            water.extend([True, state()])
            print(json.dumps({"wet_first_placement": first_placement, "wet_second_placement": water[-1],
                "held": ctl("status")["held"], "hit": ctl("looking-at")}), flush=True)
            for count in (2, 1):
                cmd("tp @s 8.5 100 6.5")
                aim_down()
                mine(f"{GOOP}[goop={count},waterlogged=true]")
                water.extend([True, state(), drop_counts()])
            result["water"] = water
            cmd("fill 5 100 6 11 102 10 minecraft:air")
            cmd("tp @s 8.5 100 6.5")

            cmd(f"setblock 8 100 8 {GOOP}[goop=3]")
            cmd("gamemode creative @s")
            aim_down()
            mine(f"{GOOP}[goop=3]")
            result["creative"] = [True, state(), drop_counts(expected=False)]
            cmd("gamemode survival @s")
            cmd(f"setblock 8 100 8 {GOOP}")
            held(GOOP)
            ctl("key", "shift", "down")
            try:
                ctl("wait", 3)
                aim_down()
                click()
            finally:
                ctl("key", "shift", "up")
            result["sneak_placement"] = [True, state(), state((8, 101, 8))]
            cmd("setblock 8 101 8 minecraft:air")
            cmd("tp @s 8.5 100 6.8 0 0")
            ctl("hold", "w", 1.15)
            result["collision"] = ctl("status")["player"]["position"][2] < 8.5
            cmd("tp @s 8.5 100 6.5")
            cmd(f"setblock 8 100 8 {GOOP}")
            cmd("clear @s")
            held(GOOP, 1, "weapon.offhand")
            aim_down(100.0)
            click()
            offhand = ctl("status")["offhand"]
            measured = [not offhand, state(), "empty" if not offhand else GOOP + "*1"]
            print(json.dumps({"offhand_stack_on_floor": measured}), flush=True)
            if expect_offhand:
                result["offhand_stack_on_floor"] = measured
            cmd("clear @s")
            cmd(f"fill 5 100 9 11 100 9 {GOOP}[goop=3]")
            ctl("wait", 6)
            markers_before = ctl("status")["particles"].get("block_marker", 0)
            ctl("wait", 25)
            assert ctl("status")["particles"].get("block_marker", 0) == markers_before
            held(GOOP, 1, "weapon.offhand")
            ctl("wait", 25)
            assert ctl("status")["particles"].get("block_marker", 0) == markers_before
            held(GOOP)
            for _ in range(12):
                ctl("wait", 6)
                if ctl("status")["particles"].get("block_marker", 0) > markers_before:
                    break
            markers = ctl("status")["particles"].get("block_marker", 0) - markers_before
            assert markers > 0, ctl("status")["particles"]
            print(json.dumps({"block_light": light, "honey_placement_sounds": placed,
                              "honey_break_sounds": broken, "survival_markers": markers}), flush=True)
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("java", type=Path)
    parser.add_argument("--expect-offhand", action="store_true")
    args = parser.parse_args()
    expected = json.loads(args.java.read_text())
    actual = capture(args.version, expected, args.expect_offhand)
    print(json.dumps(actual, indent=2), flush=True)
    assert all(actual[key] == expected[key] for key in actual), {
        key: [actual[key], expected[key]] for key in actual if actual[key] != expected[key]
    }
