"""Compare live Snail mate identity against vanilla Cow template controls."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def observe(version, types):
    with tempfile.TemporaryDirectory(prefix="nat-snail-mates-", ignore_cleanup_errors=True) as folder:
        home = Path(folder)
        socket = home / "control.sock"

        def ctl(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  capture_output=True, text=True, check=True, timeout=75)
            return json.loads(proc.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def data(name, field):
            marker = "following entity data: "
            chat = cmd(f"data get entity @e[tag=mate_{name},limit=1] {field}")
            return next((line.split(marker, 1)[1] for line in chat if marker in line), None)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Snail Breeding Mate Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 7.5 100 6.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tick freeze")
            starts = ((7.48961601217193, 8.515575981742105, -0.006390146355735238, 0.009585219533602859, 573),
                      (8.995862190386188, 8.482069491673474, -0.0020689048069067718, -0.011034158970169444, 590))
            for name, kind, (x, z, vx, vz, love) in zip(("a", "b"), types, starts, strict=True):
                rows = cmd(f"summon {kind} {x} 100 {z} {{Age:0,InLove:{love},OnGround:1b,"
                           f"Motion:[{vx}d,0.0d,{vz}d],Tags:[mate_{name}]}}")
                assert any("Summoned" in row for row in rows), rows
                actual = data(name, "InLove")
                assert actual == str(love), actual
            trace = []
            for tick in range(0, 121, 15):
                if tick:
                    cmd("tick step 15")
                    ctl("wait", 17)
                native = [mob for mob in ctl("mobs", 16)["near"] if mob["kind"] in types]
                row = {"tick": tick,
                       "parents": [{field: data(name, field) for field in ("Pos", "Motion", "InLove", "HasEgg", "Age")}
                                   for name in ("a", "b")],
                       "goals": {str(mob["id"]): mob["goals"] for mob in native}}
                print(json.dumps(row), flush=True)
                trace.append(row)
            return trace
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("version", type=Path)
    cli.add_argument("reference", type=Path)
    cli.add_argument("--control", choices=["mixed", "cows"])
    cli.add_argument("--baseline", action="store_true")
    args = cli.parse_args()
    snail, cow = "naturalist:snail", "minecraft:cow"
    types = {"mixed": (snail, cow), "cows": (cow, cow), None: (snail, snail)}[args.control]
    trace = observe(args.version, types)
    native_first_egg = next((row["tick"] for row in trace if
                             any(parent["HasEgg"] == "1b" for parent in row["parents"])), None)
    final = trace[-1]["parents"]
    gravid = sum(parent["HasEgg"] == "1b" for parent in final)
    love = [int(parent["InLove"] or 0) for parent in final]
    age = [int(parent["Age"] or 0) for parent in final]
    print(json.dumps({"first_egg": native_first_egg, "gravid": gravid, "love": love, "age": age}), flush=True)
    if args.control == "mixed":
        if args.baseline:
            assert gravid == 1 or not all(love), (gravid, love)
        else:
            assert gravid == 0 and all(love) and age == [0, 0], (gravid, love, age)
    elif args.control == "cows":
        assert not any(love) and all(value > 0 for value in age), (love, age)
    elif args.baseline:
        assert gravid == 0 and all(love), (gravid, love)
    else:
        reference = json.loads(args.reference.read_text())
        java_first_egg = next(row["ticks"] for row in reference
                             if any(parent["HasEgg"] == "1b" for parent in row["parents"]))
        assert native_first_egg is not None and abs(native_first_egg - java_first_egg) <= 15, (native_first_egg, java_first_egg)
        assert gravid == 1 and not any(love) and age == [0, 0], (gravid, love, age)
