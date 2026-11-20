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
                "taiga_saved_ages": field(taiga_selector + "Age"),
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
            cmd("time set noon")
            cmd("tick freeze")
            cmd(f"tp @s {x} 128 {z}")
            game_time = cmd("time query gametime")
            assert any("game time" in row for row in game_time), game_time
            result = {"seed": seed, "taiga_anchor": anchor, **wait_for_birds()}
            assert cmd("time query gametime") == game_time, "world ticked during chunk generation capture"
            if reopen:
                cmd("gamerule minecraft:spawn_mobs false")
                close_save()
                launch()
                cmd("gamemode spectator @s")
                cmd("tick freeze")
                result["reopened"] = wait_for_birds()
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("x", type=int)
    parser.add_argument("z", type=int)
    parser.add_argument("--seed", type=int, default=49903)
    parser.add_argument("--reopen", action="store_true")
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--fixture-pack", action="store_true")
    args = parser.parse_args()
    output = capture(args.version, args.seed, (args.x, args.z), args.reopen, args.fixture_pack)
    print(json.dumps(output, indent=2), flush=True)
    assert not output["taiga_saved_ages"], output
    if args.reference:
        reference = json.loads(args.reference.read_text())
        assert args.seed == reference["seed"] and [args.x, args.z] == reference["taiga_anchor"]
        assert output["taiga_uuids"] and set(output["taiga_variants"]) == set(reference["variants"]), output
    if args.reopen:
        saved = output["reopened"]
        assert not saved["taiga_saved_ages"], saved
        assert saved["taiga_uuids"] == output["taiga_uuids"] and saved["taiga_uuids"], output
        assert saved["taiga_variants"] == output["taiga_variants"], output
