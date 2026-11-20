"""Compare configured deer spawners under headless Play."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def replay(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-step-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "step.sock"

        def control(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    check=True, capture_output=True, text=True, timeout=45)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return control("command", line)["chat"]

        def heard():
            return control("status")["heard"]

        try:
            control("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                    "--world", "Naturalist Deer Spawner", "--creative", "--test-terrain",
                    "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs true")
            print(command("gamerule minecraft:spawner_blocks_work true"), flush=True)
            command("tp @s 8.5 106 8.5")
            control("wait", 25)
            command("fill 0 101 0 15 111 15 minecraft:air")
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("tp @s 8.5 101 13.5")
            command("time set noon")
            command("tick freeze")
            cases = ((3, "grass_block", False, "minecraft:pig"),
                     (3, "grass_block", False, "naturalist:deer"),
                     (8, "dirt", False, "naturalist:deer"),
                     (13, "stone", False, "naturalist:deer"),
                     (13, "stone", True, "naturalist:deer"))
            for x, floor, custom, kind in cases:
                command(f"kill @e[type=naturalist:deer]")
                command("kill @e[type=minecraft:pig]")
                command(f"fill {x-1} 100 5 {x+1} 100 9 minecraft:{floor}")
                command("tp @s 8.5 101 13.5")
                command("setblock 8 101 11 minecraft:air")
                options = ",custom_spawn_rules:{block_light_limit:[0,15],sky_light_limit:[0,15]}" if custom else ""
                spawner = f'{{Delay:1s,MinSpawnDelay:100s,MaxSpawnDelay:100s,SpawnCount:1s,SpawnRange:1s,SpawnData:{{entity:{{id:"{kind}",Pos:[{x}.5d,101.0d,7.5d],NoAI:1b}}{options}}}}}'
                feedback = command(f"setblock 8 101 11 minecraft:spawner{spawner}")
                assert any("Changed" in line for line in feedback), feedback
                command("tick step 30")
                control("wait", 35)
                mobs = control("entities", 18, kind)["entities"]
                delay = command("data get block 8 101 11 Delay")
                spawned = bool(mobs)
                print(json.dumps({"kind": kind, "floor": floor, "custom": custom,
                                  "spawned": spawned, "delay": delay}), flush=True)
                expected = True if kind == "minecraft:pig" or custom else reference[floor]
                assert spawned == expected, (kind, floor, custom, spawned, expected)
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    replay(args.version, json.loads(args.reference.read_text()))
