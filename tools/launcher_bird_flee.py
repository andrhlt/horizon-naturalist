"""Measure live Bird trajectories near a local survival player in Horizon."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, approach):
    with tempfile.TemporaryDirectory(prefix="nat-bird-flee-", ignore_cleanup_errors=True) as directory:
        world = Path(directory)
        socket = world / "flee.sock"

        def ctl(*args):
            completed = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                       capture_output=True, text=True, check=True, timeout=75)
            return json.loads(completed.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def sample(trial):
            marker = "following entity data: "
            raw = next(row.split(marker, 1)[-1] for row in cmd(f"data get entity @e[tag=approach_bird_{trial},limit=1] Pos")
                       if marker in row)
            return [float(value.strip().rstrip("d")) for value in raw[1:-1].split(",")]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", world, "--game-dir", world,
                "--world", "Bird Approach Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("time set noon")
            cmd("tp @s 12.5 100 16.5 90 0")
            ctl("wait", 20)
            cmd("tick freeze")
            cmd("fill 0 99 0 31 99 31 minecraft:stone")
            cmd("fill 0 100 0 31 108 31 minecraft:air")
            cmd("tp @s 12.5 100 16.5 90 0")
            cmd(f"gamemode {'survival' if approach else 'creative'} @s")
            out = []
            for trial in range(12):
                cmd("tp @s 12.5 100 16.5 90 0")
                cmd("kill @e[tag=approach_bird]")
                cmd(f"summon naturalist:bird 16.5 100 16.5 {{Tags:[approach_bird,approach_bird_{trial}],OnGround:1b}}")
                cmd("tick step 6")
                ctl("wait", 8)
                first = sample(trial)
                cmd("tick step 18")
                ctl("wait", 20)
                out.append([first, sample(trial)])
            return out
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--creative-control", action="store_true")
    args = parser.parse_args()
    print(json.dumps(capture(args.version, not args.creative_control), indent=2))
