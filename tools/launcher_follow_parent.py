"""Compare actual juvenile following near the Java parent scan boundary."""
import argparse
import json
import re
from pathlib import Path
import subprocess
import tempfile
import time

def capture(cmd, step, suite):
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
        ("deer_inside", "deer", "naturalist:deer", 7.75),
        ("deer_edge", "deer", "naturalist:deer", 8.25),
        ("deer_outside", "deer", "naturalist:deer", 9.75),
        ("deer_wrong_parent", "deer", "minecraft:cow", 7.75),
        ("boar_inside", "boar", "naturalist:boar", 7.75),
        ("boar_edge", "boar", "naturalist:boar", 8.25),
        ("boar_outside", "boar", "naturalist:boar", 9.75),
        ("boar_wrong_parent", "boar", "minecraft:pig", 7.75),
        ("capybara_inside", "capybara", "naturalist:capybara", 12),
    ]
    if suite == "bounds":
        cases = [
            (f"{kind}_{distance}", kind, f"naturalist:{kind}", distance)
            for kind in ("turkey", "rhino", "komodo_dragon", "alligator", "rat", "tiger", "lion")
            for distance in (7.75, 8.25, 12.5 if kind == "alligator" else 10.5)
        ]
    result = {}
    for name, baby, adult, distance in cases:
        if suite == "bounds":
            time_of_day = 1000 if baby in {"rat", "tiger", "lion"} else 15000
            cmd(f"time set {time_of_day}")
        cmd("kill @e[tag=parent_probe]")
        cmd("kill @e[tag=baby_probe]")
        step(25)
        rows = cmd(f"summon {adult} {6.5 + distance} 100 8.5 "
                   "{Age:0,NoAI:1b,OnGround:1b,Tags:[parent_probe]}")
        assert data("parent_probe", "Pos"), rows
        rows = cmd(f"summon naturalist:{baby} 6.5 100 8.5 "
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


def capture_horizon(version, suite):
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
                "--world", "Naturalist Parent Scan Boundary", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("tp @s 20.5 100 20.5")
            cmd("gamemode spectator @s")
            ctl("wait", 20)
            return capture(cmd, step, suite)
        finally:
            if socket.exists():
                ctl("quit")


def capture_java(server, java_home, output, suite):
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
        return capture(cmd, step, suite)

    java_reference.deer_behavior.capture = case
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "parent-scan-boundary-parity-fixture"])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--java-home", type=Path)
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--suite", choices=("core", "bounds"), default="core")
    args = parser.parse_args()
    if args.java_home:
        capture_java(args.root, args.java_home, args.output, args.suite)
    else:
        result = capture_horizon(args.root, args.suite)
        print(json.dumps(result, indent=2))
        args.output.write_text(json.dumps(result, indent=2) + "\n")
        if args.reference:
            reference = json.loads(args.reference.read_text())
            assert set(reference) == set(result), (set(reference), set(result))
            for name, actual in result.items():
                expected = reference[name]
                assert [row[0] for row in actual] == [0, 4, 8, 12, 16], (name, actual)
                if name.endswith(("_outside", "_wrong_parent", "_10.5", "_12.5")):
                    # The nearby-parent control begins pursuing on the first sampled ticks.
                    # Compare that initial path so a later unrelated stroll can run normally.
                    if args.suite == "core":
                        species = name.split("_", 1)[0]
                        treatment = f"{species}_inside"
                    else:
                        species, _ = name.rsplit("_", 1)
                        treatment = f"{species}_7.75"
                    initial = actual[:2]
                    reference_control = reference[treatment][:2]
                    observed_control = result[treatment][:2]
                    assert expected[:2] != reference_control, (name, expected, reference_control)
                    assert initial != reference_control and initial != observed_control, (
                        name, initial, reference_control, observed_control
                    )
                elif name.startswith(("rhino_", "alligator_")):
                    # Their concurrent wandering goals can change the exact turn.
                    # Both should advance towards the parent while staying near the route.
                    for samples in (expected, actual):
                        assert samples[-1][1] > samples[0][1] + 0.4, (name, expected, actual)
                        assert abs(samples[-1][3] - samples[0][3]) < 1.0, (name, expected, actual)
                else:
                    # Compare admission and initial pursuit before a random idle goal
                    # can issue a later movement request of its own.
                    assert actual[:2] == expected[:2], (name, expected, actual)
                    assert actual[-1][1] > actual[0][1], (name, expected, actual)
