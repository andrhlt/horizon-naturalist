"""Check wingbeat and footstep audio for insects during real movement in the headless Horizon launcher build."""
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
                "--world", "Insect Audio Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 8.5")
            command("time set midnight")
            command("tick freeze")

            result = {}
            for name in ("butterfly", "dragonfly", "firefly", "vulture"):
                result[name] = {}
                for scenario, y, motion in (("air", 110, "0.7d,-0.2d,0.0d"), ("ground", 100, "0.8d,-0.02d,0.0d")):
                    before = ctl("status")["heard"]
                    command(f"summon naturalist:{name} 5.5 {y} 8.5 {{Tags:[sound_probe],NoAI:0b,Motion:[{motion}]}}")
                    step(15)
                    after = ctl("status")["heard"]
                    result[name][scenario] = {key: count - before.get(key, 0) for key, count in after.items()
                                              if key.startswith("minecraft:entity.parrot.") and count > before.get(key, 0)}
                    command("kill @e[tag=sound_probe]")
                    step(20)
            print(json.dumps(result, indent=2), flush=True)
            assert all(not heard for checks in result.values() for heard in checks.values()), result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
