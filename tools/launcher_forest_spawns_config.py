"""Compare the forest wildlife rows before and after a live native config change."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-forest-config-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "ctl.sock"
        config = home / "config/naturalist-server.toml"

        def control(*options):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, options)],
                                    check=True, capture_output=True, text=True, timeout=65)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return control("command", line)["chat"]

        def sample(biome):
            command(f"fillbiome 0 100 0 15 112 15 {biome}")
            control("wait", 5)
            prefix = "Naturalist forest fixture "
            rows = command("naturalist_fixture_forest_spawns")
            result = next(json.loads(row.split(prefix, 1)[1]) for row in rows if prefix in row)
            assert result["biome"] == biome and not result["canceled"], result
            return result["rows"]

        def expected(biome):
            return sorted(row[1:] for row in reference[biome]
                          if row[1] in ("minecraft:fox", "minecraft:rabbit"))

        forest, taiga = "minecraft:forest", "minecraft:taiga"
        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3", "--home", home,
                    "--game-dir", home, "--world", "Naturalist Forest Spawn Settings",
                    "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("tp @s 8.5 107 8.5")
            control("wait", 24)
            before = {biome: sample(biome) for biome in (forest, taiga)}
            assert before == {biome: expected(biome) for biome in before}, before
            config.write_text("[disable_mobs.land_mobs]\nforest_fox_removed = true\nforest_rabbit_removed = true\n")
            time.sleep(1.5)
            control("wait", 24)
            after = {biome: sample(biome) for biome in (forest, taiga)}
            expected_forest = expected(forest)
            expected_forest.remove(["minecraft:fox", 10, 1, 2])
            expected_forest.remove(["minecraft:rabbit", 10, 2, 3])
            assert after == {forest: expected_forest, taiga: before[taiga]}, after
            return {"default": before, "configured": after}
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    print(json.dumps(capture(args.version, json.loads(args.reference.read_text())), indent=2))
