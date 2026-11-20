"""Replay the Java Elephant inventory interactions on a stock headless client."""
import argparse
import json
import re
from pathlib import Path
import subprocess
import tempfile


def capture(version, guest, reference, check_preview=False):
    with tempfile.TemporaryDirectory(prefix="nat-elephant-menu-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        actor = "NaturalistGuest" if guest else "@s"

        def control(side, *args):
            call = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(call.stdout.splitlines()[-1])

        def launch(side, *args):
            home = root / side
            home.mkdir()
            return control(side, "launch", "--headless", "--game", version / "horizon-client",
                           "--resources", version / ".local/horizon-client/26.3",
                           "--home", home, "--game-dir", home, "--render-distance", 4, *args)

        def command(line):
            return control("host", "command", line)["chat"]

        def field(target, name):
            prefix = " following entity data: "
            return next((row.split(prefix, 1)[1] for row in command(f"data get entity {target} {name}")
                         if prefix in row), None)

        client = "guest" if guest else "host"

        def open_menu(x):
            control(client, "look-at", x, 102.4, 8.5)
            assert control(client, "looking-at")["kind"] == "entity"
            control(client, "key", "shift", "down")
            try:
                control(client, "mouse", "right", "click")
                control(client, "wait", 8)
            finally:
                control(client, "key", "shift", "up")
            menu = control(client, "menu")
            assert menu["screen"] == "inventory", menu
            return menu

        try:
            launch("host", "--world", "Naturalist Elephant Menu", "--creative", "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 7.5 101 5.5")
            control("host", "wait", 20)
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("fill 1 101 1 15 110 15 minecraft:air")
            if guest:
                port = control("host", "lan", "open", 0, "commands")["port"]
                launch("guest")
                control("guest", "connect", f"127.0.0.1:{port}", actor)
                control("guest", "wait-for", "loaded", "--timeout", 55)
                command("gamemode spectator @s")
            command(f"gamemode survival {actor}")
            results = {}
            for species in ("elephant", "mammoth"):
                command(f"tp {actor} 8.5 101 5.5")
                owner = field(actor, "UUID")
                assert owner, actor
                mob = f"@e[type=naturalist:{species},tag=inventory_qa,limit=1]"
                command(f"summon naturalist:{species} 8.5 101 8.5 "
                        f"{{Owner:{owner},NoAI:1b,Chested:1b,Tags:[inventory_qa]}}")
                command(f"clear {actor}")
                command(f"item replace entity {actor} inventory.0 with minecraft:stone 19")
                command(f"item replace entity {actor} inventory.1 with minecraft:red_banner 2")
                command(f"item replace entity {actor} inventory.2 with minecraft:dirt 3")
                control(client, "wait", 10)
                menu = open_menu(8.5)
                print(species, "menu", json.dumps(menu), flush=True)
                if check_preview:
                    geometry = control(client, "elephant-menu-geometry")
                    print(species, "inset", json.dumps(geometry), flush=True)
                    assert geometry["drawn"] and geometry["inset_vertices"] > 12, geometry
                    assert geometry["responds_to_cursor"], geometry
                for slot in (9, 10):
                    assert control(client, "click-slot", slot, "shift").get("ok", True)
                    control(client, "wait", 8)
                control(client, "click-slot", 11)
                control(client, "click-slot", 325)
                control(client, "click-slot", 326)
                control(client, "wait", 6)
                storage = field(mob, "Items")

                player_storage = field(actor, "Inventory")
                print(species, "stored", storage, "player", player_storage, flush=True)
                results[species] = {
                    "stone_in_first_slot": bool(storage and "Slot: 0b" in storage and "count: 19" in storage),
                    "one_banner_in_slot": bool(storage and "Slot: 26b" in storage and "red_banner" in storage),
                    "other_banner_in_chest": bool(storage and re.search(
                        rf'\{{count: {reference[species]["custom_inventory"]["shift_banner_in_chest"]}, Slot: 1b, id: "minecraft:red_banner"\}}',
                        storage
                    )),
                    "banner_source_is_empty": bool(player_storage and "red_banner" not in player_storage),
                    "both_equipment_slots_reject_dirt": bool(storage and "minecraft:dirt" not in storage),
                }
                control(client, "click-slot", 11)
                control(client, "key", "esc", "down")
                control(client, "key", "esc", "up")
                control(client, "wait", 4)
                command(f"kill {mob}")
            return results
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    control(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--guest", action="store_true")
    parser.add_argument("--preview", action="store_true")
    args = parser.parse_args()
    results = capture(args.version, args.guest, json.loads(args.reference.read_text()), args.preview)
    print(json.dumps(results, indent=2), flush=True)
    assert all(all(row.values()) for row in results.values()), results
