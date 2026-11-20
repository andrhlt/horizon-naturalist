"""Observe day and night Firefly survival in the installed build."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-wings-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "wings.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=100)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def data(target, key):
            rows = cmd(f"data get entity {target} {key}")
            return next((row.split("following entity data: ", 1)[1] for row in rows
                         if "following entity data: " in row), None)

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Insect Wings Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 110 8.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 109 15 minecraft:air")
            cmd("time set midnight")
            cmd("tick freeze")
            cmd("summon naturalist:firefly 10.5 102 10.5 {Tags:[sun_test],NoAI:1b,NoGravity:1b,PersistenceRequired:1b}")
            cmd('summon naturalist:firefly 12.5 102 12.5 {Tags:[named_test],NoAI:1b,NoGravity:1b,'
                'PersistenceRequired:1b,CustomName:"Beacon"}')
            unmarked = "@e[tag=sun_test,limit=1]"
            named = "@e[tag=named_test,limit=1]"
            def exists(target):
                return data(target, "UUID") is not None

            step(45)
            survives_night = exists(unmarked)
            cmd("time set noon")
            step(590)
            survives_before_limit = exists(unmarked)
            step(25)
            removed_in_sun = not exists(unmarked)
            named_survives = exists(named)
            output = {
                "survives_night": survives_night,
                "survives_before_limit": survives_before_limit,
                "removed_in_sun": removed_in_sun,
                "named_survives": named_survives,
            }
            print(json.dumps(output, indent=2), flush=True)
            assert all(output.values()), output
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
