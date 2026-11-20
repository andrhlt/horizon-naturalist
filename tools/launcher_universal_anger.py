"""Replay Boar and Bear retaliation under both universal anger settings."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-universal-anger-", ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        socket = home / "anger.sock"

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

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Universal Anger", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")

        def close():
            if socket.exists():
                ctl("quit")
            deadline = time.monotonic() + 15
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.05)
            assert not socket.exists(), "The headless control socket did not close"

        try:
            launch()
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:advance_time false")
            cmd("time set 2000")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 107 15 minecraft:air")
            cmd("difficulty normal")
            cmd("gamemode survival @s")
            cmd("effect give @s minecraft:resistance infinite 3 true")
            outcomes = {}
            for animal in ("boar", "bear", "black_bear"):
                outcomes[animal] = {}
                for enabled in ("true", "false"):
                    cmd(f"gamerule minecraft:universal_anger {enabled}")
                    cmd("effect give @s minecraft:instant_health 1 4 true")
                    cmd("tp @s 8.5 101 6.5")
                    ctl("wait", 4)
                    tagged = f"@e[type=naturalist:{animal},tag=anger_fixture,limit=1]"
                    health = float(read("@s", "Health").rstrip("Ff"))
                    cmd(f"summon naturalist:{animal} 8.5 101 8.0 {{Tags:[anger_fixture],PersistenceRequired:1b}}")
                    cmd(f"damage {tagged} 1 minecraft:player_attack by @s")
                    ctl("wait", 60)
                    outputs = {"retaliated": float(read("@s", "Health").rstrip("Ff")) < health}
                    outcomes[animal][enabled] = outputs
                    cmd(f"kill {tagged}")
            return outcomes
        finally:
            close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    result = capture(options.version)
    expected = json.loads(options.fixture.read_text())["universal_anger"]
    print(json.dumps(result, indent=2), flush=True)
    differences = [(animal, rule, sample["retaliated"])
                   for animal, values in result.items()
                   for rule, sample in values.items()
                   if sample["retaliated"] != expected[animal][rule]["retaliated"]]
    assert not differences, differences
