"""Compare normal player dye, capture, and release interactions against Java."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-snail-capture-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "snail.sock"

        def control(*args):
            completed = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                capture_output=True, text=True, check=True, timeout=65)
            return json.loads(completed.stdout.splitlines()[-1])

        def command(text):
            return control("command", text)["chat"]

        def data(target, key):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in command(f"data get entity {target} {key}")
                         if marker in row), None)

        def click_snail():
            mobs = control("entities", 5, "naturalist:snail")["entities"]
            assert len(mobs) == 1, mobs
            looked = control("look-at-entity", mobs[0]["id"])
            assert looked["target"]["kind"] == "entity", looked
            control("mouse", "right", "click")
            control("wait", 5)

        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3", "--home", home,
                    "--game-dir", home, "--world", "Naturalist Snail Capture", "--creative",
                    "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 7.5 100 6.5")
            control("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 107 15 minecraft:air")
            command("tp @s 7.5 100 6.5")
            command("gamemode survival @s")
            original = "@e[tag=snail_capture,limit=1]"
            command("summon naturalist:snail 7.5 100 8.5 "
                    "{Tags:[snail_capture],NoAI:1b,NoGravity:1b,Color:0b}")
            command("item replace entity @s weapon.mainhand with minecraft:purple_dye 2")
            control("wait", 4)
            click_snail()
            dyed_once = data(original, "Color") == "10b" and control("status")["held"]["count"] == 1
            click_snail()
            retained = control("status")["held"]["count"] == 1
            command(f"data merge entity {original} {{Age:-3000,Health:2.0f,HasEgg:1b}}")
            command("item replace entity @s weapon.mainhand with naturalist:capture_net")
            control("wait", 4)
            click_snail()
            saved = data("@s", "Inventory")
            assert saved, saved
            custom = re.search(r'"minecraft:custom_data":\s*\{([^{}]*)\}', saved)
            assert custom, saved
            payload = custom[1]
            captured = data(original, "UUID") is None and '"minecraft:damage": 1' in saved
            captured = captured and "UUID" not in payload and not re.search(r"\bid:", payload)
            color = re.search(r"\bColor:\s*(\d+)", payload)
            age = re.search(r"\bAge:\s*(-?\d+)", payload)
            control("select", "naturalist:snail")
            control("look-at-block", 7, 99, 8)
            control("mouse", "right", "click")
            control("wait", 6)
            released = "@e[type=naturalist:snail,limit=1]"
            return {
                "dyes_once": dyed_once,
                "retains_matching_dye": retained,
                "body_capture": captured,
                "caught_color": int(color[1]) if color else None,
                "caught_age": int(age[1]) if age else 0,
                "restores_color_and_health": data(released, "Color") == "10b" and data(released, "Health") == "2.0f",
                "carried_egg_omitted": "HasEgg" not in payload and data(released, "HasEgg") in ("0b", "false"),
            }
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.reference.read_text())
    actual = capture(args.version)
    print(json.dumps(actual, indent=2))
    assert actual == expected, (actual, expected)
