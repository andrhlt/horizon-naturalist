"""Trace a Duck's original downward motion while its AI is disabled."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

import re
import time



def capture(command, warm=None):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    if warm is not None:
        warm()
    command("tick freeze")
    command("kill @e[tag=duck_noai_flight]")
    command("fill 4 99 4 14 99 14 minecraft:stone")
    command("fill 4 100 4 14 140 14 minecraft:air")
    fields = {}
    for no_ai, x in ((True, 8.5), (False, 11.5)):
        label = "duck_disabled" if no_ai else "duck_enabled"
        rows = command(
            f"summon naturalist:duck {x} 125 8.5 "
            + f"{{Age:0,NoAI:{int(no_ai)}b,Motion:[0.2d,-0.5d,0.0d],Tags:[duck_noai_flight,{label}]}}"
        )
        assert any("Summoned" in row for row in rows), rows
        fields[label] = f"@e[tag={label},limit=1]"

    def now():
        return next(int(m[1]) for row in command("time query gametime")
                    if (m := re.search(r"The game time is (\d+)", row)))

    def step(ticks):
        goal = now() + ticks
        command(f"tick step {ticks}")
        timeout = time.monotonic() + 12
        while time.monotonic() < timeout:
            time_advanced = now()
            if time_advanced >= goal:
                assert time_advanced == goal, (ticks, goal, time_advanced)
                return
            time.sleep(0.08)
        raise AssertionError((ticks, goal, now()))

    def vector(field, key):
        rows = command(f"data get entity {field} {key}")
        values = next((row.rsplit(": ", 1)[-1] for row in rows if "following entity data" in row), None)
        assert values is not None, (field, key, rows)
        return [float(part.strip().rstrip("df")) for part in values.strip("[]").split(",")]

    traces = {label: [] for label in fields}
    for t in range(7):
        if t:
            step(1)
        for name, selector in fields.items():
            traces[name].append({"tick": t, "pos": vector(selector, "Pos"),
                                 "motion": vector(selector, "Motion")})
    return traces


if __name__ == "__main__":

    parser = argparse.ArgumentParser()
    parser.add_argument("version", type=Path)
    parser.add_argument("--java", type=Path)
    args = parser.parse_args()
    version = args.version
    with tempfile.TemporaryDirectory(prefix="nat-duck-flight-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "duck.sock"

        def ctl(*arguments):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, arguments)],
                                    check=True, capture_output=True, text=True, timeout=70)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]
        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Duck NoAI fall", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("tp @s 8.5 100 5.5")
            ctl("wait", 15)
            actual = capture(cmd)
            print(json.dumps(actual, indent=2), flush=True)
            if args.java:
                expected = json.loads(args.java.read_text())
                assert actual == expected, {
                    key: {"horizon": rows, "java": expected[key]}
                    for key, rows in actual.items() if rows != expected[key]
                }
        finally:
            if socket.exists():
                ctl("quit")
