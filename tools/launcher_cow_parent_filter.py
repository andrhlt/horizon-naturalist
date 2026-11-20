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

    def ticks_elapsed():
        rows = cmd("time query gametime")
        value = next((re.search(r"The game time is (\d+)", row)
                      for row in rows if "The game time is " in row), None)
        assert value, rows
        return int(value[1])

    def coordinates(label):
        return [float(value.rstrip("d")) for value in data(label, "Pos")[1:-1].split(", ")]

    cases = [
        ("actual_cow", "minecraft:cow", "minecraft:cow", 7.75),
        ("snake", "minecraft:cow", "naturalist:snake", 7.75),
        ("dirt_trail", "minecraft:cow", "naturalist:dirt_trail", 7.75),
        ("pig", "minecraft:cow", "minecraft:pig", 7.75),
    ]
    result = {}
    for name, baby, adult, distance in cases:
        cmd("kill @e[tag=parent_probe]")
        cmd("kill @e[tag=baby_probe]")
        step(25)
        rows = cmd(f"summon {adult} {6.5 + distance} 100 8.5 "
                   "{Age:0,NoAI:1b,OnGround:1b,Tags:[parent_probe]}")
        assert data("parent_probe", "Pos"), rows
        rows = cmd(f"summon {baby} 6.5 100 8.5 "
                   "{Age:-10000,OnGround:1b,Tags:[baby_probe]}")
        assert data("baby_probe", "Pos"), rows
        start = ticks_elapsed()
        coordinates_over_time = [[0, *coordinates("baby_probe")]]
        for _ in range(4):
            step(4)
            ticks = ticks_elapsed() - start
            coordinates_over_time.append([ticks, *coordinates("baby_probe")])
        result[name] = coordinates_over_time
        print(name, coordinates_over_time, flush=True)
    return result


def capture_horizon(version):
    with tempfile.TemporaryDirectory(prefix="nat-follow-parent-", ignore_cleanup_errors=True) as directory:
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
                "--world", "Naturalist Cow Parent Identity", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
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
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "cow-typed-parent-parity-fixture"])


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
            assert expected.keys() == result.keys(), (expected.keys(), result.keys())
            assert result["actual_cow"][:2] == expected["actual_cow"][:2]
            for name in ("snake", "dirt_trail", "pig"):
                assert expected[name][:2] != expected["actual_cow"][:2]
                assert result[name][:2] != result["actual_cow"][:2], (name, result[name])
