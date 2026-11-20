"""Check natural nighttime Firefly spawning in an open swamp in the headless Horizon launcher build."""
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
                "--world", "Ambient Spawn Fixture", "--creative", "--test-terrain", "--render-distance", 8, "--wait")
            def report():
                return {name: ctl("entities", 90, f"naturalist:{name}")["entities"]
                        for name in ("butterfly", "dragonfly", "firefly")}

            command("gamerule minecraft:spawn_mobs false")
            command("difficulty peaceful")
            command("gamerule minecraft:advance_time false")
            command("gamerule minecraft:spawn_wandering_traders false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 80)
            for x in range(-64, 80, 16):
                for z in range(-64, 80, 16):
                    command(f"fillbiome {x} 48 {z} {x + 15} 110 {z + 15} minecraft:swamp")
                    command(f"fill {x} 99 {z} {x + 15} 99 {z + 15} minecraft:grass_block")
            command("time set noon")
            command("gamerule minecraft:spawn_mobs true")
            ctl("wait", 25)
            phases = {}
            for phase in ("noon", "midnight"):
                command(f"time set {phase}")
                command("kill @e[type=naturalist:butterfly]")
                command("kill @e[type=naturalist:dragonfly]")
                command("kill @e[type=naturalist:firefly]")
                replies = command("tick sprint 480")
                print(phase, replies, flush=True)
                ctl("wait", 60)
                counts = {}
                for name, rows in report().items():
                    counts[name] = len([row for row in rows if row["health"] > 0])
                phases[phase] = counts
            print(json.dumps(phases, indent=2), flush=True)
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
