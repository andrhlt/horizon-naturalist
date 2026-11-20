"""Compare Vulture equipment drops and experience in the headless Horizon launcher build."""
import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path


def capture(version, fixture):
    with tempfile.TemporaryDirectory(prefix="nat-vulture-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "vulture.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def data(target, nbt):
            return next((row.split("following entity data: ", 1)[1]
                         for row in command(f"data get entity {target} {nbt}")
                         if "following entity data: " in row), "")

        def step(ticks):
            command(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        def specimen(tag):
            return f"@e[tag={tag},limit=1]"

        def health(target):
            return float(data(target, "Health").rstrip("f"))

        def vector(target, nbt):
            return tuple(float(v) for v in re.findall(r"(-?\d+(?:\.\d+)?(?:e[+-]?\d+)?)d", data(target, nbt)))

        output = {}
        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Vulture Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:natural_health_regeneration false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 8.5")
            command("time set midnight")
            command("tick freeze")

            command("gamerule minecraft:mob_drops false")
            command("summon naturalist:vulture 6.5 100 8.5 {Tags:[loot],NoAI:1b}")
            bird = specimen("loot")
            command(f"item replace entity {bird} weapon.mainhand with minecraft:stick 3")
            command(f"item replace entity {bird} armor.head with minecraft:iron_helmet")
            command(f"damage {bird} 100 minecraft:generic")
            step(1)
            drops = ctl("entities", 8, "minecraft:item")["entities"]
            stacks = {row["item"]["item"]: row["item"]["count"] for row in drops}
            output = {
                "stick_count": stacks.get("minecraft:stick", 0),
                "helmet_count": stacks.get("minecraft:iron_helmet", 0),
                "only_equipment": len(drops) == 2,
            }
            command("gamerule minecraft:mob_drops true")
            command("summon naturalist:vulture 11.5 100 8.5 {Tags:[xp],NoAI:1b}")
            command(f"damage {specimen('xp')} 100 minecraft:player_attack by @s")
            step(1)
            output["base_experience"] = sum(row["value"] for row in ctl("entities", 8, "minecraft:experience_orb")["entities"])
            print(json.dumps({"actual": output, "drops": drops}, indent=2), flush=True)
            assert all(value == fixture[key] for key, value in output.items()), output
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
