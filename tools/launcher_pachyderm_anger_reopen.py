"""Save and reopen a real herd retaliation target in each isolated game."""
import argparse
import json
from pathlib import Path
import socket as ipc
import re
import subprocess
import sys
import tempfile
import time


WORLD = "Pachyderm Retaliation Reopen"


def prepare(cmd, step, species):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:advance_time false")
    cmd("forceload add 0 0 31 31")
    cmd("tick freeze")
    cmd("fill 2 101 2 15 101 15 minecraft:stone")
    cmd("fill 2 102 2 15 107 15 minecraft:air")
    cmd("kill @e[tag=anger_reopen_fixture]")
    step(25)
    cmd(f"summon naturalist:{species} 6.5 102 8.5 {{Age:-10000,OnGround:1b,Tags:[child_probe,anger_reopen_fixture]}}")
    cmd(f"summon naturalist:{species} 10.5 102 8.5 {{Age:0,OnGround:1b,Tags:[adult_probe,anger_reopen_fixture]}}")
    cmd('summon minecraft:pig 8.5 102 11.5 {NoAI:1b,PersistenceRequired:1b,Health:1000f,'
        'attributes:[{id:"minecraft:max_health",base:1000},{id:"minecraft:knockback_resistance",base:1}],Tags:[attacker_probe,anger_reopen_fixture]}')
    step(6)
    cmd("damage @e[tag=child_probe,limit=1] 1 minecraft:mob_attack by @e[tag=attacker_probe,limit=1]")
    step(6)
    cmd("save-all flush")
    return state(cmd)


def state(cmd):
    def data(tag, field):
        rows = cmd(f"data get entity @e[tag={tag},limit=1] {field}")
        marker = "following entity data: "
        return next((row.split(marker, 1)[1] for row in rows if marker in row), None)

    elapsed = next(int(match[1]) for row in cmd("time query gametime")
                   if (match := re.search(r"The game time is (\d+)", row)))
    return {"time": elapsed, "attacker": data("attacker_probe", "UUID"), "adult_uuid": data("adult_probe", "UUID"),
            "anger_target": data("adult_probe", "angry_at"), "anger_end": data("adult_probe", "anger_end_time"),
            "child_health": data("child_probe", "Health")}


def saved_entity(root, anvil_dump, expected, marker='adult_probe'):
    region = next(root.rglob("entities/r.0.0.mca"))
    output = subprocess.run([sys.executable, anvil_dump, region, "0", "0", "Entities"],
                            check=True, capture_output=True, text=True, timeout=20)
    entities = json.loads(output.stdout)
    adult = next(entity for entity in entities if marker in entity.get("Tags", []))

    def uuid(value):
        return "[I; " + ", ".join(map(str, value)) + "]"

    fields = {
        "adult_uuid": uuid(adult["UUID"]),
        "anger_target": uuid(adult["angry_at"]),
        "anger_end": f"{adult['anger_end_time']}L",
    }
    assert all(value == expected[key] for key, value in fields.items()), (fields, expected)
    return fields


def horizon(version, species, anvil_dump):
    with tempfile.TemporaryDirectory(prefix="nat-pachyderm-reopen-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "first.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    text=True, check=True, capture_output=True, timeout=75)
            return json.loads(result.stdout.splitlines()[-1])

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", WORLD, "--creative", "--test-terrain", "--render-distance", 4, "--wait")

        def cmd(line):
            return ctl("command", line)["chat"]

        def step(num):
            cmd(f"tick step {num}")
            ctl("wait", num + 2)

        try:
            launch()
            cmd("tp @s 8.5 102 6.5")
            cmd("gamemode spectator @s")
            ctl("wait", 20)
            before = prepare(cmd, step, species)
            ctl("quit")
            deadline = time.monotonic() + 15
            while time.monotonic() < deadline:
                with ipc.socket(ipc.AF_UNIX) as probe:
                    try:
                        probe.connect(str(socket))
                    except (FileNotFoundError, ConnectionRefusedError):
                        break
                time.sleep(0.05)
            snapshot = saved_entity(home, anvil_dump, before) if anvil_dump else None
            socket = home / "reopen.sock"
            launch()
            cmd("tick freeze")
            return {"before": before, "reopened": state(cmd), "disk_before_reopen": snapshot}
        finally:
            if socket.exists():
                ctl("quit")


def java(server, java_home, output, species, anvil_dump):
    import java_reference

    class Args:
        variants = grazing = False
        behavior = True

    args = Args()
    args.server, args.java_home = server, java_home
    rows = {}
    world = f"pachyderm-anger-reopen-{species}-case"
    for phase in ("before", "reopened"):
        args.output = output.with_name(f"{output.stem}-{phase}.json")

        def capture(cmd, _):
            def step(num):
                cmd(f"tick step {num}")
                time.sleep(num / 20 + 0.18)

            if phase == "before":
                return prepare(cmd, step, species)
            cmd("tick freeze")
            return state(cmd)

        java_reference.deer_behavior.capture = capture
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", world])
        rows[phase] = json.loads(args.output.read_text())
        if phase == "before" and anvil_dump:
            rows["disk_before_reopen"] = saved_entity(server / world, anvil_dump, rows[phase])
    return rows


def compare(result):
    before, reopened = result["before"], result["reopened"]
    attacker = before["attacker"]
    assert attacker and before["anger_target"] == attacker, result
    assert before["adult_uuid"] == reopened["adult_uuid"] and before["adult_uuid"], result
    assert reopened["anger_target"] == attacker and reopened["attacker"] == attacker, result
    assert before["child_health"] == reopened["child_health"] == "17.0f", result
    for row in (before, reopened):
        left = int(row["anger_end"].rstrip("L")) - row["time"]
        assert 380 <= left <= 780, (left, result)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--java-home", type=Path)
    parser.add_argument("--species", choices=("elephant", "mammoth"), default="elephant")
    parser.add_argument("--compare", action="store_true")
    parser.add_argument("--anvil-dump", type=Path, help="Horizon anvil_dump.py for exact on-disk verification")
    args = parser.parse_args()
    result = (java(args.root, args.java_home, args.output, args.species, args.anvil_dump) if args.java_home
              else horizon(args.root, args.species, args.anvil_dump))
    text = json.dumps(result, indent=2) + "\n"
    args.output.write_text(text)
    print(text)
    if args.compare:
        compare(result)
