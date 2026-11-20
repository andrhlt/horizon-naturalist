"""Replay two simultaneous projectile hits on Horizon’s deer herd."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

from deer_projectile_reference import scenario


def replay(version, expected):
    with tempfile.TemporaryDirectory(prefix="nat-herd-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "step.sock"

        def control(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    check=True, capture_output=True, text=True, timeout=45)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return control("command", line)["chat"]

        try:
            control("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                    "--world", "Naturalist Projectile Herd", "--creative", "--test-terrain",
                    "--render-distance", 4, "--wait")
            control("wait", 25)
            actual = scenario(command, lambda: control("wait", 3))
            print(json.dumps(actual), flush=True)
            assert actual == expected, (actual, expected)
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    replay(args.version, json.loads(args.reference.read_text()))
