"""Observe juvenile Bird flock movement in the installed Horizon build."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-bird-flock-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "flock.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=65)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def read_x():
            rows = cmd("data get entity @e[tag=juvenile,limit=1] Pos[0]")
            return next(float(n.group(1)) for row in rows
                        if (n := re.search(r"following entity data: ([0-9.\-]+)d?$", row)))

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Bird Flock Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 110 8.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 109 15 minecraft:air")
            cmd("tick freeze")
            cmd("summon naturalist:bird 2.5 100 8.5 {NoAI:1b,PersistenceRequired:1b,Tags:[leader]}")
            cmd("summon naturalist:bird 11.5 100 8.5 {NoAI:1b,PersistenceRequired:1b,Tags:[near]}")
            cmd("summon naturalist:bird 10.5 100 8.5 {Age:-24000,PersistenceRequired:1b,Tags:[juvenile]}")
            positions = [read_x()]
            for _ in range(8):
                step(5)
                positions.append(read_x())
            result = {"juvenile_selects_first_adult": min(positions) < positions[0] - 0.5}
            print(json.dumps({**result, "juvenile_x": positions}, indent=2), flush=True)
            assert result["juvenile_selects_first_adult"] == reference["juvenile_selects_first_adult"], positions
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.reference.read_text()))
