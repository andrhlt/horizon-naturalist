"""Check physical targeting at the Java baby Mole dimensions without a renderer."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-mole-hitbox-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "test.sock"

        def ctl(*args):
            completed = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                       check=True, capture_output=True, text=True, timeout=50)
            return json.loads(completed.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Baby Mole Hitbox", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            ctl("tp", 8.5, 101, 5.5)
            ctl("wait", 12)
            cmd("fill 4 100 4 12 100 12 minecraft:stone")
            cmd("fill 4 101 4 12 107 12 minecraft:air")
            ctl("tp", 8.5, 101, 5.5)
            assert any("Summoned" in row for row in cmd(
                "summon naturalist:mole 8.5 102.55 8.5 {Age:-24000,NoGravity:1b,NoAI:1b,Tags:[hitbox]}"))
            ctl("wait", 8)
            rays = [(8.50, 102.68), (8.66, 102.68), (8.70, 102.68),
                    (8.50, 102.86), (8.50, 102.96)]
            output = {}
            for x, y in rays:
                ctl("tp", x, y - 1.62, 5.5)
                ctl("look-at", x, y, 8.5)
                hit = ctl("looking-at")
                output[f"{x:.2f}/{y:.2f}"] = hit.get("kind") == "entity"
            return output
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--fixed", action="store_true")
    args = parser.parse_args()
    actual = capture(args.version)
    print(json.dumps(actual, indent=2), flush=True)
    if args.fixed:
        assert list(actual.values()) == [True, True, False, True, False], actual
