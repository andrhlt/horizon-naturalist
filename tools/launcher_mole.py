"""Compare Mole damage, food and digging to the pinned Java fixture."""
import argparse
import json
import re
from pathlib import Path
import subprocess
import tempfile
from launcher_managed import start as start_managed, stop as stop_managed


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-mole-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "control.sock"

        def ctl(*args):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=65,
            )
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(command):
            return ctl("command", command)["chat"]

        def data(target, key):
            marker = " following entity data: "
            rows = cmd(f"data get entity {target} {key}")
            return next((row.partition(marker)[2] for row in rows if marker in row), None)

        def number(target, key):
            value = data(target, key)
            return float(value.rstrip("bLfd")) if value is not None else None

        def trails():
            results = cmd("execute if entity @e[type=naturalist:dirt_trail]")
            return next((int(match[1]) for row in results
                         if (match := re.search(r"Count: (\d+)", row))), 0)

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        def await_phase(target, wanted, ticks):
            for elapsed in range(ticks + 1):
                if number(target, "MoleState") == wanted:
                    return elapsed
                step(1)
            raise AssertionError((target, wanted, number(target, "MoleState")))

        managed = None
        try:
            managed = start_managed(version, home, socket, "Naturalist Mole",
                                    ("--test-terrain", "--render-distance", "4"))
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:advance_time false")
            cmd("time set midnight")
            cmd("tp @s 8.5 101 4.5")
            ctl("wait", 20)
            cmd("fill 2 100 2 14 100 14 minecraft:stone")
            cmd("fill 2 101 2 14 104 14 minecraft:air")
            cmd("tick freeze")
            trial = "@e[type=naturalist:mole,tag=trial,limit=1]"
            states = {}
            for phase, expected in reference["states"].items():
                cmd(f"summon naturalist:mole 8.5 101 7.5 {{MoleState:{phase},NoAI:1b,Tags:[trial]}}")
                assert number(trial, "MoleState") == expected["saved"]
                result = cmd(f"attribute {trial} minecraft:step_height base get")
                assert any(f'is {expected["step_height"]}' in row for row in result), result
                cmd(f"damage {trial} 1 minecraft:generic")
                health = number(trial, "Health")
                states[phase] = health == 7.0
                assert states[phase] == expected["hurt"], (phase, health, states)
                cmd(f"kill {trial}")
                step(25)

            live = "@e[type=naturalist:mole,tag=dig,limit=1]"
            cmd("summon naturalist:mole 8.2 101 8.8 {Tags:[dig]}")
            cmd("summon minecraft:zombie 10.5 101 8.5 {NoAI:1b,Tags:[undead]}")
            await_phase(live, 1, 12)
            dig_time = await_phase(live, 2, 23)
            mound_count = trails()
            assert mound_count == reference["mound_count"], (mound_count, dig_time)
            cmd("kill @e[tag=undead]")
            first_calm_tick = await_phase(live, 3, 65)
            peek_time = await_phase(live, 4, 59)
            emerge_time = await_phase(live, 0, 26)
            assert dig_time == 20 and peek_time == 57 and emerge_time == 25
            assert 59 <= first_calm_tick <= 64, first_calm_tick
            assert trails() == 0
            wolf_result = None
            if "wolf_dig" in reference:
                cmd(f"kill {live}")
                step(25)
                for axis in (7, 9):
                    cmd(f"fill {axis} 101 7 {axis} 102 9 minecraft:stone")
                    cmd(f"fill 7 101 {axis} 9 102 {axis} minecraft:stone")
                cmd("summon naturalist:mole 8.5 101 8.5 {MoleState:2,Tags:[wolf_mole]}")
                cmd("summon minecraft:zombie 6.5 101 8.5 {NoAI:1b,Tags:[wolf_undead]}")
                cmd("fill 8 101 7 11 102 7 minecraft:stone")
                cmd("fill 8 101 9 11 102 9 minecraft:stone")
                cmd("fill 11 101 7 11 102 9 minecraft:stone")
                cmd("summon minecraft:wolf 10.4 101 8.5 {Tags:[digging_wolf]}")
                enclosed = "@e[tag=wolf_mole,limit=1]"
                wolf = "@e[tag=digging_wolf,limit=1]"
                # Movement goals remain active. Fixed geometry keeps the actors at
                # the dig distance used by the Java goal comparison.
                for actor in (enclosed, wolf):
                    fixed = cmd(f"attribute {actor} minecraft:movement_speed base set 0")
                    assert any("set to 0.0" in row for row in fixed), (actor, fixed)

                def wolf_digging():
                    return any(
                        row["kind"] == "minecraft:wolf"
                        and abs(row["pos"][0] - 10.4) < 1.2
                        and abs(row["pos"][2] - 8.5) < 0.6
                        and "WolfDig" in row["goals"]
                        for row in ctl("mobs", 64)["near"]
                    )

                step(59)
                assert not wolf_digging()
                assert number(enclosed, "MoleState") == 2
                cmd("fill 9 101 7 9 102 9 minecraft:air")
                ctl("tp", 8.5, 101, 5.5)
                for _ in range(135):
                    step(1)
                    if wolf_digging():
                        break
                else:
                    raise AssertionError("the Wolf did not dig out its underground target")
                step(48)
                at_49 = number(enclosed, "MoleState")
                step(1)
                at_50 = number(enclosed, "MoleState")
                wolf_result = {"phase_at_49": at_49, "phase_at_50": at_50}
                assert all(reference["wolf_dig"][key] == value for key, value in wolf_result.items()), wolf_result
            return {"damage_phases": states, "dig_ticks": dig_time, "calm_ticks": first_calm_tick,
                    "peek_ticks": peek_time, "emerge_ticks": emerge_time, "mounds": mound_count, "wolf_dig": wolf_result}
        finally:
            if socket.exists():
                ctl("quit")
            stop_managed(managed)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    print(json.dumps(capture(args.version, json.loads(args.reference.read_text())), indent=2))
