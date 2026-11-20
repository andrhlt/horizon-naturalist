"""Check the render-side Firefly glow cycle across the integrated server boundary in the headless Horizon launcher build."""
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
                "--world", "Firefly Glow Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 8.5")
            command("time set midnight")
            command("tick freeze")

            def client_frames():
                return {entity: (frame, lit) for entity, frame, lit in ctl("status")["naturalist_firefly_glow"]}

            for slot in range(20):
                x = 2.5 + slot % 5 * 2
                z = 2.5 + slot // 5 * 2
                command(f"summon naturalist:firefly {x} 101 {z} {{NoAI:1b,Tags:[glow_probe]}}")
            step(4)
            baseline = client_frames()
            assert len(baseline) == 20, baseline
            first = None
            for _ in range(30):
                step(2)
                first = next(((entity, frame) for entity, (frame, glowing) in client_frames().items()
                              if glowing and frame < 8), None)
                if first is not None:
                    break
            assert first is not None, "the render-side entities received no early glow phase"
            entity, initial_frame = first
            observed = [initial_frame]
            for _ in range(32):
                step(2)
                index, glowing = client_frames()[entity]
                if not glowing:
                    assert index == 0, index
                    break
                observed.append(index)
            else:
                raise AssertionError("glow did not stop")
            duration_upper_bound = initial_frame + 2 * len(observed)
            print(json.dumps({"initial_frame": initial_frame, "frames": observed,
                              "duration_upper_bound_ticks": duration_upper_bound}), flush=True)
            assert observed == sorted(observed) and observed[-1] == 29, observed
            assert 40 <= duration_upper_bound <= 60, duration_upper_bound
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
