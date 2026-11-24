"""Compare Zebra summoning, food, chest equipment and breeding headlessly."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
from launcher_managed import launch_managed, wait_managed


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

        def feed_at(x, kind, item):
            mobs = ctl("entities", 12, kind)["entities"]
            target_entity = min(mobs, key=lambda row: abs(row["pos"][0] - x) + abs(row["pos"][2] - 8.5))
            command(f"item replace entity @s weapon.mainhand with {item}")
            ctl("look-at-entity", target_entity["id"])
            ctl("mouse", "right", "click")
            ctl("wait", 2)

        play = None
        try:
            play = launch_managed(version, socket, home, "Naturalist Zebra Fixture",
                                  creative=True, test_terrain=True)
            command("gamerule minecraft:spawn_mobs false")
            ctl("wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("fill 0 101 0 15 106 15 minecraft:air")
            command("tp @s 8.5 101 5.5")
            command("gamemode survival @s")
            command("tick freeze")
            target = "@e[type=naturalist:zebra,tag=specimen,limit=1]"
            summon("naturalist:zebra", "8.5 101 8.5", "{NoAI:1b,Tags:[specimen]}")
            raw_variant = data(target, "Variant")
            output = {"default_stats": stats(target),
                      "variant": json.loads(raw_variant) if raw_variant else None,
                      "coverage_gaps": []}
            command(f"kill {target}")
            # Age-profile and seeded RNG probes live in the developer fixture
            # package, not the installed natural candidate. They aren't
            # required for the beta interaction path; keep them explicit.
            if output["variant"] is None:
                output["coverage_gaps"].append("Java Zebra Variant is absent from entity NBT")
            output["coverage_gaps"].append("age dimensions and exact spawn RNG not probed")
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
                # Use normal held-item interaction to enter love mode; editing
                # InLove in summon NBT does not initialize the live mob state.
                feed_at(7.5, "naturalist:zebra", "minecraft:golden_carrot")
                feed_at(9.5, mate, "minecraft:golden_carrot")
                if mate == "naturalist:zebra":
                    assert data("@e[tag=zebra_parent,limit=1]", "InLove") == "600", (mate, "zebra_parent")
                    assert data("@e[tag=zebra_partner,limit=1]", "InLove") == "600", (mate, "zebra_partner")
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
            wait_managed(play)


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
    assert_close(actual["default_stats"], expected["default_stats"])
    for key in ("feeding", "chest_added", "saddle_added", "initially_chested"):
        assert actual[key] == expected[key], (key, actual[key], expected[key])
    assert actual["can_mate"] == expected["can_mate"][:3], (actual["can_mate"], expected["can_mate"])
    print("Zebra unverified:", "; ".join(actual["coverage_gaps"]), flush=True)
