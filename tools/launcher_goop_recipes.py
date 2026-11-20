"""Run real world furnaces, campfire, and the crafting table using Glow Goop."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time


def capture(version, content=None):
    with tempfile.TemporaryDirectory(prefix="nat-goop-recipes-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        def ctl(*args, side="host", env=None):
            result = subprocess.run([version / "horizon-ctl", "--socket", root / (side + ".sock"), *map(str, args)],
                                    capture_output=True, text=True, timeout=75, env=env)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(command):
            return ctl("command", command)["chat"]

        def time_now():
            return next(int(row.split("The game time is ", 1)[1].split()[0])
                        for row in cmd("time query gametime") if "The game time is" in row)

        def advance(ticks):
            end = time_now() + ticks
            cmd(f"tick sprint {ticks}")
            deadline = time.monotonic() + 20
            while time.monotonic() < deadline:
                if time_now() >= end:
                    return
                time.sleep(0.12)
            raise AssertionError(("cook sprint did not complete", ticks))

        def output():
            return next(("naturalist:glow_goop*1" for row in cmd("data get block 6 100 8 Items")
                         if "Slot: 2b" in row and "naturalist:glow_goop" in row), "empty")

        def ground_drops():
            return next(("naturalist:glow_goop*1" for row in cmd(
                "execute as @e[type=minecraft:item,distance=..8] run data get entity @s Item")
                    if "naturalist:glow_goop" in row and "count: 1" in row), "empty")

        def click():
            ctl("mouse", "right", "click")
            ctl("wait", 5)

        try:
            launch = ["launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Glow Goop Recipes", "--creative", "--test-terrain", "--render-distance", 4, "--wait"]
            if content is not None:
                launch.extend(("--dev-mod", content))
            ctl(*launch)
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:block_drops true")
            cmd("tp @s 8.5 100 6.5")
            ctl("wait", 18)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 8.5 100 6.5")
            cmd("gamemode survival @s")
            cmd("tp @s 6.5 100 6.5")

            port = ctl("lan", "open", 0, "commands")["port"]
            guest = root / "guest"
            guest.mkdir()
            script = root / "craft-input.json"
            env = {**os.environ, "HORIZON_TICK_SCRIPT": str(script)}
            guest_launch = ["launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", guest, "--game-dir", guest, "--render-distance", 4]
            if content is not None:
                guest_launch.extend(("--dev-mod", content))
            ctl(*guest_launch, side="guest", env=env)
            ctl("connect", f"127.0.0.1:{port}", "GoopCook", side="guest")
            ctl("wait-for", "loaded", "--timeout", 55, side="guest")
            cmd("gamemode survival GoopCook")
            cmd("tp GoopCook 8.5 100 6.5")
            cmd("clear GoopCook")
            cmd("setblock 8 100 9 minecraft:crafting_table")
            cmd("item replace entity GoopCook hotbar.0 with naturalist:glow_goop 8")
            cmd("item replace entity GoopCook hotbar.1 with minecraft:arrow")
            ctl("wait", 6, side="guest")
            ctl("look-at", 8.5, 100.65, 9.5, side="guest")
            ctl("mouse", "right", "click", side="guest")
            ctl("wait", 6, side="guest")
            menu = ctl("menu", side="guest")
            assert menu["screen"] == "inventory", (menu, ctl("looking-at", side="guest"))
            slots = ["0", *(f"{slot} right" for slot in (100, 101, 102, 103, 105, 106, 107, 108)),
                     "1", "104", "200", "0"]
            steps = {i * 3 + 3: ["click-slot " + slot] for i, slot in enumerate(slots)}
            script.write_text(json.dumps({"name": "spectral", "ticks": 44, "steps": steps}))
            script.with_suffix(".go").touch()
            deadline = time.monotonic() + 12
            while not script.with_suffix(".done").exists() and time.monotonic() < deadline:
                time.sleep(0.07)
            assert script.with_suffix(".done").exists(), "craft click playback did not finish"
            result_menu = ctl("menu", side="guest")
            ctl("close", side="guest")
            shots = cmd("data get entity GoopCook Inventory")
            print(json.dumps({"craft_inventory": shots, "craft_menu": result_menu}), flush=True)
            assert any("minecraft:spectral_arrow" in row and "count: 2" in row for row in shots), shots
            assert all("naturalist:glow_goop" not in row for row in shots), shots
            ctl("quit", side="guest")
            cmd("clear @s")
            result = {"craft": ["naturalist:spectral_arrow_from_glow_goop", "minecraft:spectral_arrow*2"]}

            cmd("tick freeze")
            result["cooking"] = {}
            for oven, ticks in (("furnace", 200), ("smoker", 100)):
                cmd(f"setblock 6 100 8 minecraft:{oven}")
                cmd("item replace block 6 100 8 container.0 with minecraft:glow_berries")
                cmd("item replace block 6 100 8 container.1 with minecraft:coal")
                advance(ticks - 1)
                before = output()
                advance(1)
                result["cooking"]["minecraft:" + oven] = [before, output()]
            print(json.dumps(result), flush=True)

            cmd("tick unfreeze")
            cmd("tp @s 11.5 100 6.5")
            cmd("setblock 11 100 8 minecraft:campfire[lit=true]")
            cmd("item replace entity @s hotbar.0 with minecraft:glow_berries")
            ctl("look-at", 11.5, 100.25, 8.5)
            click()
            used = not any("minecraft:glow_berries" in row for row in cmd("data get entity @s SelectedItem"))
            assert any("minecraft:glow_berries" in row
                       for row in cmd("data get block 11 100 8 Items")), used
            cmd("tp @s 14.5 100 4.5")
            advance(150)
            before = ground_drops()
            advance(55)
            result["campfire"] = [used, "empty" if used else "glow_berries",
                [] if before == "empty" else [before], [ground_drops()]]
            return result
        finally:
            for side in ("guest", "host"):
                if (root / (side + ".sock")).exists():
                    ctl("quit", side=side)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("java", type=Path)
    parser.add_argument("--content", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.java.read_text())
    actual = capture(args.version, args.content)
    print(json.dumps(actual, indent=2), flush=True)
    assert actual == {key: expected[key] for key in actual}, actual
