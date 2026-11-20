"""Compare which nearby pachyderms alert on actual juvenile damage."""
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
    for subject, adult in [("elephant", "elephant"), ("mammoth", "mammoth"),
                           ("elephant", "mammoth"), ("mammoth", "elephant")]:
        for tag in ("alert_child", "alert_adult", "attack_probe"):
            cmd(f"kill @e[tag={tag}]")
        step(25)
        cmd(f"summon naturalist:{subject} 6.5 100 8.5 {{Age:-10000,OnGround:1b,Tags:[alert_child]}}")
        cmd(f"summon naturalist:{adult} 10.5 100 8.5 {{Age:0,OnGround:1b,Tags:[alert_adult]}}")
        cmd("summon minecraft:pig 8.5 100 14.5 {NoAI:1b,Tags:[attack_probe]}")
        source = data("attack_probe", "UUID")
        assert all(data(tag, "Pos") for tag in ("alert_child", "alert_adult", "attack_probe"))
        step(6)
        cmd("damage @e[tag=alert_child,limit=1] 1 minecraft:mob_attack by @e[tag=attack_probe,limit=1]")
        name = f"{subject}_alerts_{adult}"
        result[name] = []
        for sample in range(7):
            if sample:
                step(4)
            row = [sample * 4, data("alert_adult", "angry_at") == source,
                   data("alert_adult", "anger_end_time"), coordinates("alert_adult"),
                   data("alert_child", "Health"), data("attack_probe", "Health")]
            result[name].append(row)
        print(name, result[name], flush=True)
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
                "--world", "Naturalist Pachyderm Retaliation", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
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
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "pachyderm-live-alert-parity-fixture"])


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
            assert set(result) == set(expected)
            for name, trajectory in result.items():
                # Actual adult advancement toward the damaging mob identifies
                # the delegated alert, with its own unalerted mixed control.
                planned = expected[name][-1][3][2] - expected[name][0][3][2] > 1.0
                observed = trajectory[-1][3][2] - trajectory[0][3][2] > 1.0
                assert observed == planned, (name, expected[name], trajectory)
                assert all(row[4] == "17.0f" for row in trajectory), (name, trajectory)
