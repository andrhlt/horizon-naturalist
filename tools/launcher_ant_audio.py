"""Replay real Ant ambient, stone-step, hurt, and death sounds headlessly."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

JAR_SHA512 = (
    "b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538"
    "c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd"
)


def capture(version: Path, resources: Path, jar: Path, dev_mod: Path):
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA512:
        raise ValueError("unexpected Naturalist Java jar")
    with tempfile.TemporaryDirectory(prefix="nat-ant-audio-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        (root / "jars").mkdir()
        shutil.copy2(jar, root / "jars" / f"{JAR_SHA512}.jar")
        (root / "horizon-mods.txt").write_text("mod.naturalist:true\n")
        socket = root / "ant.sock"

        def ctl(*args):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                capture_output=True, text=True, timeout=75,
            )
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def sounds():
            return ctl("status")["heard"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", resources, "--home", root, "--game-dir", root,
                "--dev-mod", dev_mod, "--world", "Ant Audio", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 10.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tp @s 10.5 100 6.5")
            before = sounds()
            cmd("item replace entity @s weapon.mainhand with naturalist:ant_spawn_egg")
            ctl("wait", 5)
            ctl("look-at-block", 8, 99, 8, "up")
            ctl("mouse", "right", "click")
            ctl("wait", 8)
            cmd("summon naturalist:ant 10.5 100 8.5 {Tags:[ant-ambient],NoAI:1b,HillCooldown:1200}")
            ctl("wait", 200)
            cmd("summon naturalist:ant 5.5 100 8.5 {Tags:[ant-step],NoAI:0b,HillCooldown:1200,Motion:[0.8d,-0.02d,0.0d]}")
            ctl("wait", 20)
            cmd("damage @e[tag=ant-step,limit=1] 1 minecraft:generic")
            cmd("summon naturalist:ant 6.5 100 8.5 {Tags:[ant-die]}")
            cmd("damage @e[tag=ant-die,limit=1] 20 minecraft:generic")
            ctl("wait", 25)
            heard = sounds()
            increments = {name: count - before.get(name, 0)
                          for name, count in heard.items()
                          if count > before.get(name, 0)}
            return increments
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path, help="exact built candidate directory")
    parser.add_argument("fixture", type=Path, help="pinned Java sound event counts")
    parser.add_argument("--jar", type=Path, required=True)
    parser.add_argument("--resources", type=Path, required=True)
    parser.add_argument("--dev-mod", type=Path, required=True)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    heard = capture(args.version, args.resources, args.jar, args.dev_mod)
    print(json.dumps(heard, indent=2), flush=True)
    assert all(heard.get(event["sound"], 0) > 0 for event in expected), heard
    assert not any("entity.wolf." in name for name in heard), heard
