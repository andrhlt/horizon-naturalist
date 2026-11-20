"""Compare Giraffe terrain articulation with the pinned Java terrain solver."""
import argparse
import json
import math
from pathlib import Path
import socket
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-giraffe-terrain-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        path = home / "ctl.sock"

        def call(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", path, *map(str, args)],
                                    capture_output=True, text=True, check=True, timeout=75)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as stream:
                stream.settimeout(30)
                stream.connect(str(path))
                stream.sendall(f"command {line}\n".encode())
                return json.loads(stream.makefile().readline())["chat"]

        try:
            call("launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--world", "Naturalist Giraffe Terrain", "--creative",
                 "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 103 8.5")
            call("wait", 20)
            command("tick freeze")
            command("fill 6 98 6 10 98 10 minecraft:stone")
            command("fill 6 99 6 10 100 10 minecraft:air")
            command("setblock 8 99 8 minecraft:stone")
            command("setblock 7 99 8 minecraft:stone_slab[type=bottom]")
            command("setblock 7 99 7 minecraft:stone")
            marker = "Naturalist Giraffe terrain "
            return next(json.loads(row.split(marker, 1)[1])
                        for row in command("naturalist_fixture_giraffe_terrain") if marker in row)
        finally:
            if path.exists():
                call("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    actual = capture(args.version)
    worst = 0
    for want, got in zip(expected, actual, strict=True):
        for key in ("age", "yaw", "tick"):
            assert want[key] == got[key], (key, want, got)
        for key in ("feet", "body"):
            for a, b in zip(want[key], got[key], strict=True):
                worst = max(worst, abs(a - b))
                assert math.isclose(a, b, abs_tol=2e-6), (key, want, got)
    print(json.dumps({"pose_samples": len(actual), "maximum_error": worst}, indent=2))
