"""Compare creative pick and survival Command Block hatch through actual headless controls."""
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile

native = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-tortoise-eggs-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run([native / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    def block(name):
        return any("test passed" in row.lower() for row in cmd(f"execute if block 7 92 7 {name}"))

    try:
        ctl("launch", "--headless", "--game", native / "horizon-client",
            "--resources", native / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Tortoise eggs", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("time set 6000")
        cmd("tp @s 7.5 92 5.5")
        ctl("wait", 25)
        cmd("fill 3 91 2 16 91 14 minecraft:stone")
        cmd("fill 3 92 2 16 97 14 minecraft:air")
        cmd("tp @s 7.5 92 5.5")
        ctl("wait", 8)
        placement = cmd("setblock 7 92 7 naturalist:tortoise_egg[eggs=2,variant=2]")
        assert block("naturalist:tortoise_egg[variant=2]"), placement
        actual = {"picked_egg_custom_data": []}
        ctl("wait", 8)
        for include_data in (False, True):
            cmd("clear @s")
            ctl("look-at", 7.5, 92.15, 7.5)
            print("Aim", ctl("looking-at"), flush=True)
            if include_data:
                ctl("key", "ctrl", "down")
            ctl("mouse", "middle", "click")
            if include_data:
                ctl("key", "ctrl", "up")
            ctl("wait", 4)
            held = " ".join(cmd("data get entity @s Inventory"))
            tag = re.search(r'Variant:\s*(\d+)', held)
            actual["picked_egg_custom_data"].append("{Variant:" + tag[1] + "}" if tag else held)

        cmd("clear @s")
        cmd("gamemode survival @s")
        cmd("item replace entity @s weapon.mainhand with minecraft:command_block")
        cmd("setblock 7 92 7 naturalist:tortoise_egg[eggs=2,variant=1]")
        ctl("look-at", 7.5, 92.15, 7.5)
        ctl("mouse", "left", "down")
        try:
            for _ in range(15):
                ctl("wait", 3)
                if not block("naturalist:tortoise_egg"):
                    break
        finally:
            ctl("mouse", "left", "up")
        actual["command_block_breaks_egg"] = block("minecraft:air")
        juveniles = ctl("entities", 7, "naturalist:tortoise")["entities"]
        actual["command_block_babies"] = []
        for mob in juveniles:
            assert mob["type"] == "naturalist:tortoise", mob
        for _ in juveniles:
            selector = "@e[type=naturalist:tortoise,sort=nearest,tag=!seen,limit=1]"
            variant = " ".join(cmd(f"data get entity {selector} Variant"))
            age = " ".join(cmd(f"data get entity {selector} Age"))
            variant_value = re.search(r'data: "([^"]+)"', variant)
            age_value = re.search(r'data: (-\d+)', age)
            if variant_value and age_value:
                age_at_spawn = int(age_value[1])
                assert -24000 <= age_at_spawn < -23980, age_at_spawn
                actual["command_block_babies"].append(variant_value[1] + ":-24000")
            cmd(f"tag {selector} add seen")
        print(json.dumps(actual), flush=True)
        assert all(value == reference[key] for key, value in actual.items()), actual
        print("Both Tortoise picks retain Variant. Survival Command Block mining hatches two green juveniles.")
    finally:
        if socket.exists():
            ctl("quit")
