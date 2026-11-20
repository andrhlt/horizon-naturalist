"""Observe Deer loaded by new terrain before the first ambient creature spawn pass."""
import argparse
import json
import re
from pathlib import Path
import subprocess
import tempfile
import time


def observe(version, seed):
    with tempfile.TemporaryDirectory(prefix="nat-deer-fresh-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "world.sock"

        def ctl(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  capture_output=True, check=True, text=True, timeout=100)
            return json.loads(proc.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist New Terrain", "--seed", seed, "--creative",
                "--render-distance", 6, "--wait")

        try:
            launch()
            freeze = cmd("tick freeze")
            assert any("froze" in line.lower() or "frozen" in line.lower() for line in freeze), freeze
            ctl("wait", 45)
            game_time = next(int(m[1]) for line in cmd("time query gametime")
                             if (m := re.search(r"The game time is (\d+)", line)))
            assert game_time < 400, (seed, game_time)
            location = cmd("locate biome minecraft:forest")
            print("forest location", location, flush=True)
            match = next((m for line in location if (m := re.search(r"\[(-?\d+),[^,]+,\s*(-?\d+)\]", line))), None)
            bases = [(0, 0)] if not match else [tuple(map(int, match.groups()))]
            deer = []
            for x, z in bases:
                for dx, dz in ((0, 0), (128, 0), (0, 128), (-128, 0), (0, -128), (256, 0), (0, 256)):
                    cmd(f"tp @s {x + dx} 120 {z + dz}")
                    ctl("wait-ready")
                    ctl("wait", 55)
                    fauna = ctl("entities", 128)["entities"]
                    deer = [mob for mob in fauna if mob["type"] == "naturalist:deer"]
                    print(json.dumps({"seed": seed, "game_time": game_time, "at": [x + dx, z + dz],
                                      "observed_deer": len(deer), "deer_positions": [mob["pos"] for mob in deer],
                                      "fauna": sorted({mob["type"] for mob in fauna})}), flush=True)
                    if deer:
                        break
                if deer:
                    break
            if deer:
                tagged = "@e[tag=deer_worldgen_fixture,limit=1]"
                output = cmd("tag @e[type=naturalist:deer,limit=1] add deer_worldgen_fixture")
                assert any("Added tag" in line for line in output), output

                def snapshot():
                    fields = ("UUID", "Variant", "attributes", "LeftHanded")
                    result = {}
                    for key in fields:
                        values = cmd(f"data get entity {tagged} {key}")
                        result[key] = next(line.partition(" following entity data: ")[2]
                                           for line in values if " following entity data: " in line)
                    return result

                original = snapshot()
                assert original["Variant"] == '"naturalist:deer"', original
                assert "minecraft:random_spawn_bonus" in original["attributes"], original
                cmd("save-all flush")
                ctl("quit")
                deadline = time.monotonic() + 10.0
                while socket.exists() and time.monotonic() < deadline:
                    time.sleep(0.1)
                launch()
                reopened = snapshot()
                assert reopened == original, (original, reopened)
                print(json.dumps({"worldgen_saved_fields": original, "retained_on_reopen": True}), flush=True)
                return True
            return False
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("seeds", type=int, nargs="+")
    args = parser.parse_args()
    assert any(observe(args.version, seed) for seed in args.seeds), "No generated Deer in sampled worlds"
