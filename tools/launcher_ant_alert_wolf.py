"""Verify an attacked Ant alerts another Ant, but not a nearby Wolf."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time
import shutil
import hashlib
from prepare import JAR_SHA


def capture(version, jar):
    with tempfile.TemporaryDirectory(prefix="nat-ant-alert-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "ant.sock"
        (root / "jars").mkdir()
        if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
            raise ValueError("expected pinned Naturalist Java jar")
        shutil.copy2(jar, root / "jars" / f"{JAR_SHA}.jar")
        (root / "horizon-mods.txt").write_text("mod.naturalist:true\n")

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def data(entity, path):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in cmd(f"data get entity {entity} {path}")
                         if marker in row), None)

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ant Alert", "--creative", "--test-terrain", "--render-distance", 4, "--wait")

        def quit_game():
            ctl("quit")
            deadline = time.monotonic() + 12
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.1)

        try:
            launch()
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 8.5 103 14.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tp @s 8.5 100 14.5")
            cmd("gamerule minecraft:mob_drops false")
            cmd("gamemode survival @s")
            cmd("summon naturalist:ant 3.5 100 3.5 {Tags:[ant],HillCooldown:1200}")
            cmd("summon minecraft:wolf 4.5 100 3.5 {Tags:[wolf],PersistenceRequired:1b}")
            cmd("summon naturalist:ant 5.5 100 3.5 {Tags:[other_ant],HillCooldown:1200}")
            ant="@e[tag=ant,limit=1]";wolf="@e[tag=wolf,limit=1]"
            before=data(ant,"Health")
            cmd("damage @e[tag=ant,limit=1] 1 minecraft:player_attack by @s")
            after=data(ant,"Health");assert before and after and float(after.rstrip("f")) < float(before.rstrip("f")),(before,after)
            ctl("wait",120)
            ant_target=data(ant,"angry_at");wolf_target=data(wolf,"angry_at")
            other_target=data("@e[tag=other_ant,limit=1]","angry_at")
            print(json.dumps({"ant_anger":ant_target,"other_ant_anger":other_target,"wolf_anger":wolf_target}),flush=True)
            assert ant_target is not None,"Ant did not become angry"
            assert wolf_target is None,"Ant incorrectly alerted a vanilla Wolf"
            assert other_target == ant_target,"Ant failed to alert another Ant"
        finally:
            if socket.exists():
                quit_game()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--jar", type=Path, required=True)
    args = parser.parse_args()
    capture(args.version, args.jar)
