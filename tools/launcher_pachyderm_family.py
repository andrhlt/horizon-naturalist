"""Compare exact Elephant/Mammoth/Cow breeding and juvenile follow identity."""
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

    elephant, mammoth, cow = "naturalist:elephant", "naturalist:mammoth", "minecraft:cow"
    kinds = (elephant, mammoth, cow)

    def clear():
        for kind in kinds:
            cmd(f"kill @e[type={kind}]")
        step(25)

    result = {}
    cmd("time set 1000")
    for name, left, right in [
        ("elephant_pair", elephant, elephant),
        ("mammoth_pair", mammoth, mammoth),
        ("elephant_mammoth", elephant, mammoth),
        ("mammoth_elephant", mammoth, elephant),
        ("elephant_cow", elephant, cow),
        ("cow_mammoth", cow, mammoth),
    ]:
        clear()
        for x, kind, tag in [(6.5, left, "left"), (9.1, right, "right")]:
            rows = cmd(f"summon {kind} {x} 100 8.5 "
                       f"{{Age:0,InLove:600,OnGround:1b,Tags:[social_probe,{tag}]}}")
            assert data(tag, "Pos"), rows
        step(90)
        state = {
            "parent_love": [int(data(tag, "InLove") or 0) for tag in ("left", "right")],
            "parent_age": [int(data(tag, "Age") or 0) for tag in ("left", "right")],
            "newborn": [
                kind for kind in kinds
                if any(" following entity data: " in row for row in cmd(
                    f"data get entity @e[type={kind},tag=!social_probe,sort=nearest,limit=1] Age"))
            ],
        }
        result[name] = state
        print(name, state, flush=True)

    for name, baby, adult in [
        ("elephant_follow_elephant", elephant, elephant),
        ("mammoth_follow_mammoth", mammoth, mammoth),
        ("elephant_follow_mammoth", elephant, mammoth),
        ("mammoth_follow_elephant", mammoth, elephant),
        ("elephant_follow_cow", elephant, cow),
    ]:
        clear()
        rows = cmd(f"summon {adult} 20.5 100 8.5 "
                   "{Age:0,NoAI:1b,OnGround:1b,Tags:[parent_probe]}")
        assert data("parent_probe", "Pos"), rows
        rows = cmd(f"summon {baby} 6.5 100 8.5 "
                   "{Age:-10000,OnGround:1b,Tags:[baby_probe]}")
        assert data("baby_probe", "Pos"), rows
        positions = []
        for _ in range(4):
            positions.append([float(value.rstrip("d"))
                              for value in data("baby_probe", "Pos")[1:-1].split(", ")])
            if len(positions) < 4:
                step(4)
        result[name] = positions
        print(name, positions, flush=True)
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
                "--world", "Naturalist Pachyderm Families", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
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
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "pachyderm-family-parity-fixture"])


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
                if name.endswith("_pair"):
                    # The parents tick independently, so their cooldown application
                    # can differ by one tick.
                    assert actual["newborn"] == expected["newborn"], (name, expected, actual)
                    assert actual["parent_love"] == expected["parent_love"], (name, expected, actual)
                    assert all(abs(left - right) <= 1 for left, right in zip(
                        actual["parent_age"], expected["parent_age"], strict=True)), (name, expected, actual)
                elif isinstance(expected, list):
                    if expected[1] != expected[0]:
                        assert actual[:2] == expected[:2], (name, expected, actual)
                    else:
                        juvenile = name.split("_", 1)[0]
                        treatment = f"{juvenile}_follow_{juvenile}"
                        assert actual[:2] != reference[treatment][:2], (name, expected, actual)
                        assert actual[:2] != result[treatment][:2], (name, expected, actual)
                else:
                    assert actual == expected, (name, expected, actual)
