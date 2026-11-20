"""Verify real Horizon Hill defense and release with ordinary ticks and damage."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-ant-defense-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "ant.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def field(target):
            marker = "following entity data: "
            return [row.split(marker, 1)[1] for row in cmd("data get entity " + target) if marker in row]

        def block(x, workers, open_="true"):
            return any("Test passed" in row for row in cmd(
                f"execute if block {x} 100 8 naturalist:ant_hill[open={open_},workers={workers}]"))

        def wait_until(criterion, max_ticks):
            for _ in range((max_ticks + 4) // 5):
                if criterion():
                    return True
                ctl("wait", 5)
            return False

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ant Hill Defense", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 8.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("time set midnight")
            cmd("tp @s 8.5 100 6.5")
            cmd("gamemode survival @s")
            owner = field("@s UUID")[0]

            cmd("setblock 8 100 8 naturalist:ant_hill[open=true,workers=3,has_queen=false]")
            cmd(f"data merge block 8 100 8 {{Owner:{owner}}}")
            cmd("summon minecraft:zombie 9.5 100 6.5 {NoAI:1b,Tags:[ant-threat]}")
            zombie = "@e[tag=ant-threat,limit=1]"
            threat = field(zombie + " UUID")[0]
            print(json.dumps({"before_damage": field("@s Health"),
                              "damage": cmd(f"damage @s 1 minecraft:mob_attack by {zombie}")}), flush=True)
            defended = wait_until(lambda: block(8, 0), 25)
            ctl("wait", 4)
            owners = cmd("execute as @e[type=naturalist:ant] run data get entity @s Owner")
            targets = cmd("execute as @e[type=naturalist:ant] run data get entity @s angry_at")
            ants = ctl("entities", 12, "naturalist:ant")["entities"]
            print(json.dumps({"defended": defended, "ants": ants, "owners": owners,
                              "targets": targets, "threat": threat}, indent=2), flush=True)

            cmd("kill @e[type=naturalist:ant]")
            cmd(f"kill {zombie}")
            cmd("setblock 8 100 8 minecraft:air")
            cmd("setblock 7 100 8 naturalist:ant_hill[open=true,workers=2,has_queen=false]")
            cmd("setblock 9 100 8 naturalist:ant_hill[open=false,workers=2,has_queen=false]")
            cmd("gamerule minecraft:random_tick_speed 1024")
            released = wait_until(lambda: block(7, 1) or block(7, 0), 400)
            cmd("gamerule minecraft:random_tick_speed 0")
            living = [entity for entity in ctl("entities", 12, "naturalist:ant")["entities"]
                      if entity.get("health", 0) > 0]
            print(json.dumps({"released": released, "closed_unmodified": block(9, 2, "false"),
                              "release_observed": living}, indent=2), flush=True)
            assert defended and len(ants) == 3 and all(owner in row for row in owners)
            assert len(targets) == 3 and all(threat in row for row in targets)
            assert released and block(9, 2, "false") and living
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    capture(parser.parse_args().version)
