"""Compare actual egg-breeding parent ages and Tortoise-Snail rejection."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time

def capture(cmd, step):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:advance_time false")
    cmd("gamerule minecraft:mob_drops false")
    cmd("forceload add 0 0 31 31")
    cmd("time set 15000")
    cmd("tick freeze")
    for x in range(0, 32, 8):
        for z in range(0, 32, 8):
            cmd(f"fill {x} 99 {z} {x+7} 99 {z+7} minecraft:stone")
            cmd(f"fill {x} 100 {z} {x+7} 105 {z+7} minecraft:air")

    def data(label, field):
        marker = "following entity data: "
        return next((row.split(marker, 1)[1] for row in cmd(
            f"data get entity @e[tag={label},limit=1] {field}") if marker in row), None)

    cases = [
        ("alligator_pair", "naturalist:alligator", "naturalist:alligator"),
        ("tortoise_pair", "naturalist:tortoise", "naturalist:tortoise"),
        ("snail_pair", "naturalist:snail", "naturalist:snail"),
        ("tortoise_snail", "naturalist:tortoise", "naturalist:snail"),
        ("snail_tortoise", "naturalist:snail", "naturalist:tortoise"),
    ]
    result = {}
    cmd("time set 1000")
    for name, first, second in cases:
        cmd("kill @e[tag=social_probe]")
        cmd("kill @e[type=naturalist:snail]")
        cmd("kill @e[type=naturalist:alligator]")
        cmd("kill @e[type=naturalist:tortoise]")
        step(25)
        for x, kind, tag in [(6.5, first, "left"), (7.5, second, "right")]:
            owner = ",Owner:[I;22,33,44,55]" if kind == "naturalist:tortoise" else ""
            rows = cmd(f"summon {kind} {x} 100 8.5 "
                       f"{{Age:0,InLove:600,OnGround:1b,Tags:[social_probe,{tag}]{owner}}}")
            assert data(tag, "Pos"), rows
        step(90)
        state = {
            "parent_love": [int(data(tag, "InLove") or 0) for tag in ("left", "right")],
            "has_egg": [data(tag, "HasEgg") for tag in ("left", "right")],
            "parent_age": [int(data(tag, "Age") or 0) for tag in ("left", "right")],
            "newborn": [
                kind for kind in ("naturalist:tortoise", "naturalist:snail", "naturalist:alligator")
                if any(" following entity data: " in row for row in cmd(
                    f"data get entity @e[type={kind},tag=!social_probe,sort=nearest,limit=1] Age"))
            ],
        }
        result[name] = state
        print(name, state, flush=True)
    return result


def capture_horizon(version):
    with tempfile.TemporaryDirectory(prefix="nat-social-species-", ignore_cleanup_errors=True) as directory:
        folder = Path(directory)
        socket = folder / "control.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   text=True, check=True, capture_output=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", folder, "--game-dir", folder,
                "--world", "Naturalist Social Species", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("tp @s 20.5 100 20.5")
            cmd("gamemode spectator @s")
            ctl("wait", 20)
            return capture(cmd, step)
        finally:
            if socket.exists():
                ctl("quit")


def capture_java(server, java_home, output):
    import java_reference

    class Args:
        variants = grazing = False
        behavior = True

    args = Args()
    args.server, args.java_home, args.output = server, java_home, output

    def case(cmd, _):
        def step(ticks):
            cmd(f"tick step {ticks}")
            time.sleep(ticks / 20 + 0.12)
        return capture(cmd, step)

    java_reference.deer_behavior.capture = case
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "egg-family-parity-fixture"])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--java-home", type=Path)
    parser.add_argument("--reference", type=Path)
    args = parser.parse_args()
    if args.java_home:
        capture_java(args.root, args.java_home, args.output)
    else:
        result = capture_horizon(args.root)
        print(json.dumps(result, indent=2))
        args.output.write_text(json.dumps(result, indent=2) + "\n")
        if args.reference:
            reference = json.loads(args.reference.read_text())
            assert set(reference) == set(result), (set(reference), set(result))
            for name, actual in result.items():
                expected = reference[name]
                assert sorted(actual["has_egg"]) == sorted(expected["has_egg"]), (name, expected, actual)
                assert all(actual[key] == expected[key] for key in ("parent_love", "parent_age", "newborn")), (
                    name, expected, actual
                )
