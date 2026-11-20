"""Run stock headless Snake cases against the pinned NeoForge observations."""
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile

native = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())

with tempfile.TemporaryDirectory(prefix="nat-snake-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run(
            [native / "horizon-ctl", "--socket", socket, *map(str, args)],
            check=True, capture_output=True, text=True, timeout=75,
        )
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    def data(target, key):
        marker = " following entity data: "
        for row in cmd(f"data get entity {target} {key}"):
            if marker in row:
                return row.split(marker, 1)[1]
        return None

    def stat(target, key):
        for row in cmd(f"attribute {target} minecraft:{key} base get"):
            value = re.search(r" is ([0-9.eE+-]+)", row)
            if value:
                return float(value[1])
        raise AssertionError((target, key))

    def inspect(query):
        marker = "Snake fixture "
        rows = cmd(f"naturalist_fixture_snake {query}")
        for row in rows:
            if marker in row:
                return json.loads(row.split(marker, 1)[1])
        raise AssertionError((query, rows))

    def spawn(kind, x, tag, nbt=""):
        fields = f"Tags:[{tag}]" + ("," + nbt if nbt else "")
        rows = cmd(f"summon {kind} {x} 92 5.5 {{{fields}}}")
        assert any("Summoned" in row for row in rows), rows

    def remove(tag):
        cmd(f"kill @e[tag={tag}]")
        step(25)

    def held(item, count=1):
        cmd(f"item replace entity @s weapon.mainhand with {item} {count}")

    def click(x):
        mobs = ctl("entities", 6, "naturalist:snake")["entities"]
        mob = min(mobs, key=lambda entity: abs(entity["pos"][0] - x))
        aimed = ctl("look-at-entity", mob["id"])["target"]
        assert aimed.get("kind") == "entity", aimed
        ctl("mouse", "right", "click")
        ctl("wait", 3)

    def item_count():
        inv = data("@s", "Inventory")
        return int(re.search(r"count: (\d+)", inv)[1])

    try:
        ctl("launch", "--headless", "--game", native / "horizon-client", "--resources",
            native / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist Snake", "--creative", "--test-terrain",
            "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("gamerule minecraft:advance_time false")
        cmd("tp @s 7.5 92 3.5")
        ctl("wait", 24)
        cmd("tick freeze")
        cmd("fill 2 91 2 30 91 12 minecraft:stone")
        cmd("fill 2 92 2 30 95 12 minecraft:air")
        cmd("tp @s 7.5 92 3.5")
        step(4)

        kind = "naturalist:snake"
        adult = "@e[tag=adult,limit=1]"
        spawn(kind, 6.5, "adult", "NoAI:1b")
        spawn(kind, 8.5, "baby", "Age:-24000,NoAI:1b")
        a, b = inspect("adult"), inspect("baby")
        output = {
            "adult_dims": a["dims"], "baby_dims": b["dims"], "cluster": a["cluster"],
            "food_ids": sorted(a["food_ids"]), "tame_ids": sorted(a["tame_ids"]),
            "voice_volume": a["voice_volume"], "default_variant": data(adult, "Variant").strip('"'),
            "attributes": [stat(adult, key) for key in
                           ("max_health", "movement_speed", "attack_damage", "follow_range", "step_height")],
            "spawn_gear": inspect("gear"),
        }
        assert a["jump_power"] == 0.0, a
        meal = inspect("eat")
        assert meal.pop("eat_event"), meal
        output.update(meal)
        output["sleep_clock"], output["clock_readbacks"] = [], []
        for clock in reference["clock_readbacks"]:
            cmd(f"time set {clock}")
            snapshot = inspect("adult")
            output["clock_readbacks"].append(snapshot["clock"])
            output["sleep_clock"].append(snapshot["can_sleep"])
        cmd("time set 13000")
        cmd("setblock 6 92 5 minecraft:water")
        output["sleep_in_water"] = inspect("adult")["can_sleep"]
        cmd("setblock 6 92 5 minecraft:air")
        cmd(f"data merge entity {adult} {{anger_end_time:599999L}}")
        print("Anger after merge", data(adult, "anger_end_time"), inspect("adult"), flush=True)
        step(2)
        print("Anger after two ticks", data(adult, "anger_end_time"), inspect("adult"), flush=True)
        output["sleep_while_angry"] = inspect("adult")["can_sleep"]
        cmd(f"data merge entity {adult} {{anger_end_time:-1L}}")

        output["poison_duration"], output["post_bite_health"], output["rattlesnake"] = [], [], []
        for variant in ("snake", "coral_snake", "rattlesnake"):
            cmd(f'data merge entity {adult} {{Variant:"naturalist:{variant}"}}')
            spawn("minecraft:cow", 7.5, "cow", "NoAI:1b")
            snapshot = inspect("bite")
            output["poison_duration"].append(snapshot["poison"])
            output["post_bite_health"].append(snapshot["health"])
            output["rattlesnake"].append(inspect("adult")["rattlesnake"])
            remove("cow")
        remove("adult")
        remove("baby")

        # Use actual survival right-click interactions for taming and diet.
        owner = data("@s", "UUID")
        cmd("gamemode survival @s")
        output["tame_rows"] = []
        for seed, *_ in reference["tame_rows"]:
            spawn(kind, 6.5, "adult", "NoAI:1b")
            held("minecraft:spider_eye", 2)
            inspect(f"seed {seed}")
            click(6.5)
            snapshot = inspect("adult")
            count = item_count()
            output["tame_rows"].append([seed, count == 1, count, snapshot["tamed"], snapshot["sitting"]])
            cmd("kill " + adult)
            step(25)
        spawn(kind, 6.5, "adult", "NoAI:1b")
        held("minecraft:egg", 2)
        click(6.5)
        snapshot = inspect("adult")
        output["ordinary_egg_feeds_without_taming"] = [item_count(), snapshot["tamed"], snapshot["in_love"]]
        cmd("kill " + adult)
        step(25)
        spawn(kind, 6.5, "adult", f"NoAI:1b,Owner:{owner},Health:5.0f")
        held("minecraft:spider_eye", 2)
        click(6.5)
        count = item_count()
        output["owner_heal"] = [float(data(adult, "Health").rstrip("fF")), count]
        output["owner_heal_consumed"] = count == 1
        cmd("clear @s")
        click(6.5)
        output["owner_sits"] = data(adult, "Sitting") == "1b"
        held("naturalist:whistle")
        click(6.5)
        output["owner_whistle_stops_follow"] = data(adult, "FollowingOwner") == "0b"
        print(json.dumps(output, indent=2), flush=True)
        mismatch = {key: [reference[key], value] for key, value in output.items()
                    if value != reference[key] and key not in {"voice_volume", "attributes"}}
        if abs(output["voice_volume"] - reference["voice_volume"]) > 1e-7:
            mismatch["voice_volume"] = [reference["voice_volume"], output["voice_volume"]]
        if not all(abs(x - y) < 1e-7 for x, y in zip(output["attributes"], reference["attributes"])):
            mismatch["attributes"] = [reference["attributes"], output["attributes"]]
        assert not mismatch, mismatch
        print(f"Snake matched all {len(output)} headless Java groups in the core comparison.")
    finally:
        if socket.exists():
            ctl("quit")
