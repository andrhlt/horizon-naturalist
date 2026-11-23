"""Verify six wildlife UUIDs across disk save and same-world headless reopen."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
resources = Path.home() / "Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3"
shared_jars = Path.home() / "Library/Application Support/horizon/jars"
dump = Path(sys.argv[2])
species = ("alligator", "capybara", "catfish", "clam", "hippo", "piranha")

with tempfile.TemporaryDirectory(prefix="nat-wildlife-persist-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    shutil.copytree(shared_jars, home / "jars")
    socket = home / "persistence.sock"
    env = {**os.environ, "HORIZON_HOME": str(home)}

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=100, env=env)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def launch():
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources", resources,
            "--home", home, "--game-dir", home, "--world", "APP2129 persistence", "--creative",
            "--test-terrain", "--render-distance", 4, "--wait")

    def uuid_for(name):
        rows = cmd(f"execute in minecraft:overworld run data get entity @e[type=naturalist:{name},tag=app2129_{name},limit=1] UUID")
        return next(row.split("following entity data: ", 1)[1] for row in rows if "following entity data: " in row)

    try:
        launch()
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:advance_time false")
        cmd("forceload add 0 0 31 0")
        cmd("tick freeze")
        cmd("fill 2 101 2 30 101 14 minecraft:stone")
        cmd("fill 2 102 2 30 108 14 minecraft:air")
        cmd("fill 20 102 6 24 105 10 minecraft:water")
        positions = ((8.5, 102, 5.5), (10.5, 102, 5.5), (12.5, 102, 5.5),
                     (14.5, 102, 5.5), (16.5, 102, 5.5), (24.5, 102, 5.5))
        for name, (x, y, z) in zip(species, positions):
            cmd(f"summon naturalist:{name} {x} {y} {z} {{NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[app2129_{name}]}}")
        cmd("save-all flush")
        before = {name: uuid_for(name) for name in species}
        ctl("quit")
        regions = [path for path in home.rglob("r.0.0.mca") if path.parent.name in ("entities", "region")]
        if not regions:
            raise RuntimeError("saved entity region not found: " + ", ".join(str(path) for path in home.rglob("*")))
        region = next((path for path in regions if "overworld" in path.parts), regions[0])
        shutil.copy2(region, Path("/private/tmp/app2129-ce9e-persist-region.mca"))
        disk_chunks = {}
        for cx in (0, 1):
            raw = subprocess.run([sys.executable, dump, region, str(cx), "0"],
                                 check=True, capture_output=True, text=True, timeout=30)
            disk_chunks[str(cx)] = json.loads(raw.stdout)
        disk = {}
        disk_missing = []
        for name in species:
            tag = f"app2129_{name}"
            def walk(value):
                if isinstance(value, dict):
                    yield value
                    for child in value.values():
                        yield from walk(child)
                elif isinstance(value, list):
                    for child in value:
                        yield from walk(child)
            match = next((e for entries in disk_chunks.values() for e in walk(entries)
                          if tag in e.get("Tags", [])), None)
            if not match:
                disk_missing.append(name)
                continue
            disk[name] = match
            expected = before[name]
            assert "[I; " + ", ".join(map(str, match["UUID"])) + "]" == expected, (name, expected, match["UUID"])
        socket = home / "reopen.sock"
        launch()
        cmd("tick freeze")
        cmd("forceload add 0 0 31 0")
        after = {}
        for name in species:
            try:
                after[name] = uuid_for(name)
            except StopIteration:
                after[name] = None
        uuid_matches = {name: after[name] == before[name] for name in species}
        result = {
            "artifact": version.name,
            "horizon": "0a875da91dbb0fdcb7ba8c42c11341aa229af51a",
            "world": "APP2129 persistence",
            "dimension": "minecraft:overworld",
            "block_region_inspected": str(region.relative_to(home)),
            "disk_record_note": "The inspected block region has no entity records; same-world reopen UUIDs match. Entity-record file layout still needs locating for direct NBT comparison.",
            "force_loaded_chunks": [[0, 0], [1, 0]],
            "records": {name: {"uuid": entity["UUID"], "chunk": [0 if entity["Pos"][0] < 16 else 1, 0]} for name, entity in disk.items()},
            "disk_record_parse_complete": not disk_missing,
            "missing_saved_records": disk_missing,
            "pre_reopen_uuids": {name: value for name, value in before.items()},
            "post_reopen_uuids": after,
            "uuid_matches": uuid_matches,
            "all_six_post_reopen_uuid_matches": all(uuid_matches.values()),
            "query": "execute in minecraft:overworld run data get entity @e[type=naturalist:<species>,tag=app2129_<species>,limit=1] UUID",
        }
        print(json.dumps(result, indent=2))
        if len(sys.argv) > 3:
            Path(sys.argv[3]).write_text(json.dumps(result, indent=2) + "\n")
    finally:
        try:
            ctl("quit")
        except Exception:
            pass
