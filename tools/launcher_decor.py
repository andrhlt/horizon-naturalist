"""Replay actual decorative blocks and Frog-produced Froglass in headless Horizon."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

from decor_block_reference import capture_blocks
from froglass_reference import capture_froglass


def capture(version, fixture, probe):
    with tempfile.TemporaryDirectory(prefix="nat-decor-", ignore_cleanup_errors=True) as folder:
        home = Path(folder)
        socket = home / "control.sock"

        def ctl(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=75)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist Decorations", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("tp @s 4.5 102 4.5")
            command("gamemode spectator @s")
            actual = probe(command, lambda ticks: ctl("wait", ticks))
            print(json.dumps(actual, indent=2), flush=True)
            expected = json.loads(fixture.read_text())
            assert actual == expected, (expected, actual)
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("kind", choices=("blocks", "froglass"))
    args = parser.parse_args()
    capture(args.version, args.fixture, {"blocks": capture_blocks, "froglass": capture_froglass}[args.kind])
