"""Compare integrated Horizon bird foraging against the Java goal fixture."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-bird-goals-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "bird.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    check=True, capture_output=True, text=True, timeout=45)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def summon(entity):
            rows = cmd("summon " + entity)
            assert any("Summoned" in row for row in rows), rows

        def count(tag):
            lines = cmd(f"data get entity @e[type=minecraft:item,tag={tag},limit=1] Item.count")
            return next((int(match.group(1)) for row in lines
                         if (match := re.search(r"following entity data: (\d+)$", row))), None)

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Bird Goals Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 110 8.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 109 15 minecraft:air")
            cmd("tick freeze")
            summon('naturalist:bird 8.5 100 8.5 {PersistenceRequired:1b,Tags:[bird_forager]}')
            summon('minecraft:item 8.7 100.05 8.5 {NoGravity:1b,PickupDelay:32767s,'
                   'Tags:[bird_distractor],Item:{id:"minecraft:carrot",count:3}}')
            summon('minecraft:item 8.7 100.05 8.5 {NoGravity:1b,PickupDelay:32767s,'
                   'Tags:[bird_seeds],Item:{id:"minecraft:pumpkin_seeds",count:3}}')
            assert count("bird_seeds") == 3
            for _ in range(10):
                step(2)
                after_first = count("bird_seeds")
                if after_first != 3:
                    break
            step(20)
            after_cooldown = count("bird_seeds")
            for _ in range(16):
                step(2)
                after_second = count("bird_seeds")
                if after_second != after_cooldown:
                    break
            owner = cmd("data get entity @e[tag=bird_forager,limit=1] Owner")
            result = {
                "chooses_seeds": count("bird_distractor") == 3,
                "after_first": after_first,
                "after_cooldown": after_cooldown,
                "after_second": after_second,
                "ownerless_stays_wild": not any("following entity data" in row for row in owner),
            }
            print(json.dumps(result, indent=2), flush=True)
            assert all(reference[key] == value for key, value in result.items()), result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.reference.read_text()))
