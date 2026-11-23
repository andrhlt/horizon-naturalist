"""Check that in-love adult Whales lack the Java mod’s autonomous breed goal."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import shutil

version = Path(sys.argv[1])
with tempfile.TemporaryDirectory(prefix="nat-whale-natural-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "whale.sock"
    jar = Path.home() / "Library/Application Support/horizon/jars/b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd.jar"
    if jar.is_file():
        (home / "jars").mkdir()
        shutil.copyfile(jar, home / "jars" / jar.name)
        (home / "horizon-mods.txt").write_text("mod.naturalist:true\n")

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    try:
        manifest = json.loads((version / "native-mods.json").read_text())
        resources = Path.home() / "Library/Application Support/horizon-launcher/versions" / manifest["base"] / ".local/horizon-client/26.3"
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            resources, "--home", home, "--game-dir", home,
            "--world", "Whale courtship", "--seed", 17, "--creative",
            "--test-terrain", "--render-distance", 5, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("difficulty normal")
        cmd("tp @s 8.5 105 0.5")
        ctl("wait-ready")
        cmd("tick freeze")
        cmd("fill 0 91 0 20 91 22 minecraft:stone")
        cmd("fill 0 92 0 20 108 22 minecraft:water")
        kind = "naturalist:whale"
        for x in (7.5, 13.5):
            result = cmd(f"summon {kind} {x} 98 10.5 {{InLove:600,NoGravity:1b}}")
            assert any("Summoned" in row for row in result), result
        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 3)
        step(150)
        rows = ctl("entities", 100, kind)["entities"]
        assert len(rows) == 2 and all(not row["baby"] for row in rows), rows
        print(json.dumps({"adult_count": len(rows), "actual_breed_goal_spawned_offspring": False}), flush=True)
    finally:
        if socket.exists():
            ctl("quit")
