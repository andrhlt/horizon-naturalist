"""Verify a rhino's charge preparation and shield stun in headless Play."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-rhino-", ignore_cleanup_errors=True) as path:
        home = Path(path)
        socket = home / "rhino.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     capture_output=True, text=True, timeout=75)
            if process.returncode:
                raise RuntimeError((args, process.stdout, process.stderr))
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

        def attr(who, name):
            rows = cmd(f"attribute {who} minecraft:{name} base get")
            return next((float(match[1]) for row in rows if (match := re.search(r" is ([0-9.eE+-]+)", row))), None)

        def block_matches(state):
            rows = cmd(f"execute if block {state} run say rhino-block-matched")
            return any("rhino-block-matched" in row for row in rows)

        def shield_charge(rhino):
            cmd("item replace entity @s weapon.mainhand with minecraft:shield")
            ctl("look-at", 8.5, 102.25, 11.5)
            ctl("mouse", "right", "down")
            ctl("wait", 8)
            assert ctl("state")["using"] is not None, "Fixture shield was not raised"
            cmd("item replace entity @s armor.head with minecraft:carved_pumpkin")
            stun = 0
            for _ in range(50):
                ctl("wait", 3)
                stun = int(read(rhino, "StunTick") or 0)
                if stun > 0:
                    break
            speed = attr(rhino, "movement_speed")
            ctl("mouse", "right", "up")
            cmd("item replace entity @s armor.head with minecraft:air")
            return stun, speed

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Rhino Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 106 15 minecraft:air")
            cmd("tp @s 8.5 101 5.5")
            ctl("wait", 10)
            cmd("gamemode survival @s")
            cmd("effect give @s minecraft:resistance infinite 4 true")
            rhino = "@e[type=naturalist:rhino,tag=charge_fixture,limit=1]"
            rows = cmd("summon naturalist:rhino 8.5 101 11.5 {NoAI:1b,Tags:[charge_fixture]}")
            assert any("Summoned" in row for row in rows), rows

            result = {name: attr(rhino, name) for name in (
                "max_health", "movement_speed", "attack_damage", "attack_knockback",
                "knockback_resistance", "step_height",
            )}
            result["variant"] = json.loads(read(rhino, "Variant") or "null")
            cmd("item replace entity @s weapon.mainhand with minecraft:wheat 2")
            ctl("look-at", 8.5, 102.25, 11.5)
            ctl("mouse", "right", "click")
            ctl("wait", 2)
            result["no_food"] = not int(read(rhino, "InLove") or 0)
            cmd(f"data merge entity {rhino} {{NoAI:0b}}")
            ctl("wait", 25)

            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("setblock 8 100 9 minecraft:dirt")
            cmd("setblock 8 101 9 minecraft:dandelion")
            cmd("setblock 9 101 9 minecraft:torch")
            result["shield_stun_ticks"], result["stunned_speed"] = shield_charge(rhino)
            result["charge_removed_dandelion"] = block_matches("8 101 9 minecraft:air")
            result["charge_preserved_torch"] = block_matches("9 101 9 minecraft:torch")
            ctl("wait", 65)
            result["recovered_speed"] = attr(rhino, "movement_speed")
            result["horn_registered"] = any("Gave" in row
                for row in ctl("command", "give @s naturalist:rhino_horn")["chat"])

            cmd(f"kill {rhino}")
            cmd("gamerule minecraft:mob_griefing false")
            cmd("setblock 8 101 9 minecraft:dandelion")
            cmd("tp @s 8.5 101 5.5")
            cmd("summon naturalist:rhino 8.5 101 11.5 {Tags:[nogrief_fixture]}")
            ctl("wait", 12)
            guard = "@e[type=naturalist:rhino,tag=nogrief_fixture,limit=1]"
            no_grief_stun, _ = shield_charge(guard)
            assert no_grief_stun > 0, "The same charge must run with mob_griefing disabled"
            result["no_grief_dandelion_survived"] = block_matches("8 101 9 minecraft:dandelion")
            return result
        finally:
            if socket.exists():
                ctl("quit")


def compare(output, reference):
    for key, actual in output.items():
        if key == "stunned_speed":
            assert actual == 0.0, (key, actual)
        elif key == "recovered_speed":
            assert actual == reference["movement_speed"], (key, actual)
        elif key == "shield_stun_ticks":
            assert 50 <= actual <= reference[key], (key, actual)
        elif type(reference[key]) is float:
            assert actual is not None and abs(actual - reference[key]) < 1e-6, (key, actual)
        else:
            assert actual == reference[key], (key, actual)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    output = capture(options.version)
    print(json.dumps(output, indent=2), flush=True)
    compare(output, json.loads(options.fixture.read_text()))
