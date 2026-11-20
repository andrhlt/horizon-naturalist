"""Check a baby Caterpillar climbing a stone wall in the headless Horizon launcher build."""
import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-bird-flap-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "bird-flap.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def step(ticks):
            command(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Bird Flap Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 8.5")
            command("time set midnight")
            command("tick freeze")

            command("fill 8 100 3 8 106 13 minecraft:stone")
            command("summon naturalist:caterpillar 7.6 100 8.5 {Tags:[crawler],Age:-24000,Motion:[0.4d,0d,0d]}")
            target = "@e[tag=crawler,limit=1]"
            for _ in range(8):
                command(f"data modify entity {target} Motion[0] set value 0.4d")
                step(1)
            [line] = command(f"data get entity {target} Pos")
            x, y, _ = (float(value) for value in re.findall(r"(-?\d+(?:\.\d+)?)d", line))
            actual = {"baby_width": round(2.0 * (8.0 - x), 6), "rise": round(y - 100.0, 6)}
            print(json.dumps(actual), flush=True)
            assert abs(actual["baby_width"] - 0.4) < 1e-5 and abs(actual["rise"] - 0.3528) < 1e-4, actual
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
