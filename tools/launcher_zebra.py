"""Compare Zebra summoning, food, chest equipment and breeding headlessly."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


ATTRIBUTES = ("max_health", "movement_speed", "jump_strength", "step_height", "follow_range")


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-zebra-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "zebra.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    check=True, capture_output=True, text=True, timeout=75)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def data(selector, key):
            return next((line.split("following entity data: ", 1)[1]
                         for line in command(f"data get entity {selector} {key}")
                         if "following entity data: " in line), None)

        def stats(selector):
            values = []
            for attribute in ATTRIBUTES:
                lines = command(f"attribute {selector} minecraft:{attribute} base get")
                values.append(next(float(line.rsplit(" is ", 1)[1].split()[0])
                                   for line in lines if " is " in line))
            return values

        def step(ticks):
            command(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        def summon(kind, pos, nbt=""):
            lines = command(f"summon {kind} {pos} {nbt}".strip())
            assert any("summoned" in line.lower() for line in lines), lines

        def interact(item, count):
            command(f"item replace entity @s weapon.mainhand with {item} {count}")
            ctl("wait", 4)
            ctl("look-at", 8.5, 101.75, 8.5)
            ctl("mouse", "right", "click")
            ctl("wait", 3)
            return (ctl("status").get("held") or {}).get("count", 0)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist Zebra Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 5.5")
            ctl("wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("fill 0 101 0 15 106 15 minecraft:air")
            command("tp @s 8.5 101 5.5")
            command("gamemode survival @s")
            command("tick freeze")
            target = "@e[type=naturalist:zebra,tag=specimen,limit=1]"
            summon("naturalist:zebra", "8.5 101 8.5", "{NoAI:1b,Tags:[specimen]}")
            output = {"default_stats": stats(target), "variant": json.loads(data(target, "Variant"))}
            for label, age in (("adult", 0), ("juvenile", -24000)):
                command(f"data merge entity {target} {{Age:{age}}}")
                rows = command("naturalist_fixture_mob_age_profile naturalist:zebra")
                row = next(line.split("Naturalist mob age profile ", 1)[1] for line in rows
                           if "Naturalist mob age profile " in line)
                profile = json.loads(row)
                output[label + "_dimensions"] = profile["dimensions"]
            command(f"kill {target}")
            rows = command("naturalist_fixture_zebra_roll")
            marker = "Naturalist zebra rolls "
            output["seeded"] = next(json.loads(row.split(marker, 1)[1]) for row in rows if marker in row)
            output["randomized"] = []
            for _ in range(8):
                summon("naturalist:zebra", "8.5 101 8.5")
                who = "@e[type=naturalist:zebra,sort=nearest,limit=1]"
                output["randomized"].append([*stats(who), float(data(who, "Health").rstrip("f"))])
                command(f"kill {who}")
            # Clear the earlier specimens before ray-casting real clicks.
            command("tick unfreeze")
            ctl("wait", 26)
            # Freeze during feeding so passive equine healing cannot change the result.
            output["feeding"] = {}
            for item in reference["feeding"]:
                command("tick freeze")
                summon("naturalist:zebra", "8.5 101 8.5", "{NoAI:1b,Health:48.0f,Temper:0,Tags:[specimen]}")
                remaining = interact(item, 2)
                output["feeding"][item] = [remaining == 1, int(data(target, "Temper")),
                                           float(data(target, "Health").rstrip("f")), remaining]
                command(f"kill {target}")
                command("tick unfreeze")
                ctl("wait", 24)
            summon("naturalist:zebra", "8.5 101 8.5", "{NoAI:1b,Tame:1b,Tags:[specimen]}")
            output["initially_chested"] = data(target, "ChestedHorse") == "1b"
            remaining = interact("minecraft:chest", 1)
            output["chest_added"] = data(target, "ChestedHorse") == "1b" and remaining == 0
            remaining = interact("minecraft:saddle", 1)
            equipment = data(target, "equipment") or ""
            output["saddle_added"] = "minecraft:saddle" in equipment and remaining == 0
            command(f"kill {target}")
            ctl("wait", 26)
            command("tick freeze")
            babies = []
            for mate in ("naturalist:zebra", "minecraft:donkey", "minecraft:horse"):
                for kind, x, label in (("naturalist:zebra", 7.5, "zebra_parent"), (mate, 9.5, "zebra_partner")):
                    summon(kind, f"{x} 101 8.5", f"{{Age:0,Tame:1b,InLove:600,Tags:[{label}]}}")
                    assert data(f"@e[tag={label},limit=1]", "InLove") == "600", (kind, label)
                step(105)
                babies.append([row["type"] for row in ctl("entities", 20)["entities"]
                               if row.get("baby") and row.get("health", 0) > 0])
                for kind in {"naturalist:zebra", mate, "minecraft:mule"}:
                    command(f"kill @e[type={kind}]")
                step(6)
            output["can_mate"] = ["naturalist:zebra" in babies[0], bool(babies[1]), bool(babies[2])]
            return output
        finally:
            if socket.exists():
                ctl("quit")


def assert_close(actual, expected):
    assert all(abs(a - b) < 1e-5 for a, b in zip(actual, expected, strict=True)), (actual, expected)


if __name__ == "__main__":
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("version", type=Path)
    cli.add_argument("reference", type=Path)
    args = cli.parse_args()
    expected = json.loads(args.reference.read_text())
    actual = capture(args.version, expected)
    print(json.dumps(actual, indent=2), flush=True)
    for key in ("default_stats", "adult_dimensions", "juvenile_dimensions"):
        assert_close(actual[key], expected[key])
    for actual_roll, expected_roll in zip(actual["seeded"], expected["randomized"], strict=True):
        assert_close(actual_roll, expected_roll[:3])
    spawn_reference = json.loads(args.reference.with_name("zebra-spawn-java.json").read_text())
    for health, speed, jump, step_height, follow_range, current_health in actual["randomized"]:
        assert 15 <= health <= 30 and 0.125 <= speed <= 0.35 and 0.4 <= jump <= 0.5 + 1e-7
        assert_close([step_height, follow_range], expected["default_stats"][-2:])
        assert current_health in {row[-1] for row in spawn_reference["normal"]}, current_health
    for key in ("variant", "feeding", "chest_added", "saddle_added", "initially_chested"):
        assert actual[key] == expected[key], (key, actual[key], expected[key])
    assert actual["can_mate"] == expected["can_mate"][:3], (actual["can_mate"], expected["can_mate"])
