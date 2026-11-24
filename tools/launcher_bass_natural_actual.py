"""Observe naturally spawned Bass and their initialized variants in a real water fixture."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


def capture(version: Path):
    resources = Path(os.environ.get("NATURALIST_RESOURCES", version / ".local/horizon-client/26.3"))
    with tempfile.TemporaryDirectory(prefix="nat-bass-natural-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "bass.sock"
        jar = Path(os.environ.get(
            "NATURALIST_JAR",
            Path.home() / "Library/Application Support/horizon/jars/"
            "b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd.jar",
        ))
        (home / "jars").mkdir()
        shutil.copy2(jar, home / "jars" / jar.name)

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    check=False, capture_output=True, text=True, timeout=75)
            if result.returncode:
                raise RuntimeError(f"horizon-ctl failed ({result.returncode}): {result.stderr}\n{result.stdout}")
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                resources, "--home", home, "--game-dir", home, "--world", "Bass Natural Spawn",
                "--seed", 17, "--creative", "--test-terrain", "--render-distance", 5, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("difficulty normal")
            command("tp @s 8.5 97 0.5")
            ctl("wait-ready")
            command("tick freeze")
            for offset in (0, 16, 32):
                x = 32 + offset
                command(f"fill {x} 89 -16 {x + 15} 89 15 minecraft:stone")
                command(f"fill {x} 90 -16 {x + 15} 104 15 minecraft:water")
                command(f"fillbiome {x} 90 -16 {x + 15} 104 15 minecraft:swamp")
            bundles = list((home / ".horizon-cache/mod-content").glob("*/worldgen/biomes.json"))
            assert len(bundles) == 1, bundles
            loaded = json.loads(bundles[0].read_text())

            def bass_rows(biome):
                document = json.loads(loaded[biome])
                spawns = document["attributes"]["minecraft:gameplay/natural_mob_spawns"]
                spawns = spawns.get("argument", spawns)
                return [
                    [category, entry["type"], entry["weight"], entry["count"]]
                    for category, entries in spawns["spawns_by_category"].items()
                    for entry in entries if entry["type"] == "naturalist:bass"
                ]
            loaded_rows = {biome: bass_rows(biome) for biome in ("minecraft:swamp", "minecraft:river")}
            print(json.dumps({"loaded_bass_rows": loaded_rows}, indent=2), flush=True)
            command("gamerule minecraft:spawn_mobs true")

            variants = set()
            groups = []
            for _ in range(36):
                command("kill @e[type=naturalist:bass]")
                command("tick sprint 800")
                ctl("wait", 3)
                fish = [row for row in ctl("entities", 96, "naturalist:bass")["entities"]
                        if 32 <= row["pos"][0] <= 80 and 89 <= row["pos"][1] <= 105]
                if fish:
                    groups.append(len(fish))
                    row = command("data get entity @e[type=naturalist:bass,limit=1] Variant")
                    variants.update(line.rsplit(" ", 1)[-1].strip('"') for line in row
                                    if "following entity data:" in line)
                    if len(variants) == 3:
                        break
            result = {"loaded_bass_rows": loaded_rows, "observed_groups": groups,
                      "initialized_variants": sorted(variants)}
            print(json.dumps(result, indent=2), flush=True)
            assert any(loaded_rows.values()), result
            assert result["observed_groups"], result
            assert set(result["initialized_variants"]).issubset({
                "naturalist:bass", "naturalist:bass_medium", "naturalist:bass_large"}), result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    capture(Path(os.environ.get("NATURALIST_VERSION", os.sys.argv[1])))
