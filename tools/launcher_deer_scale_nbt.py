"""Compare Java and Horizon deer scale during standard NBT commands."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

from deer_scale_nbt_reference import capture


def replay(version, expected):
    with tempfile.TemporaryDirectory(prefix="nat-scale-nbt-", ignore_cleanup_errors=True) as temp:
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
                    "--world", "Naturalist Deer Scale NBT", "--creative", "--test-terrain",
                    "--render-distance", 4, "--wait")
            control("wait", 25)
            observed = capture(command, None)
            for key in ("modified_nbt", "after_remove_nbt"):
                nbt = observed.pop(key)
                assert nbt and "naturalist:fixture_scale" in nbt and "add_multiplied_total" in nbt, nbt
                nbt = expected.pop(key)
                assert "naturalist:fixture_scale" in nbt and "add_multiplied_total" in nbt, nbt
            print(json.dumps(observed, indent=2), flush=True)
            assert observed == expected, observed
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    replay(args.version, json.loads(args.reference.read_text()))
