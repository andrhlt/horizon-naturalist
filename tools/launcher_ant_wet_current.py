"""Replay real Ant ambient, stone steps, hurt and death sound IDs headlessly."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import shutil
import hashlib
from prepare import JAR_SHA


def capture(version, jar):
    with tempfile.TemporaryDirectory(prefix="nat-ant-audio-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "ant.sock"
        if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA: raise ValueError("wrong reference jar")
        (root / "jars").mkdir()
        shutil.copy2(jar, root / "jars" / f"{JAR_SHA}.jar")
        (root / "horizon-mods.txt").write_text("mod.naturalist:true\n")

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def sounds():
            return ctl("status")["heard"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ant Audio", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 10.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tp @s 10.5 100 6.5")
            before = sounds()
            cmd("fill 4 100 7 6 102 9 minecraft:stone hollow")
            cmd("setblock 5 100 8 minecraft:water")
            cmd("summon naturalist:ant 5.5 100 8.5 {Tags:[wet_ant],NoAI:0b,HillCooldown:1200}")
            ctl("wait", 50)
            cmd("setblock 5 100 8 minecraft:air")
            ctl("wait", 100)
            increments = {name: count - before.get(name, 0) for name, count in sounds().items() if count > before.get(name, 0)}
            print(json.dumps({"ant":increments},indent=2),flush=True)
            assert not any("entity.wolf.shake" in name for name in increments), increments
            cmd("kill @e[tag=wet_ant]")
            ctl("wait", 25)
            baseline = sounds()
            cmd("setblock 5 100 8 minecraft:water")
            cmd("summon minecraft:wolf 5.5 100 8.5 {Tags:[wet_wolf],NoAI:0b}")
            ctl("wait", 50)
            cmd("setblock 5 100 8 minecraft:air")
            ctl("wait", 100)
            wolf = {name: count - baseline.get(name, 0) for name, count in sounds().items() if count > baseline.get(name, 0)}
            print(json.dumps({"wolf":wolf},indent=2),flush=True)
            assert any("entity.wolf.shake" in name for name in wolf), wolf
            return increments
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--jar",type=Path,required=True)
    args = parser.parse_args()

    sounds = capture(args.version, args.jar)
    assert not any("entity.wolf.shake" in name for name in sounds), sounds
