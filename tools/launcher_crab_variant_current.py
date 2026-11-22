"""Verify wild and owned Crab capture/release against Java-selected hand traits."""
import argparse
import hashlib
import shutil
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time


def capture(version, jar):
    with tempfile.TemporaryDirectory(prefix="nat-capture-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "capture.sock"
        if jar is not None:
            from prepare import JAR_SHA
            if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
                raise ValueError("expected the pinned Naturalist 2.0.7 NeoForge 26.3 jar")
            (directory / "jars").mkdir()
            shutil.copyfile(jar, directory / "jars" / f"{JAR_SHA}.jar")
            (directory / "horizon-mods.txt").write_text("mod.naturalist:true\n")

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75, cwd=directory,
                                   env={**os.environ, "HORIZON_HOME": str(directory), "HORIZON_NO_AUDIO": "1"})
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def data(target, nbt):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {target} {nbt}")
                         if "following entity data: " in row), None)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Capture Net Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 7.5 100 6.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 7.5 100 6.5")
            cmd("gamemode creative @s")
            cases = [("absent", "", "naturalist:blue"), ("minus_one", "Variant:-1,", "naturalist:yellow"),
                     ("five", "Variant:5,", "naturalist:blue"), ("fraction", "Variant:-1.2f,", "naturalist:red"),
                     ("custom", 'Variant:"fixture:striped",', "fixture:striped"), ("invalid", 'Variant:"bad value!",', "naturalist:blue")]
            actual = {}
            for tag, value, expected in cases:
                cmd(f"summon naturalist:crab 7.5 100 8.5 {{{value}NoAI:1b,Tags:[v_{tag}]}}")
                got = data(f"@e[tag=v_{tag},limit=1]", "Variant")
                actual[tag]=got
                assert got == f'"{expected}"', (tag, got, expected)
            print(json.dumps(actual), flush=True)
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--jar", type=Path, help="original jar required by the current external-jar package")
    args = parser.parse_args()
    capture(args.version, args.jar)
