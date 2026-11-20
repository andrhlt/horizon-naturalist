"""Compare actual juvenile following near the Java parent scan boundary."""
import argparse
import json
import re
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

    others = ("naturalist:hippo", "naturalist:alligator", "naturalist:mole")
    result = {}
    cmd("time set 1000")
    for kind in others:
        for entity in ("minecraft:cow", *others):
            cmd(f"kill @e[type={entity}]")
        step(25)
        rows = cmd('summon minecraft:cow 6.5 100 8.5 {Age:0,InLove:600,OnGround:1b,Tags:[cow_probe]}')
        assert data("cow_probe", "Pos"), rows
        rows = cmd(f'summon {kind} 8.75 100 8.5 '
                   '{Age:0,InLove:600,NoAI:1b,OnGround:1b,Tags:[other_probe]}')
        assert data("other_probe", "Pos"), rows
        step(90)
        state = {
            "love": [int(data(tag, "InLove") or 0) for tag in ("cow_probe", "other_probe")],
            "age": [int(data(tag, "Age") or 0) for tag in ("cow_probe", "other_probe")],
            "new_cow": next((row.split("following entity data: ", 1)[1] for row in cmd(
                "data get entity @e[type=minecraft:cow,tag=!cow_probe,limit=1] Age")
                if "following entity data: " in row), None),
        }
        result[kind] = state
        print(kind, state, flush=True)
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
                "--world", "Naturalist Cow And Other Families", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
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
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "other-cow-templates-parity-fixture"])


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
            expected = json.loads(args.reference.read_text())
            assert result == expected, (expected, result)
