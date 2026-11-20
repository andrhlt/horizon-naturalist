"""Replay the pinned Java variant selector with exact shared level RNG seeds."""
import argparse
import json
from pathlib import Path
import socket
import subprocess
import tempfile


def compare(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-variant-rng-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        path = home / "ctl.sock"

        def call(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", path, *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=75)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as stream:
                stream.settimeout(30)
                stream.connect(str(path))
                stream.sendall(f"command {line}\n".encode())
                reply = json.loads(stream.makefile().readline())
            return reply["chat"]

        try:
            call("launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--world", "Naturalist Variant RNG", "--creative",
                 "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 100 8.5")
            call("wait", 20)
            command("tick freeze")
            marker = "Naturalist variant RNG "
            comparisons = []
            for biome, reports in reference.items():
                command(f"fillbiome 4 96 4 11 103 11 minecraft:{biome}")
                for seed, expected in reports.items():
                    lines = command(f"naturalist_fixture_variant_rng {seed}")
                    actual = next(json.loads(line.split(marker, 1)[1]) for line in lines if marker in line)
                    errors = [(i, pair, got) for i, (pair, got) in enumerate(
                        zip(expected["choices"], actual["choices"], strict=True)) if pair != got]
                    comparisons.append({"biome": biome, "seed": int(seed), "draws": len(actual["choices"]),
                                        "differences": errors,
                                        "actual_biome": actual["biome"],
                                        "sentinel": [expected["sentinel"], actual["sentinel"]]})
            return comparisons
        finally:
            if path.exists():
                call("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    comparisons = compare(args.version, json.loads(args.reference.read_text()))
    print(json.dumps(comparisons, indent=2))
    assert all(not row["differences"] and row["sentinel"][0] == row["sentinel"][1]
               and row["actual_biome"] == f"minecraft:{row['biome']}" for row in comparisons)
