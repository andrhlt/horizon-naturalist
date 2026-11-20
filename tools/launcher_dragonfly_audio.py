"""Check Dragonfly loop ownership in the headless sound engine in the headless Horizon launcher build."""
import argparse
import json
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
                "--world", "Dragonfly Audio Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 8.5")
            command("time set midnight")
            command("tick freeze")

            def instances():
                return ctl("status")["sounds"]["instances"]

            before = instances()
            measured = []

            command("summon naturalist:dragonfly 8.5 105 8.5 {Tags:[buzz1],NoAI:1b}")
            step(4)
            measured.append(instances() - before)
            step(20)
            measured.append(instances() - before)
            command("summon naturalist:dragonfly 9.5 105 8.5 {Tags:[buzz2],NoAI:1b}")
            step(4)
            measured.append(instances() - before)
            command("data merge entity @e[tag=buzz2,limit=1] {Silent:1b}")
            step(4)
            measured.append(instances() - before)
            command("data merge entity @e[tag=buzz2,limit=1] {Silent:0b}")
            step(4)
            measured.append(instances() - before)
            command("summon naturalist:dragonfly 9.5 105 8.5 {Tags:[silent_buzz],NoAI:1b,Silent:1b}")
            step(4)
            measured.append(instances() - before)
            command("data merge entity @e[tag=silent_buzz,limit=1] {Silent:0b}")
            step(4)
            measured.append(instances() - before)
            command("kill @e[tag=buzz1]")
            step(30)
            measured.append(instances() - before)
            print(json.dumps({"instance_deltas": measured}), flush=True)
            assert measured == [1, 1, 2, 1, 1, 1, 1, 0], measured
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
