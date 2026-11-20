"""Measure whether a live Horizon Ant climbs to food on the Java arena's ledge."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-ant-wall-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "wall.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def ants():
            return ctl("entities", 32, "naturalist:ant")["entities"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ant Wall Navigation", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 6.5 103 14.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("fill 11 100 5 11 102 7 minecraft:stone")
            cmd("tp @s 6.5 100 14.5")
            cmd("summon naturalist:ant 10.6 100 6.5 {Tags:[wall],HillCooldown:1200}")
            ctl("wait", 3)
            ant_id = ants()[0]["id"]
            selector = "@e[tag=wall,limit=1]"
            if ant_id & 1:
                cmd(f"kill {selector}")
                selector = "@e[tag=wall2,limit=1]"
                cmd("summon naturalist:ant 10.6 100 6.5 {Tags:[wall2],HillCooldown:1200}")
                ctl("wait", 3)
                ant_id = next(entity["id"] for entity in ants() if entity["health"] > 0)
            assert ant_id & 1 == 0, ants()
            cmd('summon minecraft:item 11.5 103 6.5 {Item:{id:"minecraft:apple",count:1},PickupDelay:0s,NoGravity:1b}')
            positions = []
            for _ in range(24):
                ctl("wait", 5)
                positions.append(next(entity["pos"] for entity in ants() if entity["id"] == ant_id))
            found = any("following entity data: " in row for row in cmd(f"data get entity {selector} CarriedFood"))
            return {"high": max(pos[1] for pos in positions) - 100, "food": found,
                    "positions": positions}
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())["wall"]
    observed = capture(args.version)
    print(json.dumps({"expected": expected, "observed": observed}, indent=2))
    assert observed["food"] == expected["food"] and abs(observed["high"] - expected["high"]) <= 0.25
