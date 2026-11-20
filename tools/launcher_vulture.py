"""Compare Vulture combat, scavenging and flight in the headless Horizon launcher build."""
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

            command("summon naturalist:vulture 8.5 100 8.5 {Tags:[immune],NoAI:1b}")
            bird = specimen("immune")
            assert health(bird) == 14
            command(f"damage {bird} 1 minecraft:cactus")
            output["cactus_immune"] = health(bird) == 14
            command(f"effect give {bird} minecraft:hunger 10")
            output["hunger_immune"] = "hunger" not in data(bird, "active_effects")
            command(f"damage {bird} 1 minecraft:generic at 7.5 101 8.5")
            impulse = vector(bird, "Motion")
            output["startle"] = health(bird) == 13 and len(impulse) == 3 and all(
                abs(actual - expected) < 1e-4 for actual, expected in zip(impulse, (0.4, 0.5, 0.0)))
            command(f"kill {bird}")

            command("summon naturalist:vulture 6.5 100 5.5 {Tags:[hunter],PersistenceRequired:1b}")
            command("summon minecraft:zombie 6.5 100 6.15 {Tags:[prey],NoAI:1b,PersistenceRequired:1b}")
            prey = specimen("prey")
            for _ in range(50):
                step(1)
                if health(prey) < 20.0:
                    break
            output["zombie_damage"] = round(20.0 - health(prey), 1)
            command(f"kill {prey}")
            command(f"kill {specimen('hunter')}")

            command("summon naturalist:vulture 6.5 100 5.5 {Tags:[scavenger],PersistenceRequired:1b,CanPickUpLoot:1b}")
            bird = specimen("scavenger")
            command('summon minecraft:item 6.5 100.9 5.5 {Tags:[flesh],PickupDelay:300s,Item:{id:"minecraft:rotten_flesh",count:3}}')
            step(1)
            gear = data(bird, "equipment.mainhand")
            output["took_rotten_flesh"] = 3 if "rotten_flesh" in gear and "count: 3" in gear else 0
            step(599)
            before = data(bird, "equipment.mainhand")
            step(1)
            after = data(bird, "equipment.mainhand")
            output["eats_at_tick_601"] = "count: 3" in before and "count: 2" in after
            command(f"kill {bird}")
            command("kill @e[type=minecraft:item,distance=..12]")

            command("fill 0 99 0 35 99 15 minecraft:stone")
            command("fill 0 100 0 35 109 15 minecraft:air")
            command("tp @s 28.5 100 8.5")
            command("gamemode survival @s")
            command("damage @s 14 minecraft:generic")
            command("item replace entity @s weapon.mainhand with minecraft:stick 3")
            command("summon naturalist:vulture 8.5 100 9.1 {Tags:[thief],PersistenceRequired:1b}")
            bird = specimen("thief")
            # Acquire the attack target outside the competing flee goal's range.
            command(f"attribute {bird} minecraft:follow_range base set 48")
            step(28)
            for _ in range(12):
                if "minecraft:stick" in data(bird, "equipment.mainhand"):
                    break
                x, y, z = vector(bird, "Pos")
                command(f"tp @s {x + 0.55} {y} {z}")
                step(3)
            pack = data("@s", "Inventory")
            gear = data(bird, "equipment.mainhand")
            output["steals_one_without_damage"] = health("@s") == 6.0 and "minecraft:stick" in pack \
                and "count: 2" in pack and "minecraft:stick" in gear and "count: 1" in gear
            print(json.dumps(output, indent=2), flush=True)
            assert output == fixture, (output, fixture)
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
