"""Verify owner and non-owner Capybara interactions over headless LAN."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def call(side, *args):
            env = {**os.environ, "HORIZON_HOME": str(root / side)}
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                check=True, capture_output=True, text=True, timeout=65, env=env)
            return json.loads(reply.stdout.splitlines()[-1])

        def launch(side, *options):
            home = root / side
            home.mkdir()
            resources = Path.home() / "Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3"
            shared_jars = Path.home() / "Library/Application Support/horizon/jars"
            if shared_jars.exists():
                shutil.copytree(shared_jars, home / "jars", dirs_exist_ok=True)
            call(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", resources, "--home", home,
                 "--game-dir", home, "--render-distance", 4, *options)

        def command(text):
            return call("host", "command", text)["chat"]

        def data(selector, field):
            rows = command(f"data get entity {selector} {field}")
            return next((row.split("following entity data: ", 1)[1]
                         for row in rows if "following entity data:" in row), None)

        def live_pet():
            rows = call("host", "entities", 32, "naturalist:capybara")["entities"]
            assert len(rows) == 1, rows
            return rows[0]

        def set_hand(player, item, count=1):
            command(f"item replace entity {player} weapon.mainhand with {item} {count}")

        def click(side):
            call(side, "look-at", 8.5, 101.55, 8.5)
            call(side, "wait", 2)
            target = call(side, "looking-at")
            assert target.get("entity") and target["entity"].get("type") == "naturalist:capybara", target
            call(side, "mouse", "right", "click")
            call("host", "wait", 5)

        def flag(target, field):
            return data(target, field) in ("true", "1", "1b")

        try:
            launch("host", "--world", "Naturalist Capybara LAN Owners", "--creative",
                   "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:natural_regeneration false")
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
            # Java's owner-following setup leaves the pet unsat; newly tamed
            # runtime pets start seated, so use the owner interaction here.
            set_hand("@s", "minecraft:air")
            click("host")
            command(f"damage {pet} 10 minecraft:generic")
            call("host", "wait", 2)
            setup_health = float(live_pet()["health"])
            assert abs(setup_health - 4.0) < 0.01, live_pet()
            print("CAPYBARA_PLAYERS", json.dumps({"host": data("@s", "UUID"), "guest": data("CapybaraGuest", "UUID")}), flush=True)
            print("CAPYBARA_SETUP", json.dumps({"health": live_pet()["health"], **{field: data(pet, field) for
                  field in ("Age", "Owner", "Tame", "FollowingOwner", "Sitting", "InLove")}}), flush=True)
            assert data(pet, "Owner") == data("@s", "UUID")
            assert data(pet, "Owner") != data("CapybaraGuest", "UUID")
            sitting_before_guest = flag(pet, "Sitting")
            set_hand("CapybaraGuest", "minecraft:air")
            click("guest")
            print("CAPYBARA_AFTER_GUEST_FOOD", json.dumps({"health": live_pet()["health"], **{field: data(pet, field) for
                  field in ("Age", "Owner", "Tame", "InLove")}}), flush=True)
            sample = {"guest_toggles_sit": flag(pet, "Sitting") != sitting_before_guest}
            set_hand("CapybaraGuest", "naturalist:whistle")
            click("guest")
            sample["guest_toggles_follow"] = not flag(pet, "FollowingOwner")
            set_hand("CapybaraGuest", "minecraft:melon_slice", 2)
            click("guest")
            print("CAPYBARA_AFTER_GUEST_FEED", json.dumps({"health": live_pet()["health"], **{field: data(pet, field) for
                  field in ("Age", "Owner", "Tame", "InLove")}}), flush=True)
            sample["guest_heals"] = live_pet()["health"] > 4.0
            sample["guest_breeds"] = int(data(pet, "InLove") or "0") > 0
            sample["guest_food_consumed"] = "count: 1" in str(data("CapybaraGuest", "SelectedItem"))
            set_hand("@s", "minecraft:melon_slice", 2)
            click("host")
            print("CAPYBARA_AFTER_OWNER_FOOD", json.dumps({"health": live_pet()["health"], **{field: data(pet, field) for
                  field in ("Age", "Owner", "Tame", "InLove", "Sitting")}}), flush=True)
            sample["owner_heals"] = live_pet()["health"] == 6.0
            sitting_before_owner = flag(pet, "Sitting")
            set_hand("@s", "minecraft:air")
            print("CAPYBARA_OWNER_EMPTY_HAND", data("@s", "SelectedItem"), flush=True)
            click("host")
            print("CAPYBARA_OWNER_SIT_TOGGLE", json.dumps({"before": sitting_before_owner,
                  "after": flag(pet, "Sitting"), "health": live_pet()["health"]}), flush=True)
            sample["owner_toggles_sit"] = flag(pet, "Sitting") != sitting_before_owner
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
