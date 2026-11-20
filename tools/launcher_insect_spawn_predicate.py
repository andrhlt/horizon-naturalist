"""Check registered insect spawn predicates in actual arena lighting in the headless Horizon launcher build."""
import argparse
import json
import subprocess
import tempfile
from pathlib import Path


def capture(version, expected):
    with tempfile.TemporaryDirectory(prefix="nat-bird-flap-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "bird-flap.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Insect Spawn Rules Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:advance_time false")
            command("difficulty peaceful")
            command("time set noon")
            command("tp @s 8.5 100 18.5")
            ctl("wait", 30)
            command("fill 0 100 0 15 105 15 minecraft:air")
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("setblock 2 99 2 minecraft:mud")
            command("fill -16 100 18 32 109 51 minecraft:air")
            command("fill -16 102 18 32 102 51 minecraft:stone")
            command("setblock 2 99 34 minecraft:grass_block")
            command("setblock 11 99 34 minecraft:stone")
            ctl("wait", 20)

            prefix = "Insect spawn fixture "
            lines = command("naturalist_fixture_insect_spawn")
            actual = next(json.loads(line.split(prefix, 1)[1]) for line in lines if prefix in line)
            print(json.dumps(actual, indent=2), flush=True)
            assert actual == expected, actual
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
