"""Exercise Snail Egg delayed hatching and support loss in a built headless world."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time
from launcher_setup import prepare_home


def capture(version, jar, resources=None):
    with tempfile.TemporaryDirectory(prefix="nat-snail-eggs-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "snail-eggs.sock"
        prepare_home(directory, jar)

        def ctl(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def game_time():
            return int(next(row for row in cmd("time query gametime") if "The game time is " in row).split("The game time is ", 1)[-1].split()[0])

        def sprint(ticks):
            before = game_time()
            cmd(f"tick sprint {ticks}")
            deadline = time.monotonic() + 30
            while game_time() < before + ticks:
                assert time.monotonic() < deadline, (ticks, before, game_time())
                time.sleep(0.05)
            ctl("wait", 5)

        def place_egg(x, y=100, z=8):
            cmd("item replace entity @s weapon.mainhand with naturalist:snail_eggs 1")
            cmd(f"tp @s {x + 0.5} {y} {z - 1.5}")
            ctl("look-at", x + 0.5, y - 0.05, z + 0.5)
            ctl("mouse", "right", "click")
            ctl("wait", 3)
            return cmd(f"execute if block {x} {y} {z} naturalist:snail_eggs")

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", resources or version / ".local/horizon-client/26.3", "--home", directory,
                "--game-dir", directory, "--world", "Snail Eggs Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 7.5 100 6.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 7.5 100 6.5")
            cmd("forceload add 0 0")
            cmd("tick freeze")
            placed = place_egg(7)
            assert any("Test passed" in row for row in placed), placed
            sprint(599)
            still_eggs = cmd("execute if block 7 100 8 naturalist:snail_eggs")
            assert any("Test passed" in row for row in still_eggs), still_eggs
            sprint(1801)
            removed = cmd("execute if block 7 100 8 minecraft:air")
            assert any("Test passed" in row for row in removed), removed
            snails = ctl("entities", 24, "naturalist:snail")["entities"]
            ages = cmd("execute as @e[type=naturalist:snail] run data get entity @s Age")
            observed = {"count": len(snails), "all_babies": all(row["baby"] for row in snails), "age_rows": ages}
            print(json.dumps(observed, indent=2), flush=True)
            assert 2 <= len(snails) <= 5 and observed["all_babies"], observed
            sprint(2400)
            after_retry_window = ctl("entities", 24, "naturalist:snail")["entities"]
            assert len(after_retry_window) == len(snails), (snails, after_retry_window)

            placed_then_replaced = place_egg(5)
            assert any("Test passed" in row for row in placed_then_replaced), placed_then_replaced
            cmd("setblock 5 100 8 minecraft:stone")
            sprint(2400)
            replaced = ctl("entities", 24, "naturalist:snail")["entities"]
            assert len(replaced) == len(snails), (snails, replaced)

            cmd("setblock 4 99 8 minecraft:stone_slab[type=bottom]")
            unsupported_placement = place_egg(4)
            assert not any("Test passed" in row for row in unsupported_placement), unsupported_placement
            held = cmd("data get entity @s SelectedItem")
            assert any("naturalist:snail_eggs" in row for row in held), held
            cmd("clear @s naturalist:snail_eggs")

            cmd("setblock 6 99 8 minecraft:stone")
            placed_on_stone = place_egg(6)
            assert any("Test passed" in row for row in placed_on_stone), placed_on_stone
            cmd("setblock 6 99 8 minecraft:air")
            unsupported = cmd("execute if block 6 100 8 minecraft:air")
            assert any("Test passed" in row for row in unsupported), unsupported
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--jar", type=Path, required=True)
    parser.add_argument("--resources", type=Path)
    args = parser.parse_args()
    capture(args.version, args.jar, args.resources)
