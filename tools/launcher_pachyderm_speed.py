"""Compare pachyderm pursuit, effective sprint speed, and rider behavior."""
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

    result = {}
    for species in ("elephant", "mammoth"):
        kind = f"naturalist:{species}"
        for rider in (False, True):
            cmd("kill @e[tag=pachyderm_target]")
            cmd("kill @e[tag=baby_probe]")
            cmd("kill @e[tag=rider_probe]")
            step(25)
            start = ticks_elapsed()
            cmd(f"summon {kind} 20.5 100 8.5 {{Age:0,OnGround:1b,NoAI:1b,Tags:[pachyderm_target]}}")
            passenger = ',Passengers:[{id:"minecraft:chicken",NoAI:1b,Tags:[rider_probe]}]' if rider else ""
            cmd(f"summon {kind} 6.5 100 8.5 {{Age:-10000,OnGround:1b,Tags:[baby_probe]{passenger}}}")
            has_passenger = data("baby_probe", "Passengers[0].UUID") is not None
            assert has_passenger == rider, (species, rider, has_passenger)
            name = f"{species}_{'mounted' if rider else 'unmounted'}"
            result[name] = []
            for tick in range(9):
                assert ticks_elapsed() - start == tick, (ticks_elapsed(), start, tick)
                query = "@e[tag=baby_probe,limit=1]"
                reply = cmd(f"attribute {query} minecraft:movement_speed get 1000000")
                attrs = [float(row.rsplit(" is ", 1)[1]) for row in reply if " is " in row]
                row = [tick, coordinates("baby_probe"), attrs]
                result[name].append(row)
                print(name, row, flush=True)
                if tick < 8:
                    step(1)
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
                "--world", "Naturalist Parent Scan Boundary", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
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
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "elephant-follow-speed-parity-fixture"])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--java-home", type=Path)
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--check-attribute", action="store_true", help="Check the command readback in addition to movement")
    args = parser.parse_args()
    if args.java_home:
        capture_java(args.root, args.java_home, args.output)
    else:
        result = capture_horizon(args.root)
        print(json.dumps(result, indent=2))
        args.output.write_text(json.dumps(result, indent=2) + "\n")
        if args.reference:
            expected = json.loads(args.reference.read_text())
            assert set(result) == set(expected)
            for name, actual in result.items():
                sample = expected[name]
                if args.check_attribute:
                    assert [row[2] for row in actual] == [row[2] for row in sample], (name, sample, actual)
                if name.endswith("_unmounted"):
                    assert [row[:2] for row in actual[:5]] == [row[:2] for row in sample[:5]], (
                        name, sample, actual
                    )
