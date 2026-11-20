"""Compare the native mob health modifiers against the headless Java reference."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

from baby_health_reference import capture


def replay(version, expected, *, isolated=False):
    with tempfile.TemporaryDirectory(prefix="nat-baby-health-", ignore_cleanup_errors=True) as temp:
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
                    "--world", "Naturalist Baby Health", "--creative", "--test-terrain",
                    "--render-distance", 4, "--wait")
            command("tp @s 8.5 151 5.5")
            control("wait", 25)
            actual = capture(command, lambda: control("wait", 7),
                             damage="juvenile_damaged" in expected,
                             live_merge="after_live_merge" in expected,
                             species=tuple(expected["before_first_tick"]), isolated=isolated)
            print(json.dumps(actual, indent=2), flush=True)
            for phase, species in expected.items():
                assert actual[phase].keys() == species.keys()
                for name, pair in species.items():
                    assert all(abs(a - b) < 1e-5 for a, b in zip(actual[phase][name], pair, strict=True)), (phase, name, actual[phase][name], pair)
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--isolated", action="store_true")
    args = parser.parse_args()
    replay(args.version, json.loads(args.reference.read_text()), isolated=args.isolated)
