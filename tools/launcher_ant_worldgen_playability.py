"""Verify fresh natural Ant hills and their block entities survive a reopen."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import uuid

from launcher_managed import launch_managed, wait_managed
from prepare import JAR_SHA


# Seed-42 locations are within the ordinary has_ant_hill biome/rarity placement
# chain, taken from the pinned Java worldgen fixture. Only shape/state invariants
# are asserted; feature RNG parity is intentionally covered by another fixture.
HILLS = {
    "small": (-2772, 63, -1671, 10),
    "big": (-2870, 64, -1679, 15),
}


def capture(version: Path, jar: Path) -> dict:
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
        raise ValueError("expected the pinned Naturalist Java jar")
    with tempfile.TemporaryDirectory(prefix="nat-ant-worldgen-") as temp:
        home = Path(temp)
        socket = home / "ant-worldgen.sock"
        world_suffix = uuid.uuid4().hex[:8]
        os.environ.setdefault("NATURALIST_TEST_LAUNCHER_HOME", str(version.parent.parent))
        os.environ.setdefault("NATURALIST_TEST_INSTANCE", f"app-2124-ant-{world_suffix}")
        process = None

        def ctl(*args):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                capture_output=True, text=True, timeout=100,
            )
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def launch():
            nonlocal process
            process = launch_managed(
                version, socket, home, "Natural Ant Hill Playability", creative=True,
                render_distance=6, world_suffix=world_suffix, seed=42,
            )

        def close():
            nonlocal process
            ctl("quit")
            wait_managed(process)
            process = None

        def snapshot(anchor, expected_cells, require_pristine_queen=True):
            scan = ctl("find-block", "naturalist:ant_hill", 10)
            positions = [tuple(item["pos"]) for item in scan["nearest"]]
            assert scan["count"] == expected_cells, (anchor, scan)
            assert len(positions) == expected_cells, (anchor, scan)
            rows = []
            for pos in positions:
                px, py, pz = pos
                def matches(properties):
                    answer = cmd(f"execute if block {px} {py} {pz} naturalist:ant_hill[{properties}]")
                    return any("Test passed" in line for line in answer)

                state = cmd(f"data get block {px} {py} {pz}")
                storage = cmd(f"data get block {px} {py} {pz} Storage")
                block_entity = next((line.partition("following block data: ")[2]
                                     for line in state if "following block data: " in line), None)
                storage_text = next((line.partition("following block data: ")[2]
                                     for line in storage if "following block data: " in line), None)
                assert block_entity is not None and "naturalist:ant_hill" in block_entity, (pos, state)
                assert storage_text is not None, (pos, storage)
                open_ = matches("open=true")
                queen = matches("has_queen=true")
                workers = next((count for count in range(4) if matches(f"workers={count}")), None)
                assert workers is not None, (pos, "invalid workers property")
                rows.append({"pos": pos, "open": open_, "queen": queen,
                             "workers": workers, "storage": storage_text})
            entrances = [row for row in rows if row["open"]]
            queens = [row for row in entrances if row["queen"]]
            assert 4 <= len(entrances) <= 6, (anchor, rows)
            assert len(queens) == 1, (anchor, rows)
            if require_pristine_queen:
                assert queens[0]["workers"] == 0, queens[0]
            return sorted(rows, key=lambda row: row["pos"])

        try:
            launch()
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            results = {}
            for kind, (x, y, z, count) in HILLS.items():
                cmd(f"tp @s {x + 0.5} {y + 4} {z + 0.5}")
                ctl("wait-ready")
                ctl("wait", 80)
                # The scan center is the expected Java hill location. Moving it
                # here keeps unrelated, rarer hills outside the 10-block query.
                results[kind] = snapshot((x, y, z), count)

            # Exercise the generated hill's normal ownerless/wild-queen route:
            # an unowned Ant can enter an open unowned hill entrance.
            entrances = [row for row in results["small"] if row["open"] and row["workers"] < 3]
            entrance = entrances[0]
            ex, ey, ez = entrance["pos"]
            cmd(f"tp @s {ex + 0.5} {ey + 4} {ez + 0.5}")
            ctl("wait-ready")
            cmd(f"summon naturalist:ant {ex + 2.5} {ey + 1} {ez + 0.5} "
                "{Tags:[natural_hill_worker],HillCooldown:0}")
            increased = False
            for _ in range(40):
                ctl("wait", 20)
                scan = ctl("find-block", "naturalist:ant_hill", 10)
                positions = [tuple(item["pos"]) for item in scan["nearest"]]
                assert all(tuple(row["pos"]) in positions for row in entrances), (entrances, scan)
                increased = any(
                    any("Test passed" in line for line in cmd(
                        f"execute if block {row['pos'][0]} {row['pos'][1]} {row['pos'][2]} "
                        "naturalist:ant_hill[open=true,workers=" + str(row["workers"] + 1) + "]"))
                    for row in entrances
                )
                if increased:
                    break
            assert increased, "a wild Ant did not enter its naturally generated open hill"
            results["small"] = snapshot(HILLS["small"][:3], HILLS["small"][3], False)
            cmd("save-all flush")
            close()

            launch()
            reopened = {}
            for kind, (x, y, z, count) in HILLS.items():
                cmd(f"tp @s {x + 0.5} {y + 4} {z + 0.5}")
                ctl("wait-ready")
                reopened[kind] = snapshot((x, y, z), count, kind != "small")
            assert reopened == results, {"before": results, "after": reopened}
            return {"seed": 42, "fresh_world": True, "small_cells": 10, "big_cells": 15,
                    "queen_and_entrances": True, "wild_ant_entry": True,
                    "block_entity_storage": True,
                    "save_reopen_identical": True}
        finally:
            if socket.exists():
                try:
                    ctl("quit")
                finally:
                    wait_managed(process)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--jar", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(capture(args.version, args.jar), indent=2), flush=True)
