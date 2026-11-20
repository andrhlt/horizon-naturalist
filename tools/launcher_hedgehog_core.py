"""Replay real Hedgehog hide and pet interactions in a headless stock build."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-hedgehog-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "hedgehog.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def field(entity, key):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in cmd(f"data get entity {entity} {key}")
                         if marker in row), None)

        def held(item):
            cmd("item replace entity @s weapon.mainhand with " + item)
            ctl("wait", 5)

        def click(mob):
            ctl("look-at-entity", mob)
            ctl("mouse", "right", "click")
            ctl("wait", 5)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Hedgehog Pet", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tp @s 8.5 100 6.5")
            cmd("gamemode survival @s")
            owner = field("@s", "UUID")
            assert owner
            spawned = cmd("summon naturalist:hedgehog 8.5 100 8.5 {Tags:[pet]}")
            assert any("Summoned" in row for row in spawned), spawned
            pet = "@e[tag=pet,limit=1]"
            ctl("wait", 12)
            before = ctl("status")["heard"]
            hedgehog = ctl("entities", 6, "naturalist:hedgehog")["entities"][0]["id"]
            hides = before.get("naturalist:entity.hedgehog.hide", 0) > 0
            held("minecraft:sweet_berries 20")
            after = ctl("status")["heard"]
            stopped_hiding = after.get("naturalist:entity.hedgehog.unhide", 0) > before.get("naturalist:entity.hedgehog.unhide", 0)
            for _ in range(20):
                if field(pet, "Owner") == owner:
                    break
                click(hedgehog)
            tamed = field(pet, "Owner") == owner and field(pet, "DyeColor") == "14"
            print(json.dumps({"pet_before": field(pet, ""), "heard": after}), flush=True)

            cmd(f"data merge entity {pet} {{Health:5f}}")
            before_amount = int(field("@s", "SelectedItem.count"))
            tick_health = field(pet, "Health")
            click(hedgehog)
            healed = field(pet, "Health") in {"7.0f", "7f"} and int(field("@s", "SelectedItem.count")) == before_amount - 1
            held("minecraft:purple_dye")
            click(hedgehog)
            dyed = field(pet, "DyeColor") == "10" and field("@s", "SelectedItem.id") is None
            held("minecraft:shears")
            click(hedgehog)
            cleared = field(pet, "DyeColor") == "-1" and field("@s", 'SelectedItem.components."minecraft:damage"') == "1"
            held("minecraft:air")
            click(hedgehog)
            sat = field(pet, "Sitting") == "1b"
            held("naturalist:whistle")
            click(hedgehog)
            wandering = field(pet, "FollowingOwner") == "0b" and field(pet, "Sitting") == "0b"
            cmd(f"damage {pet} 2 minecraft:cactus")
            cactus = field(pet, "Health") in {"7.0f", "7f"}
            cmd(f"damage {pet} 2 minecraft:sweet_berry_bush")
            berries = field(pet, "Health") in {"7.0f", "7f"}
            return {
                "visible_player_hides": hides,
                "held_berries_prevents_hide": stopped_hiding,
                "tamed_default_red": tamed,
                "owner_heals_two": healed,
                "purple_dye": dyed,
                "shears_clear": cleared,
                "empty_hand_sits": sat,
                "whistle_wanders_and_stands": wandering,
                "cactus_immune": cactus,
                "bush_immune": berries,
                "health_before_use": tick_health,
            }
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    actual = capture(args.version)
    print(json.dumps(actual, indent=2))
    assert actual.pop("health_before_use") in {"5.0f", "5f"}, actual
    assert all(expected[key] == value for key, value in actual.items()), actual
