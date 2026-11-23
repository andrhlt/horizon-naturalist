"""Replay Piranha school recruitment and retaliation headlessly."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
resources = Path.home() / "Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3"
shared_jars = Path.home() / "Library/Application Support/horizon/jars"
with tempfile.TemporaryDirectory(prefix="nat-piranha-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    shutil.copytree(shared_jars, home / "jars", dirs_exist_ok=True)
    socket = home / "piranha.sock"
    env = {**os.environ, "HORIZON_HOME": str(home)}

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=100, env=env)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 3)

    def probe(query):
        marker = "Piranha fixture "
        rows = cmd("naturalist_fixture_piranha " + query)
        return next(json.loads(row.split(marker, 1)[1]) for row in rows if marker in row)

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources", resources,
            "--home", home, "--game-dir", home, "--world", "Piranha School Replay", "--creative",
            "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("tp @s 8.5 92 8.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("forceload add 0 0 31 31")
        cmd("fill 0 91 0 31 91 31 minecraft:stone")
        cmd("fill 0 92 0 31 96 31 minecraft:water")
        summon_count = 0
        for i in range(25):
            x, z = 4.5 + (i % 5) * 1.5, 4.5 + (i // 5) * 1.5
            rows = cmd(f"summon naturalist:piranha {x} 92.5 {z} {{PersistenceRequired:1b}}")
            summon_count += sum("Summoned" in row for row in rows)
        before_tick = probe("before-tick")
        step(220)
        school = probe("school")
        cmd("summon minecraft:zombie 12.5 92 12.5 {PersistenceRequired:1b}")
        cmd("damage @e[type=naturalist:piranha,limit=1] 1 minecraft:mob_attack by @e[type=minecraft:zombie,limit=1]")
        step(3)
        retaliation = probe("retaliation")
        result = {"artifact": version.name, "summon_count": summon_count,
                  "before_school_tick": before_tick, "school": school, "retaliation": retaliation,
                  "java_baseline": json.loads(Path("fixtures/piranha-java.json").read_text())}
        print(json.dumps(result, indent=2), flush=True)
        Path(sys.argv[2]).write_text(json.dumps(result, indent=2) + "\n")
        assert summon_count == 25 and school["members"] == 25 and school["largest_school"] == 24 and school["attached_followers"] >= 23, school
        assert retaliation["targeted"] >= 1, retaliation
    finally:
        try:
            ctl("quit")
        except Exception:
            pass
