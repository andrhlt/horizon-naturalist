"""Check synchronized Vulture perch metadata and animation while cactus perching in the headless Horizon launcher build."""
import argparse
import json
import subprocess
from socket import AF_UNIX, SOCK_STREAM, socket as connect_socket
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
            def pose_snapshot():
                # A private, headless-only mod probe on the existing control channel.
                with connect_socket(AF_UNIX, SOCK_STREAM) as connection:
                    connection.settimeout(10)
                    connection.connect(str(socket))
                    connection.sendall(b"naturalist_vulture_perch\n")
                    with connection.makefile() as reply:
                        return json.loads(reply.readline())

            perched = None
            for _ in range(24):
                step(40)
                trace = pose_snapshot()
                assert trace["client_proxy"], trace
                perched = next(
                    (row for row in trace["vultures"]
                     if tuple(row["position"]) in perch_positions and row["perched"]
                     and row["weights"] and row["weights"][0] > 0.5),
                    None,
                )
                if perched:
                    break
            assert perched, "no tracked Vulture entered its sit animation"

            x, y, z = perched["position"]
            command(f"execute positioned {x} {y} {z} run damage "
                    "@e[type=naturalist:vulture,distance=..0.25,limit=1] 1 minecraft:generic")
            step(10)
            fled = next(row for row in pose_snapshot()["vultures"] if row["id"] == perched["id"])
            print(json.dumps({"perched": perched, "after_hit": fled}, indent=2), flush=True)
            assert not fled["perched"] and fled["weights"][1] > 0.5, fled
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
