"""Check vertical damping for flying mobs in the headless Horizon launcher build."""
import argparse
import json
import re
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

        def step(ticks):
            command(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Wing Descent Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 8.5")
            command("time set midnight")
            command("tick freeze")

            result = {}
            for name in ("butterfly", "dragonfly", "firefly", "vulture", "bird"):
                command(f"summon naturalist:{name} 5.5 110 8.5 {{Tags:[wing_descent],NoGravity:1b,Motion:[0.0d,-0.2d,0.0d]}}")
                step(1)
                target = "@e[tag=wing_descent,limit=1]"
                [pos] = command(f"data get entity {target} Pos")
                [motion] = command(f"data get entity {target} Motion")
                y_pos, y_motion = (float(re.findall(r"(-?\d+(?:\.\d+)?)d", row)[1]) for row in [pos, motion])
                result[name] = {"rise": round(y_pos - 110, 9), "velocity": round(y_motion, 9)}
                command(f"kill {target}")
                step(25)
            print(json.dumps(result, indent=2), flush=True)
            assert result.keys() == expected.keys(), result
            for species, observed in result.items():
                assert all(abs(observed[field] - target) < 1e-7 for field, target in expected[species].items()), result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
