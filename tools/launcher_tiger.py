"""Compare Tiger characteristics, ownership and nighttime hunting against pinned Java."""
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile

native = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-tiger-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run([native / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    def step(n):
        cmd(f"tick step {n}")
        ctl("wait", n + 2)

    def data(target, key, optional=False):
        rows = cmd(f"data get entity {target} {key}")
        marker = " following entity data: "
        for row in rows:
            if marker in row:
                return row.split(marker, 1)[1]
        if not optional:
            raise AssertionError((target, key, rows))
        return None

    def stat(target, key):
        for row in cmd(f"attribute {target} minecraft:{key} base get"):
            value = re.search(r" is ([0-9.eE+-]+)", row)
            if value:
                return float(value[1])
        raise AssertionError((target, key))

    def inspect(query):
        marker = "Tiger fixture "
        rows = cmd(f"naturalist_fixture_tiger {query}")
        for row in rows:
            if marker in row:
                return json.loads(row.split(marker, 1)[1])
        raise AssertionError((query, rows))

    def spawn(kind, x, z, tag, nbt):
        rows = cmd(f"summon {kind} {x} 92 {z} {{Tags:[{tag}]{',' if nbt else ''}{nbt}}}")
        assert any("Summoned" in row for row in rows), rows

    def remove_tigers():
        cmd("kill @e[type=naturalist:tiger]")
        step(25)

    def held(item, n=1):
        cmd(f"item replace entity @s weapon.mainhand with {item} {n}")

    def click(x):
        mobs = ctl("entities", 6, "naturalist:tiger")["entities"]
        mob = min(mobs, key=lambda entity: abs(entity["pos"][0] - x))
        aimed = ctl("look-at-entity", mob["id"])["target"]
        assert aimed.get("kind") == "entity", aimed
        ctl("mouse", "right", "click")
        ctl("wait", 3)

    try:
        ctl("launch", "--headless", "--game", native / "horizon-client", "--resources",
            native / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist Tiger", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("gamerule minecraft:advance_time false")
        cmd("tp @s 7.5 92 3.5")
        ctl("wait", 24)
        cmd("tick freeze")
        cmd("fill 2 91 2 30 91 12 minecraft:stone")
        cmd("fill 2 92 2 30 96 12 minecraft:air")
        cmd("tp @s 7.5 92 3.5")
        step(4)
        adult, cub = "@e[tag=adult,limit=1]", "@e[tag=cub,limit=1]"
        spawn("naturalist:tiger", 6.5, 5.5, "adult", "NoAI:1b")
        spawn("naturalist:tiger", 8.5, 5.5, "cub", "NoAI:1b,Age:-24000")
        a, b = inspect("adult"), inspect("baby")
        compared = {
            "adult_dims": a["dims"], "baby_dims": b["dims"], "ambient_interval": a["ambient_interval"],
            "food": a["food"], "food_ids": sorted(a["food_ids"]), "default_variant": data(adult, "Variant").strip('"'),
            "attributes": [stat(adult, identifier) for identifier in
                ("max_health","movement_speed","attack_damage","follow_range","step_height","knockback_resistance")],
        }
        compared["sleep_clock"] = []
        compared["clock_readbacks"] = []
        for time in reference["clock_readbacks"]:
            cmd(f"time set {time}")
            frame = inspect("adult")
            compared["clock_readbacks"].append(frame["clock"])
            compared["sleep_clock"].append(frame["can_sleep"])
        cmd("time set 8000")
        cmd("setblock 6 92 5 minecraft:water")
        compared["sleep_in_water"] = inspect("adult")["can_sleep"]
        cmd("setblock 6 92 5 minecraft:air")
        owner = data("@s", "UUID")
        remove_tigers()
        cmd("gamemode survival @s")
        cmd("tp @s 7.5 92 3.5")
        spawn("naturalist:tiger", 6.5, 5.5, "adult", f"NoAI:1b,Owner:{owner},Health:12.0f")
        spawn("naturalist:tiger", 8.5, 5.5, "cub", f"NoAI:1b,Owner:{owner},Age:-24000")
        compared["sleep_tamed"] = inspect("baby")["can_sleep"]
        cmd("tick unfreeze")
        held("minecraft:cooked_mutton", 2)
        click(6.5)
        inv = " ".join(cmd("data get entity @s Inventory"))
        compared["owner_heal"] = [float(data(adult, "Health").rstrip("fF")), 1 if "count: 1" in inv else -1]
        compared["owner_heal_consumed"] = compared["owner_heal"][1] == 1
        held("minecraft:beef", 2)
        age_before = int(data(cub, "Age"))
        click(8.5)
        age_after = int(data(cub, "Age"))
        inv = " ".join(cmd("data get entity @s Inventory"))
        compared["tamed_cub_grows"] = 2370 <= age_after - age_before <= 2410 and "count: 1" in inv
        cmd("clear @s")
        click(6.5)
        compared["owner_sits"] = data(adult, "Sitting") == "1b"
        held("naturalist:whistle")
        click(6.5)
        compared["owner_whistle_stops_follow"] = data(adult, "FollowingOwner") != "1b"
        print("Attributes and pets", json.dumps(compared), flush=True)
        cmd("tick freeze")
        remove_tigers()

        cmd("gamemode spectator @s")
        cmd("tp @s 7.5 92 3.5")
        cmd("time set 18000")
        spawn("naturalist:tiger", 6.5, 7.5, "adult", "PersistenceRequired:1b")
        spawn("minecraft:pig", 22.5, 7.5, "prey", "NoAI:1b")
        ever_stalk = False
        target = None
        cooldown = 0
        for _ in range(90):
            step(4)
            frame = inspect("adult")
            ever_stalk |= frame["stalking"]
            target = target or frame["target"]
            cooldown = frame["cooldown"]
            if cooldown > 0:
                break
        hunting = {"found": target, "stalked": ever_stalk, "cooldown": cooldown}
        print("Real night hunt", hunting, flush=True)
        assert target == "minecraft:pig" and ever_stalk and 2396 <= cooldown <= 2400, hunting
        differences = {key: [reference[key], actual] for key, actual in compared.items() if actual != reference[key]}
        assert not differences, differences
        print(f"Tiger matched {len(compared)} comparisons and its live night stalking and kill.")
    finally:
        if socket.exists():
            ctl("quit")
