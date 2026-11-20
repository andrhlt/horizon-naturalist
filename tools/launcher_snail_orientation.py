"""Compare Snail rendering orientation in the built Horizon world and Java."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-snail-surface-", ignore_cleanup_errors=True) as folder:
        game_dir = Path(folder)
        socket = game_dir / "snail.sock"

        def control(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3",
                    "--home", game_dir, "--game-dir", game_dir, "--world", "Snail Surface Fixture",
                    "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            for command in (
                "gamerule minecraft:spawn_mobs false",
                "tp @s 8.5 107 8.5",
                "fill 0 93 0 15 101 15 minecraft:air",
                "fill 0 92 0 15 92 15 minecraft:stone",
                "fill 8 93 3 8 97 13 minecraft:stone",
                "fill 2 97 3 7 97 13 minecraft:stone",
                "tick freeze",
            ):
                control("command", command)
            reply = control("command", "naturalist_fixture_snail_orientation")
            assert any("wrote 33 frames" in row for row in reply["chat"]), reply
            log = socket.with_suffix(".log")
            if not log.exists():
                log = next(game_dir.glob("*.log"))
            return [json.loads(row.split("naturalist_snail_orientation ", 1)[1])
                    for row in log.read_text().splitlines() if "naturalist_snail_orientation " in row]
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.reference.read_text())["poses"]
    actual = capture(args.version)
    assert len(actual) == len(expected) == 33, (len(actual), len(expected))
    worst = 0.0
    first_mismatches = []
    for before, after in zip(expected, actual):
        assert before["tick"] == after["tick"] and abs(before["partial"] - after["partial"]) < 1e-6
        for key in ("normal", "forward", "anchor", "tail_lag"):
            b, a = before[key], after[key]
            error = (max(abs(x - y) for x, y in zip(b, a)) if isinstance(b, list) else abs(b - a))
            worst = max(worst, error)
            if error > 2e-5:
                first_mismatches.append((before["tick"], before["partial"], key, b, a))
    print(json.dumps({"frames": len(actual), "max_error": worst, "mismatches": len(first_mismatches),
                      "examples": first_mismatches[:8]}, indent=2), flush=True)
    assert not first_mismatches
