"""Replay Bird contact and cramming on an installed headless Horizon build."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

from bird_contact_arena import capture, stepper


def run(version):
    with tempfile.TemporaryDirectory(prefix="nat-bird-contact-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "contact.sock"

        def ctl(*args):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=70,
            )
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", root,
                "--game-dir", root, "--render-distance", 4, "--world", "Bird contact",
                "--creative", "--test-terrain", "--wait")
            command("tp @s 8.5 108 8.5")
            ctl("wait", 20)
            return capture(command, stepper(command))
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    print(json.dumps(run(args.version), indent=2), flush=True)
