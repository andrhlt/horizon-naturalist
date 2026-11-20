"""Compare naturally decorated Horizon Ant Hill cells with Java on a shared seed."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-ant-generation-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "control.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=90)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Natural Ant Hills", "--seed", reference["seed"], "--creative",
                "--render-distance", 7, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            x, z = reference["center"]
            cmd(f"tp @s {x + 0.5} 95 {z + 0.5}")
            ctl("wait", 80)
            scan = ctl("find-block", "naturalist:ant_hill", 64)
            print(json.dumps({"scan": scan}), flush=True)
            assert scan["count"] == len(reference["blocks"]), scan

            results = []
            for x, y, z, open_, workers, queen, owner in reference["blocks"]:
                props = ",".join(f"{key}={str(value).lower()}" for key, value in
                                 (("open", open_), ("workers", workers), ("has_queen", queen)))
                block = "naturalist:ant_hill"
                result = cmd(f"execute if block {x} {y} {z} {block}[{props}]")
                data = cmd(f"data get block {x} {y} {z}")
                results.append({
                    "pos": [x, y, z],
                    "state_exact": any("Test passed" in row for row in result),
                    "block_entity": any("following block data" in row and block in row for row in data),
                    "owner_exact": any("Owner:" in row for row in data) == owner,
                })
            return results
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    results = capture(args.version, expected)
    print(json.dumps(results, indent=2))
    failures = [row for row in results if not all(row[field] for field in ("state_exact", "block_entity", "owner_exact"))]
    assert not failures, failures
