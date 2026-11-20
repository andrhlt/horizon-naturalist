"""Compare the observed duck-egg hatch branches on an installed headless Horizon build."""
import argparse
from collections import Counter
import json
import math
from pathlib import Path
import socket
import subprocess
import tempfile


def capture(version, fixture):
    with tempfile.TemporaryDirectory(prefix="nat-duck-hatch-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        path = directory / "hatch.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", path, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def raw(line):
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as channel:
                channel.settimeout(15)
                channel.connect(str(path))
                channel.sendall((line + "\n").encode())
                with channel.makefile() as stream:
                    return json.loads(stream.readline())

        def cmd(line):
            return raw("command " + line)["chat"]

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        def data(target, key):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {target} {key}")
                         if "following entity data: " in row), None)

        def clear_ducks():
            cmd("kill @e[type=naturalist:duck,distance=..48]")
            step(22)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Duck Hatch Fixture", "--creative", "--test-terrain", "--render-distance", 6, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 0.5 103 0.5")
            ctl("wait", 40)
            cmd("fill -24 100 -24 24 100 24 minecraft:stone")
            cmd("fill -24 101 -24 24 108 24 minecraft:air")
            cmd("tp @s 0.5 101 0.5")
            cmd("tick freeze")
            clear_ducks()

            cells = [(x + 0.5, z + 0.5) for x in range(-23, 24, 2) for z in range(-23, 24, 2)]
            counts = Counter()
            samples = {}
            for attempt in range(4):
                for x, z in cells:
                    reply = cmd(f"summon naturalist:duck_egg {x} 104 {z} {{Motion:[0.0d,-4.0d,0.0d]}}")
                    assert any("Summoned" in row for row in reply), reply
                step(1)
                ducks = raw("entities 48 naturalist:duck")["entities"]
                groups = Counter()
                for duck in ducks:
                    x, y, z = duck["pos"]
                    assert (x, z) in cells and abs(y - 101.0) < 1e-8, duck
                    assert duck["baby"], duck
                    groups[(x, z)] += 1
                for cell, count in groups.items():
                    if count in samples:
                        continue
                    x, z = cell
                    target = f"@e[type=naturalist:duck,x={x},y=101,z={z},distance=..0.4,limit=1]"
                    samples[count] = {"age": data(target, "Age"), "persistent": data(target, "PersistenceRequired")}
                counts.update(groups.values())
                counts[0] += len(cells) - len(groups)
                clear_ducks()
                print(f"Completed group {attempt + 1} of 4", flush=True)
            total = 4 * len(cells)
            hatched = counts[1] + counts[4]
            expected_hatches = total / 8
            hatch_stddev = math.sqrt(total * (1 / 8) * (7 / 8))
            expected_four = hatched / 32
            four_stddev = math.sqrt(hatched * (1 / 32) * (31 / 32))
            report = {"trials": total, "outcome_counts": dict(sorted(counts.items())),
                      "per_branch_metadata": samples}
            print(json.dumps(report, indent=2), flush=True)
            assert sorted(counts) == sorted(case["hatchlings"] for case in fixture["cases"]), report
            assert abs(hatched - expected_hatches) < 5 * hatch_stddev, report
            assert abs(counts[4] - expected_four) < 5 * four_stddev, report
            assert all(sample["age"] == "-24000" and sample["persistent"] in ("0b", "false")
                       for sample in samples.values()), report
        finally:
            if path.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
