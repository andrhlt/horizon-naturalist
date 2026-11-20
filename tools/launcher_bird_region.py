"""Compare chunk generation Bird groups with the two Java region RNG implementations."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, expected):
    with tempfile.TemporaryDirectory(prefix="nat-bird-region-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "region.sock"

        def control(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=65)
            return json.loads(reply.stdout.splitlines()[-1])

        try:
            control("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                    "--world", "Naturalist Bird Region", "--seed", "1234", "--creative",
                    "--render-distance", 4, "--wait")
            prefix = "Naturalist Bird region "
            def live_group(stream, seed):
                reply = control("command", f"naturalist_fixture_bird_region {stream} {seed}")["chat"]
                found = next((json.loads(row.partition(prefix)[2]) for row in reply if prefix in row), None)
                assert found is not None, (stream, seed, reply)
                return found

            return [dict(live_group(row["stream"],row["seed"]), stream=row["stream"], seed=row["seed"])
                    for row in expected["groups"]]
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    expected=json.loads(args.reference.read_text())
    actual = capture(args.version, expected)
    print(json.dumps(actual, indent=2))
    assert len(actual) == len(expected["groups"])
    for got, want in zip(actual, expected["groups"], strict=True):
        assert (got["stream"], got["seed"], got["next"]) == (want["stream"], want["seed"], want["next"]), (got, want)
        assert len(got["members"]) == len(want["members"])
        for member, original in zip(got["members"], want["members"], strict=True):
            for field in ("variant", "age", "left_handed"):
                assert member[field] == original[field], (field, member, original)
            assert abs(member["follow_bonus"] - original["follow_bonus"]) < 1e-12, (member, original)

