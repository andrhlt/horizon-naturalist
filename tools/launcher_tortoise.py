"""Compare Horizon Tortoise through production gameplay against the pinned Java run."""
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
from launcher_managed import start as start_managed, stop as stop_managed

version = Path(sys.argv[1])
java = json.loads(Path(sys.argv[2]).read_text())
spawn_java = json.loads(Path(sys.argv[3]).read_text())

with tempfile.TemporaryDirectory(prefix="nat-tortoise-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def data(target, name, optional=False):
        rows = cmd(f"data get entity {target} {name}")
        marker = "following entity data: "
        for row in rows:
            if marker in row:
                return row.split(marker, 1)[1]
        if not optional:
            raise AssertionError((target, name, rows))
        return None

    def stat(target, identifier):
        rows = cmd(f"attribute {target} minecraft:{identifier} base get")
        for row in rows:
            match = re.search(r" is ([0-9.eE+-]+)", row)
            if match:
                return float(match[1])
        raise AssertionError((target, identifier, rows))

    def health(target):
        return float(data(target, "Health").rstrip("fF"))

    def stats(target):
        return [stat(target, "max_health"), stat(target, "attack_damage"), health(target)]

    def step(count):
        cmd(f"tick step {count}")
        ctl("wait", count + 2)

    def inspect(query):
        marker = "Tortoise fixture "
        rows = cmd(f"naturalist_fixture_tortoise {query}")
        for row in rows:
            if marker in row:
                return json.loads(row.split(marker, 1)[1])
        raise AssertionError((query, rows))

    def target(tag):
        return f"@e[type=naturalist:tortoise,tag={tag},limit=1]"

    def spawn(x, z, tag, fields=""):
        suffix = f",{fields}" if fields else ""
        rows = cmd(f"summon naturalist:tortoise {x} 92 {z} {{Tags:[{tag}]{suffix}}}")
        assert any("Summoned" in row for row in rows), rows

    def remove_tortoises():
        cmd("kill @e[type=naturalist:tortoise]")
        step(24)

    def block(x, y, z, state):
        return any("test passed" in row.lower() or "succeeded" in row.lower()
                   for row in cmd(f"execute if block {x} {y} {z} {state}"))

    def click(x, count=1):
        for _ in range(count):
            nearby = ctl("entities", 7, "naturalist:tortoise")["entities"]
            picked = min(nearby, key=lambda mob: abs(mob["pos"][0] - x))
            aim = ctl("look-at-entity", picked["id"])["target"]
            assert aim.get("kind") == "entity", (picked, aim)
            ctl("mouse", "right", "click")
            ctl("wait", 3)

    def replace(item, count=1):
        cmd(f"item replace entity @s weapon.mainhand with {item} {count}")

    managed = None
    try:
        managed = start_managed(version, home, socket, "Naturalist Tortoise",
                                ("--test-terrain", "--render-distance", "4"))
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("gamerule minecraft:mob_griefing true")
        cmd("tp @s 6.5 92 3.5")
        ctl("wait", 22)
        cmd("tick freeze")
        cmd("fill 3 91 2 16 91 14 minecraft:stone")
        cmd("fill 3 92 2 16 97 14 minecraft:air")
        cmd("time set 21600")
        owner = data("@s", "UUID")
        adult, baby = target("adult"), target("baby")
        frozen = "NoAI:1b,NoGravity:1b"
        cmd("summon naturalist:tortoise 6.5 92 5.5")
        normal = stats("@e[type=naturalist:tortoise,limit=1]")
        remove_tortoises()
        spawn(6.5, 5.5, "adult", f"{frozen},Health:17.0f")
        spawn(8.5, 5.5, "baby", f"{frozen},Age:-24000,Health:17.0f")
        spawn(12.5, 5.5, "owned", f"{frozen},Owner:{owner},Health:17.0f")
        summons = {"normal": normal, "explicit_wild": stats(adult), "explicit_tamed": stats(target("owned"))}
        assert all(summons[key] == spawn_java[key][0] for key in summons), summons
        cmd("tick unfreeze")
        ctl("wait", 2)
        cmd("tick freeze")
        first, second = inspect("adult"), inspect("baby")
        actual = {
            "adult_dims": first["dims"], "baby_dims": second["dims"],
            "food": first["food"], "knockback": [first["knockback"], second["knockback"]],
            "head_yaw": first["head_yaw"], "variant": data(adult, "Variant").strip('"'),
            "knockback_resistance": stat(adult, "knockback_resistance"), "step": stat(adult, "step_height"),
            "speed": stat(adult, "movement_speed"),
        }
        cmd("setblock 11 92 5 naturalist:tortoise_egg[eggs=3,variant=1]")
        phases = [inspect("hatch") for _ in range(3)]
        actual["hatch_stages_stone"] = [phase["stage"] for phase in phases]
        for key in ("hatch_baby_ages", "hatch_baby_variants", "hatch_baby_tame", "hatch_offsets"):
            actual[key] = phases[-1][key]
        remove_tortoises()

        cmd("time set 6000")
        cmd("gamemode survival @s")
        cmd("clear @s")
        ctl("tp", 6.5, 92, 3.5)
        ctl("wait", 12)
        spawn(6.5, 5.5, "adult", f"{frozen},Health:17.0f")
        for _ in range(4):
            step(2)
            if inspect("adult")["hidden"]:
                break
        else:
            raise AssertionError(("Empty-handed survival player should trigger hiding",
                                  ctl("state")["player"], inspect("adult"), data(adult, "Pos")))
        replace("minecraft:cactus", 32)
        step(2)
        assert not inspect("adult")["hidden"], "Food in either hand should prevent hiding"
        cmd("tick unfreeze")
        for _ in range(30):
            click(6.5)
            if data(adult, "Owner", optional=True):
                break
        else:
            raise AssertionError("Tortoise resisted all 30 real taming attempts")
        actual["tamed_stats"] = stats(adult)
        cmd(f"damage {adult} 12")
        before = health(adult)
        replace("minecraft:cactus", 2)
        click(6.5)
        inventory = " ".join(cmd("data get entity @s Inventory"))
        actual["owner_heals"] = [health(adult), 1 if re.search(r'count: 1[^0-9]', inventory) else -1]
        assert before == 18.0, before
        if data(adult, "Sitting") == "1b":
            cmd("clear @s")
            click(6.5)
        cmd("clear @s")
        click(6.5)
        actual["owner_sits"] = data(adult, "Sitting") == "1b"
        replace("naturalist:whistle")
        click(6.5)
        actual["owner_whistle_stops_follow"] = data(adult, "FollowingOwner") != "1b"
        cmd("tick freeze")
        remove_tortoises()

        cmd("gamemode creative @s")
        cmd("setblock 6 91 5 minecraft:mud")
        spawn(6.5, 5.5, "adult", "NoGravity:1b,HasEgg:1b,Variant:2")
        for _ in range(44):
            step(2)
            current = inspect("adult")
            if current["laying"]:
                break
        else:
            raise AssertionError("Tortoise failed to visit the valid mud nest")
        assert 2 <= current["counter"] <= 3, current
        step(61 - current["counter"])
        assert inspect("adult")["has_egg"]
        step(1)
        after = inspect("adult")
        actual.update(lay_block=block(6,92,5,"naturalist:tortoise_egg"),
                      lay_fresh=block(6,92,5,"naturalist:tortoise_egg[hatch=0]"),
                      lay_eggs_in_1_4=any(block(6,92,5,f"naturalist:tortoise_egg[eggs={n}]") for n in range(1,5)),
                      lay_variant=next((n for n in range(3) if block(6,92,5,f"naturalist:tortoise_egg[variant={n}]")), -1),
                      lay_post_love_after_ai=after["in_love"],
                      lay_post_flags=[after["has_egg"],after["laying"]])
        remove_tortoises()
        cmd("setblock 6 92 5 minecraft:air")
        cmd("setblock 6 91 5 minecraft:stone")

        for tamed in (False, True):
            owned = f",Owner:{owner}" if tamed else ""
            spawn(6.5, 5.5, "adult", f"NoGravity:1b,InLove:600{owned}")
            spawn(8.5, 5.5, "mate", f"NoGravity:1b,InLove:600{owned}")
            mate = target("mate")
            if tamed:
                replace("minecraft:cactus", 2)
                click(6.5)
                click(8.5)
            has_egg = []
            for _ in range(12):
                step(8)
                has_egg = [data(parent, "HasEgg", optional=True) == "1b" for parent in (adult, mate)]
                if any(has_egg):
                    break
            if tamed:
                actual["bred"] = sum(has_egg) == 1 and all(int(data(parent, "InLove")) == 0 for parent in (adult,mate))
                actual["child_count_after_breed"] = len(ctl("entities", 16, "naturalist:tortoise")["entities"]) - 2
            else:
                actual["wild_mates"] = any(has_egg)
            remove_tortoises()

        print(json.dumps({"summons": summons, "comparisons": actual}), flush=True)
        differences = {key: [java[key], value] for key, value in actual.items()
                       if isinstance(value, float) and abs(java[key] - value) > 1e-7}
        differences.update({key: [java[key], value] for key, value in actual.items()
                            if not isinstance(value, float) and java[key] != value})
        assert not differences, differences
        print(f"Tortoise matched {len(actual)} Java comparisons and all three command-spawn paths.", flush=True)
    finally:
        if socket.exists():
            ctl("quit")
        stop_managed(managed)
