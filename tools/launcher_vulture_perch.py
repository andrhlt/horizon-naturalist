"""Check Vulture cactus-top snapping in the headless Horizon launcher build."""
import argparse
import json
import subprocess
import tempfile
from pathlib import Path


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-vulture-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "vulture.sock"

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
                "--world", "Vulture Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:natural_health_regeneration false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 8.5")
            command("time set midnight")
            command("tick freeze")

            command("fill 0 98 0 15 98 15 minecraft:stone")
            command("fill 0 99 0 15 99 15 minecraft:sand")
            command("fill 0 100 0 0 105 15 minecraft:glass")
            command("fill 15 100 0 15 105 15 minecraft:glass")
            command("fill 0 100 0 15 105 0 minecraft:glass")
            command("fill 0 100 15 15 105 15 minecraft:glass")
            command("fill 0 106 0 15 106 15 minecraft:glass")
            positions = [(x, z) for x in range(2, 15, 4) for z in range(2, 15, 4)]
            for x, z in positions:
                command(f"setblock {x} 100 {z} minecraft:cactus")
                command(f"setblock {x} 101 {z} minecraft:cactus[age=0]")
                command(f"summon naturalist:vulture {x + 1.0} 102.3 {z + .5} {{Tags:[perch_test],PersistenceRequired:1b}}")
            perch_positions = {(x + 0.5, 102, z + 0.5) for x, z in positions}
            found = []
            for _ in range(18):
                step(80)
                vultures = [row for row in ctl("entities", 24, "naturalist:vulture")["entities"] if row["health"] > 0]
                at = [tuple(row["pos"]) for row in vultures]
                found = [x for x in at if x in perch_positions]
                print("step", len(found), len(at), at[:3], flush=True)
                if found:
                    break
            print(json.dumps({"occupied_cactus_tops": found}), flush=True)
            assert found
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
