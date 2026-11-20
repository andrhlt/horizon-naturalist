"""Verify owner and non-owner Capybara interactions over headless LAN."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def call(side, *args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                check=True, capture_output=True, text=True, timeout=65)
            return json.loads(reply.stdout.splitlines()[-1])

        def launch(side, *options):
            home = root / side
            home.mkdir()
            call(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--render-distance", 4, *options)

        def command(text):
            return call("host", "command", text)["chat"]

        def data(selector, field):
            rows = command(f"data get entity {selector} {field}")
            return next((row.split("following entity data: ", 1)[1]
                         for row in rows if "following entity data:" in row), None)

        def set_hand(player, item, count=1):
            command(f"item replace entity {player} weapon.mainhand with {item} {count}")

        def click(side):
            call(side, "look-at", 8.5, 101.55, 8.5)
            call(side, "mouse", "right", "click")
            call("host", "wait", 5)

        def flag(target, field):
            return data(target, field) in ("true", "1", "1b")

        try:
            launch("host", "--world", "Naturalist Capybara LAN Owners", "--creative",
                   "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("forceload add 0 0")
            command("tp @s 8.5 101 6.0")
            call("host", "wait", 20)
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("fill 1 101 1 15 106 15 minecraft:air")
            command("tp @s 8.5 101 6.0")
            port = call("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            call("guest", "connect", f"127.0.0.1:{port}", "CapybaraGuest")
            call("guest", "wait-for", "loaded", "--timeout", 55)
            command("tp CapybaraGuest 8.5 101 10.5")
            command("gamemode survival CapybaraGuest")
            command("gamemode survival @s")
            pet = "@e[type=naturalist:capybara,tag=lan_pet,limit=1]"
            summon = command("summon naturalist:capybara 8.5 101 8.5 "
                             "{Tags:[lan_pet],NoAI:1b}")
            assert any("Summoned" in row for row in summon), summon
            call("guest", "wait", 12)
            set_hand("@s", "minecraft:melon_slice", 32)
            for _ in range(12):
                click("host")
                if data(pet, "Owner") is not None:
                    break
            assert data(pet, "Owner") is not None
            command(f"data merge entity {pet} {{Sitting:0b,FollowingOwner:1b,Health:4.0f,InLove:0}}")
            assert data(pet, "Owner") == data("@s", "UUID")
            assert data(pet, "Owner") != data("CapybaraGuest", "UUID")
            set_hand("CapybaraGuest", "minecraft:air")
            click("guest")
            sample = {"guest_toggles_sit": flag(pet, "Sitting")}
            set_hand("CapybaraGuest", "naturalist:whistle")
            click("guest")
            sample["guest_toggles_follow"] = not flag(pet, "FollowingOwner")
            set_hand("CapybaraGuest", "minecraft:melon_slice", 2)
            click("guest")
            sample["guest_heals"] = float(data(pet, "Health").rstrip("f")) > 4.0
            sample["guest_breeds"] = int(data(pet, "InLove") or "0") > 0
            sample["guest_food_consumed"] = "count: 1" in str(data("CapybaraGuest", "SelectedItem"))
            set_hand("@s", "minecraft:melon_slice", 2)
            click("host")
            sample["owner_heals"] = float(data(pet, "Health").rstrip("f")) == 6.0
            set_hand("@s", "minecraft:air")
            click("host")
            sample["owner_toggles_sit"] = flag(pet, "Sitting")
            return sample
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    call(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    actual = capture(options.version)
    expected = json.loads(options.fixture.read_text())["capybara_owner"]
    print(json.dumps(actual, indent=2), flush=True)
    assert actual == expected, (actual, expected)
