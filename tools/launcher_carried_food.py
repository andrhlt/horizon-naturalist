"""Compare a standalone carried apple with the pinned Naturalist Java reference."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, water, stress=False):
    with tempfile.TemporaryDirectory(prefix="nat-carried-item-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "test.sock"

        def control(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=60)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(text):
            return control("command", text)["chat"]

        def step(ticks):
            command(f"tick step {ticks}")
            control("wait", ticks + 2)

        def field(entity, key):
            marker = " following entity data: "
            return next((line.split(marker, 1)[1] for line in command(f"data get entity {entity} {key}")
                         if marker in line), None)

        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3", "--home", home,
                    "--game-dir", home, "--world", "Carried Food", "--creative",
                    "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            control("tp", 8.5, 101, 14.5)
            control("wait", 15)
            command("fill 4 99 4 12 99 12 minecraft:stone")
            medium = "minecraft:water" if water else "minecraft:air"
            command(f"fill 4 100 4 12 104 12 {medium}")
            command("kill @e[type=minecraft:item]")
            command("tick freeze")
            entity = "@e[type=naturalist:carried_food,tag=probe,limit=1]"
            empty = command("summon naturalist:carried_food 8.5 100 8.5 {Tags:[probe]}")
            assert any("Summoned new Air" in line for line in empty), empty
            assert field(entity, "UUID") is None
            result = command('summon naturalist:carried_food 8.5 100 8.5 {Item:{id:"minecraft:apple",count:2},Tags:[probe]}')
            assert any("Summoned new Apple" in line for line in result), result
            watched = ("Pos", "Motion", "Item.id", "Item.count", "Age", "PickupDelay")
            before = {key: field(entity, key) for key in watched}
            assert before["Item.id"] == '"minecraft:apple"' and before["Item.count"] == "2", before
            assert before["Age"] == "0s" and before["PickupDelay"] == "0s", before
            ordinary = "@e[type=minecraft:item,tag=ordinary,limit=1]"
            if stress:
                for index in range(260):
                    x, z = 5.5 + (index % 8), 5.5 + (index % 7)
                    command(f'summon minecraft:item {x} 103 {z} {{Item:{{id:"minecraft:stick",count:1}},PickupDelay:32767s}}')
                command('summon minecraft:item 9.5 100 8.5 {Item:{id:"minecraft:coal",count:1},PickupDelay:32767s,Tags:[ordinary]}')
                vanilla_before = field(ordinary, "Pos")
                assert vanilla_before is not None
            step(1)
            if stress:
                assert field(ordinary, "Pos") != vanilla_before, "neighbor did not tick"
                command("kill @e[type=minecraft:item]")
            after_one = {key: field(entity, key) for key in watched}
            assert after_one == before, (before, after_one)
            step(59)
            at_60 = field(entity, "UUID")
            assert at_60, "original exists at tick 60"
            step(1)
            at_61 = field(entity, "UUID")
            assert at_61 is None, "original is gone at tick 61"
            item = "@e[type=minecraft:item,limit=1]"
            released = {key: field(item, key) for key in watched}
            assert released["Item.id"] == before["Item.id"] and released["Item.count"] == "2", released
            assert released["PickupDelay"] == "10s" and released["Age"] == "0s", released
            assert released["Pos"] == before["Pos"], released
            return {"before": before, "tick_one": after_one, "present_at_60": True,
                    "present_at_61": False, "released": released}
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--water", action="store_true")
    parser.add_argument("--stress", action="store_true")
    args = parser.parse_args()
    print(json.dumps(capture(args.version, args.water, args.stress), indent=2), flush=True)
