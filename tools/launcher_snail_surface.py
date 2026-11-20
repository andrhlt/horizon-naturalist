"""Compare Snail attachment transitions in the built Horizon world and Java."""
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
            prefix = "Naturalist snail surface "
            return next(json.loads(line.split(prefix, 1)[1])
                        for line in control("command", "naturalist_fixture_snail_surface")["chat"]
                        if prefix in line)
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.reference.read_text())
    report = capture(args.version)
    print(json.dumps(report, indent=2))
    for key in ("wall_tick", "ceiling_tick", "attached"):
        assert report[key] == expected[key], (key, report, expected)
    for key in ("wall_y", "ceiling_y"):
        assert abs(report[key] - expected[key]) < 1e-6, (key, report, expected)
