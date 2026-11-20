"""Compare the launcher-built deer's scale attribute and geometry against Java."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time


def replay(version, expected):
    with tempfile.TemporaryDirectory(prefix="nat-step-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "step.sock"

        def control(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    check=True, capture_output=True, text=True, timeout=45)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return control("command", line)["chat"]

        def heard():
            return control("status")["heard"]

        def launch():
            control("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                    "--world", "Naturalist Deer Scale", "--creative", "--test-terrain",
                    "--render-distance", 4, "--wait")

        try:
            launch()
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 5.5 101 5.5")
            control("wait", 25)
            command("fill 1 101 1 15 106 15 minecraft:air")
            command("fill 1 100 5 15 100 7 minecraft:stone")
            command("fill 1 100 8 15 100 12 minecraft:grass_block")
            command("tp @s 5.5 101 5.5")
            control("wait", 15)
            for name, age in (("adult", 0), ("fawn", -20000)):
                result = command(f"summon naturalist:deer 3.5 101 6.5 {{NoAI:1b,Age:{age},Tags:[scale_{name}]}}")
                assert any("Summoned" in row for row in result), result

            def geometry(baby):
                key = "naturalist_fixture_deer_scale "
                result = command(key.strip())
                shapes = next(json.loads(line.split(key, 1)[1]) for line in result if key in line)
                return next(shape for is_baby, shape in shapes if baby == is_baby)

            def set_scale(who, operation):
                result = command(f"attribute @e[tag=scale_{who},limit=1] minecraft:scale {operation}")
                print(result[-1:], flush=True)
                assert result, operation

            actual = [geometry(False)]
            set_scale("adult", "base set 2")
            actual.append(geometry(False))
            modifier = "modifier add naturalist:fixture_scale 0.5 add_multiplied_total"
            set_scale("adult", modifier)
            actual.append(geometry(False))
            set_scale("fawn", "base set 2")
            set_scale("fawn", modifier)
            actual.append(geometry(True))
            set_scale("fawn", "modifier remove naturalist:fixture_scale")
            set_scale("fawn", "base reset")
            actual.append(geometry(True))
            print(json.dumps(actual), flush=True)
            for shape, java_shape in zip(actual, expected, strict=True):
                assert all(abs(val - ref) < 1e-6 for val, ref in zip(shape, java_shape, strict=True)), (shape, java_shape)
            scale_tag = 'attributes[{id:"minecraft:scale"}]' 
            data = command(f"data get entity @e[tag=scale_adult,limit=1] {scale_tag}")
            assert any("fixture_scale" in row and "add_multiplied_total" in row for row in data), data
            changed = command("data merge entity @e[tag=scale_adult,limit=1] {NoAI:1b,Silent:1b}")
            assert any("Modified" in row for row in changed), changed
            assert geometry(False) == actual[2]
            control("quit")
            deadline = time.monotonic() + 10
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.1)
            assert not socket.exists()
            launch()
            control("wait", 25)
            assert geometry(False) == actual[2]
            assert geometry(True) == actual[4]
            data = command("attribute @e[tag=scale_adult,limit=1] minecraft:scale get")
            assert any("3.0" in row for row in data), data
            reset = command(f"data remove entity @e[tag=scale_adult,limit=1] {scale_tag}")
            assert any("Modified" in row for row in reset), reset
            assert geometry(False) == actual[2]
            print(json.dumps({"kept_on_nbt_update": True, "reopened_base_and_modifier": True,
                              "nbt_remove_retains_live_scale": True}), flush=True)
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    replay(args.version, json.loads(args.reference.read_text()))
