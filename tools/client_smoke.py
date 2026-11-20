"""Check original deer models, apple interaction, and live LAN entity tracking."""
import argparse
from forest_dasher import brew
import deer_variants
import json
import re
from pathlib import Path
import subprocess
import tempfile
import time


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    for name in ("ctl", "game", "resources", "pack"):
        cli.add_argument(name, type=lambda path: Path(path).resolve(strict=True))
    cli.add_argument("screenshot", type=Path)
    cli.add_argument("--appearance", action="store_true")
    args = cli.parse_args()
    with tempfile.TemporaryDirectory(prefix="deer-clients-", ignore_cleanup_errors=True) as tmp:
        root = Path(tmp)

        def call(side, *words):
            command = [args.ctl, "--socket", root / f"{side}.sock", *map(str, words)]
            result = subprocess.run(command, capture_output=True, text=True, timeout=70)
            if result.returncode:
                raise RuntimeError(f"{side} {words}: {result.stdout} {result.stderr}")
            return json.loads(result.stdout.splitlines()[-1])

        def launch(side, *opts):
            home = root / side
            home.mkdir(exist_ok=True)
            call(side, "launch", "--game", args.game, "--resources", args.resources,
                 "--home", home, "--game-dir", home, "--dev-mod", args.pack, *opts)

        def command(text):
            return call("host", "command", "/" + text)["chat"]

        def screenshot(side, suffix):
            call(side, "look-at", 8.5, 101.72, 9.5)
            call(side, "key", "f1", "tap")
            call(side, "wait", 12)
            path = args.screenshot.resolve()
            path = path.with_name(f"{path.stem}-{suffix}{path.suffix}")
            call(side, "screenshot", path)
            call(side, "key", "f1", "tap")
            print(f"{suffix} rendered deer: {path}", flush=True)

        try:
            launch("host", "--world", "Deer Clients", "--creative", "--test-terrain",
                   "--render-distance", 4, "--wait")

            if args.appearance:
                home = root / "host" / "saves"
                worlds = [p.parent for p in home.rglob("level.dat")]
                if not worlds:
                    command("save-all")
                    worlds = [p.parent for p in home.rglob("level.dat")]
                assert len(worlds) == 1, worlds
                deer_variants.write_pack(worlds[0] / "datapacks" / deer_variants.PACK)
                call("host", "quit")
                wait_until = time.monotonic() + 10
                while (root / "host.sock").exists() and time.monotonic() < wait_until:
                    time.sleep(0.1)
                launch("host", "--world", "Deer Clients", "--creative", "--test-terrain",
                       "--render-distance", 4, "--wait")
                if not any(deer_variants.PACK in line for line in command("datapack list enabled")):
                    command(f'datapack enable "file/{deer_variants.PACK}"')
                command("gamerule minecraft:spawn_mobs false")
                command("tp @s 8.5 101 5.5")
                call("host", "wait", 30)
                command("fill 2 100 2 15 100 15 minecraft:grass_block")
                command("fill 4 101 4 14 105 14 minecraft:air")
                command("time set noon")
                command("tp @s 8.5 101 5.5")
                adult = "@e[type=naturalist:deer,tag=variant_photo,limit=1]"
                command('summon naturalist:deer 8.5 101 9.5 '
                        '{Variant:"naturalist:deer",Rotation:[120f,0f],NoAI:1b,Tags:[variant_photo]}')
                screenshot("host", "default")
                command(f'data merge entity {adult} {{Variant:"deer_fixture:woodland"}}')
                assert any("deer_fixture:woodland" in line for line in command(f"data get entity {adult} Variant"))
                screenshot("host", "host-variant")
                command("tp @s 12.5 101 5.5")
                port = call("host", "lan", "open", 0, "commands")["port"]
                launch("guest", "--render-distance", 4)
                call("guest", "connect", f"127.0.0.1:{port}", "DeerGuest")
                call("guest", "wait-for", "loaded", "--timeout", 60)
                command("tp DeerGuest 8.5 101 5.5")
                call("guest", "wait", 35)
                assert call("guest", "status")["server"]["entities"].get("naturalist:deer", 0) >= 1
                screenshot("guest", "guest-variant")
                command("tp @s 8.5 101 6.5")
                command("gamemode creative @s")
                command("item replace entity @s weapon.mainhand with naturalist:deer_spawn_egg")
                call("host", "look-at", 8.5, 101.86, 9.5)
                call("host", "wait", 6)
                call("host", "mouse", "right", "click")
                call("host", "wait", 10)
                nearby = call("host", "entities", 12, "naturalist:deer")["entities"]
                assert any(entity["baby"] for entity in nearby), nearby
                fawn = "@e[type=naturalist:deer,tag=!variant_photo,distance=..8,sort=nearest,limit=1]"
                reply = command(f"data get entity {fawn} Variant")
                assert any("deer_fixture:woodland" in line for line in reply), reply
                print("A spawn egg used on the variant adult produced an inheriting fawn", flush=True)
                return

            command("tp @s 8.5 102 6.5")
            call("host", "wait", 35)
            command("fill 2 100 2 15 100 15 minecraft:grass_block")
            command("fill 4 101 4 14 105 14 minecraft:air")
            command("time set noon")
            command("tp @s 8.5 101 6.5")
            command('summon naturalist:deer 8.5 101 9.5 {Rotation:[120f,0f],NoAI:1b,Tags:[photo_deer]}')
            command('summon naturalist:deer 10.3 101 9.5 {Age:-24000,Rotation:[120f,0f],NoAI:1b,Tags:[photo_fawn]}')
            command("tp @s 8.5 101 5.4")
            call("host", "look-at", 9.2, 101.85, 9.5)
            call("host", "key", "f1", "tap")
            call("host", "wait", 15)
            call("host", "screenshot", args.screenshot.resolve())
            call("host", "key", "f1", "tap")
            command("tp @s 8.5 101 6.5")
            print("Captured adult and fawn on a rendered frame", flush=True)

            command("gamemode survival @s")
            command("item replace entity @s weapon.mainhand with minecraft:apple 2")
            call("host", "look-at", 8.5, 101.86, 9.5)
            call("host", "wait", 6)
            call("host", "mouse", "right", "click")
            call("host", "wait", 7)
            reply = command("data get entity @e[tag=photo_deer,limit=1] InLove")
            assert any("following entity data" in line and line.split()[-1].isdigit()
                       and int(line.split()[-1]) > 0 for line in reply), reply
            assert call("host", "status")["held"] == {"item": "minecraft:apple", "count": 1}
            print("Right-click feeding used one apple and put the deer in love", flush=True)

            command("gamemode creative @s")
            command("item replace entity @s weapon.mainhand with naturalist:deer_spawn_egg")
            command("setblock 7 101 8 minecraft:stone")
            call("host", "look-at", "block", 7, 101, 8, "north")
            call("host", "wait", 6)
            call("host", "mouse", "right", "click")
            call("host", "wait", 10)
            deer = call("host", "entities", 32, "naturalist:deer")["entities"]
            assert len(deer) >= 3, deer
            print("Deer spawn egg spawned a third deer", flush=True)

            port = call("host", "lan", "open", 0, "commands")["port"]
            launch("guest", "--headless", "--render-distance", 4)
            call("guest", "connect", f"127.0.0.1:{port}", "DeerGuest")
            call("guest", "wait-for", "loaded", "--timeout", 60)
            command("tp DeerGuest 8.5 101 6.5")
            deadline = time.monotonic() + 8
            while time.monotonic() < deadline:
                entities = call("guest", "status")["server"]["entities"]
                if entities.get("naturalist:deer", 0) >= 2:
                    break
                time.sleep(0.15)
            else:
                raise AssertionError(f"Guest did not receive the deer: {entities}")
            print("Live LAN guest received the adult and fawn with the custom entity ID", flush=True)
            drops = command("loot give @s kill @e[type=naturalist:deer,tag=photo_deer,limit=1]")
            collected = command("data get entity @s Inventory")
            assert any("naturalist:venison" in line for line in collected), (drops, collected)
            print("Deer loot command awarded Naturalist venison", flush=True)
            command("kill @e[type=naturalist:deer,tag=photo_deer,limit=1]")
            call("host", "wait", 35)
            drops = call("host", "entities", 32, "minecraft:item")["entities"]
            assert any(drop["item"]["item"] == "naturalist:venison" for drop in drops), drops
            print("Adult deer also dropped venison on death", flush=True)

            command("setblock 6 101 7 minecraft:furnace")
            seeded = command(
                'data merge block 6 101 7 {Items:[{Slot:0b,id:"naturalist:venison",count:1},'
                '{Slot:1b,id:"minecraft:coal",count:1}],cooking_time_spent:195,cooking_total_time:200}'
            )
            call("host", "wait", 30)
            cooked = command("data get block 6 101 7 Items")
            assert any("naturalist:cooked_venison" in line for line in cooked), (seeded, cooked)
            print("A furnace cooked raw venison into cooked venison", flush=True)

            command("tick freeze")
            brew(command, 12, 101, 11)
            moved = command(
                "item replace entity @s weapon.mainhand from block 12 101 11 container.0"
            )
            held = call("host", "status")["held"]
            assert held and held["item"] == "minecraft:potion", (moved, held)
            guest_moved = command(
                "item replace entity DeerGuest weapon.mainhand from block 12 101 11 container.1"
            )
            assert any("Replaced" in line for line in guest_moved), guest_moved
            command("tick unfreeze")
            call("guest", "wait", 15)
            items = call("guest", "menu")["slots"]
            assert "naturalist:strong_forest_dasher" in json.dumps(items), items
            print("LAN guest received the strong splash potion with its registry ID", flush=True)
            command("gamemode survival @s")
            call("host", "look-at", 8.5, 105, 5.5)
            command("tick unfreeze")
            call("host", "mouse", "right", "down")
            call("host", "wait", 38)
            call("host", "mouse", "right", "up")
            command("tick freeze")
            effects = " ".join(command("data get entity @s active_effects"))
            for effect, amplifier in (("speed", 2), ("weakness", 1)):
                fields = next((entry for entry in re.findall(r"\{([^{}]+)\}", effects)
                               if f'id: "minecraft:{effect}"' in entry), "")
                level = re.search(r"amplifier: (\d+)b?", fields)
                duration = re.search(r"duration: (\d+)", fields)
                assert level and int(level[1]) == amplifier, effects
                assert duration and 380 <= int(duration[1]) <= 400, effects
            print(f"Drinking strong Forest Dasher applied both effects in-game: {effects}", flush=True)

        finally:
            for player in ("guest", "host"):
                try:
                    call(player, "quit")
                except (RuntimeError, subprocess.TimeoutExpired):
                    pass


if __name__ == "__main__":
    main()
