"""Compare taming, tail detachment and regrowth with pinned Java."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-lizard-", ignore_cleanup_errors=True) as path:
        home = Path(path)
        socket = home / "lizard.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75)
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        def read(who, key):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {who} {key}")
                         if "following entity data: " in row), None)

        def attr(who, attribute):
            return next((float(match[1]) for row in cmd(f"attribute {who} minecraft:{attribute} base get")
                         if (match := re.search(r" is ([0-9.eE+-]+)", row))), None)

        def health(who):
            return float((read(who, "Health") or "0").rstrip("fd"))

        def flag(who, key):
            return read(who, key) in ("1b", "1", "true")

        def feed():
            cmd("item replace entity @s weapon.mainhand with minecraft:spider_eye 2")
            ctl("look-at", 8.5, 101.25, 8.5)
            ctl("mouse", "right", "click")
            ctl("wait", 4)
            stack = read("@s", "SelectedItem") or ""
            return bool(re.search(r"count:\s*1\b", stack))

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Lizard Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 105 15 minecraft:air")
            cmd("tp @s 8.5 101 6.5")
            ctl("wait", 10)
            cmd("gamemode survival @s")
            rows = cmd("summon naturalist:lizard 8.5 101 8.5 {HasTail:1b,NoAI:1b,Tags:[fixture]}")
            assert any("Summoned" in row for row in rows), rows
            lizard = "@e[type=naturalist:lizard,tag=fixture,limit=1]"
            result = {attribute: attr(lizard, attribute)
                      for attribute in ("max_health", "movement_speed", "attack_damage")}
            result["variant"] = json.loads(read(lizard, "Variant") or "null")
            result["initial_tail"] = flag(lizard, "HasTail")
            for _ in range(40):
                assert feed(), "Taming did not consume a spider eye"
                if read(lizard, "Owner"):
                    break
            result["tame_succeeded"] = read(lizard, "Owner") is not None
            result["tame_sitting"] = flag(lizard, "Sitting")
            result["tame_max_health"] = attr(lizard, "max_health")
            result["tame_attack_damage"] = attr(lizard, "attack_damage")
            result["tame_health"] = health(lizard)
            cmd(f'data merge entity {lizard} {{Health:10f,Variant:"naturalist:leopard_gecko"}}')
            cmd(f"damage {lizard} 1 minecraft:generic")
            ctl("wait", 10)
            tail = "@e[type=naturalist:lizard_tail,limit=1]"
            result["dropped_tail_at_half_health"] = not flag(lizard, "HasTail")
            result["tail_variant"] = json.loads(read(tail, "Variant") or "null")
            result["tail_health"] = health(tail)
            consumed = []
            for _ in range(4):
                previous = health(lizard)
                if previous >= 20.0:
                    break
                consumed.append(feed())
                after = health(lizard)
                assert abs(after - min(20.0, previous + 5.0)) < 1e-6, (previous, after)
                if after < 20.0:
                    assert not flag(lizard, "HasTail")
            result["healing_food_consumed"] = bool(consumed) and all(consumed)
            result["health_after_feeding"] = health(lizard)
            result["tail_restored_when_healed"] = flag(lizard, "HasTail")
            return result
        finally:
            if socket.exists():
                ctl("quit")


def compare(output, reference):
    for key, actual in output.items():
        expected = reference[key]
        if type(expected) is float:
            assert actual is not None and abs(actual - expected) < 1e-6, (key, actual, expected)
        else:
            assert actual == expected, (key, actual, expected)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    output = capture(options.version)
    print(json.dumps(output, indent=2), flush=True)
    compare(output, json.loads(options.fixture.read_text()))
