"""Read actual world-generated Birds in a fresh, seeded headless Horizon region."""
import argparse
from collections import Counter
import json
from pathlib import Path
import subprocess
import tempfile
import time

from bird_generation_pack import write_pack


def capture(version, seed, anchor, reopen, fixture_pack):
    with tempfile.TemporaryDirectory(prefix="nat-bird-worldgen-", ignore_cleanup_errors=True) as scratch:
        work = Path(scratch)
        socket = work / "bird-worldgen.sock"

        def ctl(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=85,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(command):
            return ctl("command", command)["chat"]

        def field(query):
            return [row.split("following entity data: ", 1)[-1].strip('"')
                    for row in cmd(query) if "following entity data:" in row]

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", work, "--game-dir", work,
                "--world", "Natural Bird chunk generation", "--seed", seed, "--creative",
                "--render-distance", 6, "--wait")

        def close_save():
            ctl("quit")
            deadline = time.monotonic() + 15
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.1)
            assert not socket.exists(), "Horizon was still running after quit"

        x, z = anchor
        selector = f"execute positioned {x} 100 {z} run execute as @e[type=naturalist:bird,distance=..115] at @s"
        taiga_selector = selector + " if biome ~ ~ ~ minecraft:taiga run data get entity @s "

        def sample():
            return {
                "all_variants": dict(Counter(field(selector + " run data get entity @s Variant"))),
                "taiga_variants": dict(Counter(field(taiga_selector + "Variant"))),
                "taiga_uuids": sorted(field(taiga_selector + "UUID")),
            }

        def wait_for_birds():
            ctl("wait-ready")
            deadline = time.monotonic() + 45
            previous = None
            stable = 0
            while time.monotonic() < deadline:
                observed = sample()
                stable = stable + 1 if observed == previous else 0
                previous = observed
                if observed["taiga_uuids"] and stable >= 3:
                    return observed
                time.sleep(1)
            return observed

        try:
            launch()
            if fixture_pack:
                close_save()
                (save,) = work.rglob("level.dat")
                write_pack(save.parent / "datapacks")
                launch()
                enabled = cmd("datapack list enabled")
                assert any("file/bird_gen_fixture" in row for row in enabled), enabled
            cmd("gamemode spectator @s")
            cmd("gamerule minecraft:spawn_mobs true")
            cmd("gamerule minecraft:advance_time false")
            cmd("tick freeze")
            print("forceload", cmd("forceload add 4160 3936 4287 4063"), file=__import__("sys").stderr, flush=True)
            ctl("wait-ready")
            time.sleep(20)
            before = cmd("time query gametime")
            print("loaded", cmd("forceload query"), file=__import__("sys").stderr, flush=True)
            loaded_checks = [cmd(f"execute if loaded {cx*16} 80 {cz*16} run say chunk_loaded") for cx in range(260,268) for cz in range(246,254)]
            assert sum(any("chunk_loaded" in row for row in rows) for rows in loaded_checks) == 64, loaded_checks
            cmd("summon minecraft:sheep 4224 128 4000 {NoAI:1b,Tags:[density_control]}")
            assert field("data get entity @e[tag=density_control,limit=1] Pos"), "missing positive sheep control"
            result = {}
            for kind in ("naturalist:bird", "minecraft:pig", "minecraft:sheep", "minecraft:cow"):
                q = f"execute as @e[type={kind},x=4160,y=-64,z=3936,dx=127,dy=512,dz=127] run data get entity @s Pos"
                result[kind] = field(q)
            assert before == cmd("time query gametime")
            return result
        finally:
            if socket.exists():
                close_save()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("x", type=int)
    parser.add_argument("z", type=int)
    parser.add_argument("--seed", type=int, default=49903)
    parser.add_argument("--reference", type=Path)
    args = parser.parse_args()
    assert (args.x, args.z) == (4224, 4000), "fixture uses the fixed 8x8 reference rectangle"
    output = capture(args.version, args.seed, (args.x, args.z), False, False)
    print(json.dumps(output, indent=2), flush=True)
    if args.reference:
        reference = json.loads(args.reference.read_text())
        for kind in ("naturalist:bird", "minecraft:pig", "minecraft:sheep", "minecraft:cow"):
            assert sorted(output[kind]) == sorted(reference[kind]), (kind, output[kind], reference[kind])
