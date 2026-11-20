"""Check actual guest-only Ostrich mounting, attachment, travel and jumping."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, expected_offset):
    with tempfile.TemporaryDirectory(prefix="nat-ostrich-lan-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)

        def ctl(side, *args):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                capture_output=True, text=True, timeout=75,
            )
            assert result.returncode == 0, (side, args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def launch(side, *args):
            home = root / side
            home.mkdir()
            ctl(side, "launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--render-distance", 4, *args)

        def cmd(text):
            return ctl("host", "command", text)["chat"]

        def field(entity, name):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in cmd(f"data get entity {entity} {name}")
                         if marker in row), None)

        def position(target):
            numbers = re.findall(r"-?\d+(?:\.\d+)?", field(target, "Pos") or "")
            assert len(numbers) == 3, target
            return [float(number) for number in numbers]

        target = "@e[type=naturalist:ostrich,tag=lan_mount,limit=1]"
        guest_name = "OstrichGuest"

        def interact(side):
            entities = ctl(side, "entities", 6, "naturalist:ostrich")["entities"]
            assert len(entities) == 1, entities
            ctl(side, "look-at-entity", entities[0]["id"])
            ctl(side, "mouse", "right", "click")
            ctl("host", "wait", 7)

        try:
            launch("host", "--world", "Ostrich LAN", "--creative", "--test-terrain", "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 104 6.5")
            ctl("host", "wait", 15)
            cmd("fill 0 99 0 15 99 29 minecraft:stone")
            cmd("fill 0 100 0 15 107 29 minecraft:air")
            cmd("tp @s 8.5 100 6.5")
            port = ctl("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            ctl("guest", "connect", f"127.0.0.1:{port}", guest_name)
            ctl("guest", "wait-for", "loaded", "--timeout", 55)
            cmd(f"tp {guest_name} 8.5 100 6.0")
            cmd(f"gamemode survival {guest_name}")
            cmd("gamemode survival @s")
            owner = field(guest_name, "UUID")
            summon = cmd(f"summon naturalist:ostrich 8.5 100 8.5 "
                         f"{{Tags:[lan_mount],Owner:{owner},Saddled:1b,NoAI:1b,Rotation:[0f,0f]}}")
            assert any("Summoned" in message for message in summon), summon
            ctl("guest", "wait", 10)
            interact("host")
            host_denied = ctl("host", "status").get("riding") is None
            cmd("tp @s 6.5 100 6.5")
            interact("guest")
            guest_mounted = ctl("guest", "status").get("riding") is not None
            seat = [player - mob for player, mob in zip(position(guest_name), position(target))]
            matches_seat = all(abs(actual - java) < 0.05 for actual, java in zip(seat, expected_offset, strict=True))
            print(json.dumps({"host_denied": host_denied, "guest_mounted": guest_mounted, "seat": seat}), flush=True)
            normal_inventory = False
            if guest_mounted:
                ctl("guest", "key", "e", "down")
                ctl("guest", "key", "e", "up")
                ctl("guest", "wait", 5)
                menu = ctl("guest", "menu")
                saddle_slot = any(slot["slot"] == 700 for slot in menu.get("slots", []))
                normal_inventory = menu.get("screen") == "inventory" and not saddle_slot
                print(json.dumps({"mounted_inventory": menu, "mount_saddle_slot": saddle_slot}), flush=True)
                if menu.get("screen") == "inventory":
                    ctl("guest", "key", "esc", "down")
                    ctl("guest", "key", "esc", "up")
            cmd(f"data merge entity {target} {{NoAI:0b}}")
            ctl("guest", "look-at", 8.5, 102.5, 28.5)
            start = position(target)
            ctl("guest", "key", "w", "down")
            ctl("guest", "wait", 25)
            ctl("guest", "key", "w", "up")
            ctl("host", "wait", 5)
            walked = position(target)
            jump_sound = "minecraft:entity.donkey.jump"
            before_jump = ctl("guest", "status").get("heard", {}).get(jump_sound, 0)
            ctl("guest", "key", "space", "down")
            ctl("guest", "wait", 10)
            ctl("guest", "key", "space", "up")
            y = [position(target)[1] for _ in range(12)]
            ctl("guest", "key", "shift", "down")
            ctl("guest", "wait", 5)
            ctl("guest", "key", "shift", "up")
            after_jump = ctl("guest", "status").get("heard", {}).get(jump_sound, 0)
            print(json.dumps({"start": start, "walked": walked, "jump_y": y,
                              "donkey_jump_sounds": after_jump - before_jump}), flush=True)
            dismounted = ctl("guest", "status").get("riding") is None
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("setblock 8 100 23 naturalist:ostrich_egg")
            packed = (8 << 38) | (23 << 12) | 100
            guard = "@e[type=naturalist:ostrich,tag=guard,limit=1]"
            cmd(f"summon naturalist:ostrich 12.5 100 23.5 {{Tags:[guard],OwnedEggs:[L;{packed}L],NoAI:1b}}")
            cmd(f"tp {guest_name} 8.5 100 21.5")
            cmd(f"item replace entity {guest_name} weapon.mainhand with minecraft:diamond_pickaxe")
            ctl("guest", "wait", 6)
            ctl("guest", "look-at", 8.5, 100.48, 23.5)
            ctl("guest", "mouse", "left", "down")
            ctl("guest", "wait", 20)
            ctl("guest", "mouse", "left", "up")
            cmd("tick freeze")
            gone = any("Test passed" in row for row in cmd("execute if block 8 100 23 minecraft:air"))
            targeted_player = field(guard, "angry_at")
            owned_eggs = field(guard, "OwnedEggs")
            print(json.dumps({"guest_broke_egg": gone, "angry_at": targeted_player,
                              "guest_uuid": owner, "owned_eggs_after": owned_eggs}), flush=True)
            return {"host_denied": host_denied, "guest_mounted": guest_mounted, "seat_matches_java": matches_seat,
                    "guest_steered": walked[2] - start[2] > 0.5, "guest_jumped": max(y) - walked[1] > 0.25,
                    "dismounted": dismounted,
                    "normal_player_inventory": normal_inventory, "no_donkey_jump_sound": before_jump == after_jump,
                    "guest_destroys_owned_egg": gone and targeted_player == owner and owned_eggs == "[L;]"}
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    ctl(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.reference.read_text())["rider_seat_offset"]
    actual = capture(args.version, expected)
    print(json.dumps(actual, indent=2), flush=True)
    assert all(actual.values()), actual
