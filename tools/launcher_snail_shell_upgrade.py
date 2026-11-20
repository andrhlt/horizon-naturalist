"""Replay Java Snail shell placement, single-plant storage, and silk-independent drops."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time


def capture(version, upgrade_from):
    target_version = version
    version = upgrade_from
    with tempfile.TemporaryDirectory(prefix="nat-shell-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "shell.sock"

        def ctl(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def read(line):
            marker = "following"
            return next((row.rsplit(": ", 1)[-1] for row in cmd(line) if marker in row), None)

        def click_block(y):
            ctl("look-at-block", 8, y, 8)
            ctl("mouse", "right", "click")
            ctl("wait", 6)

        def test_state(properties):
            return any("Test passed" in row
                       for row in cmd(f"execute if block 8 100 8 naturalist:snail_shell[{properties}]"))

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", directory,
                "--game-dir", directory, "--world", "Snail Shell Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 10.5 100 6.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 10.5 100 6.5")
            cmd("gamemode survival @s")
            cmd("clear @s")
            cmd("item replace entity @s weapon.mainhand with naturalist:snail_shell[minecraft:custom_data={Color:10}]")
            ctl("wait", 6)
            click_block(99)
            color = "purple" if test_state("color=purple") else None
            rotation = 2 if test_state("rotation=2") else None

            cmd("item replace entity @s weapon.mainhand with minecraft:cactus 2")
            ctl("wait", 6)
            click_block(100)
            cactus = read("data get entity @s SelectedItem.count")
            stored_cactus = read("data get block 8 100 8 Flower.id")
            cmd("item replace entity @s weapon.mainhand with minecraft:dandelion 2")
            ctl("wait", 6)
            click_block(100)
            used = read("data get entity @s SelectedItem.count")
            flower_id = read("data get block 8 100 8 Flower.id")
            flower_count = read("data get block 8 100 8 Flower.count")
            click_block(100)
            second = read("data get entity @s SelectedItem.count")

            before_upgrade = {"purple": test_state("color=purple"), "rotation": test_state("rotation=2"),
                              "flower_id": read("data get block 8 100 8 Flower.id"),
                              "flower_count": read("data get block 8 100 8 Flower.count")}
            cmd("save-all flush")
            ctl("quit")
            deadline = time.monotonic() + 10
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.1)
            version = target_version
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", directory,
                "--game-dir", directory, "--world", "Snail Shell Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            ctl("wait", 20)
            after_upgrade = {"purple": test_state("color=purple"), "rotation": test_state("rotation=2"),
                             "flower_id": read("data get block 8 100 8 Flower.id"),
                             "flower_count": read("data get block 8 100 8 Flower.count")}
            print(json.dumps({"before_upgrade": before_upgrade, "after_upgrade": after_upgrade}), flush=True)
            assert before_upgrade == after_upgrade == {"purple": True, "rotation": True,
                                                       "flower_id": '"minecraft:dandelion"', "flower_count": "1"}
            cmd("tick freeze")
            cmd("setblock 8 100 8 minecraft:air destroy")
            drops = cmd("execute as @e[type=minecraft:item] run data get entity @s Item")
            result = {
                "color": color, "rotation": rotation,
                "refuses_cactus": cactus == "2" and stored_cactus is None,
                "one_flower": used == "1" and flower_count == "1" and flower_id == '"minecraft:dandelion"',
                "refuses_second": second == "1",
                "saves_flower": flower_id == '"minecraft:dandelion"',
                "colored_drop": any("naturalist:snail_shell" in row and "Color: 10" in row for row in drops),
                "flower_drop": any("minecraft:dandelion" in row and "count: 1" in row for row in drops),
            }
            print(json.dumps({"result": result, "drops": drops}, indent=2), flush=True)
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--upgrade-from", type=Path, required=True)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    actual = capture(args.version, args.upgrade_from)
    assert actual == expected, (actual, expected)
