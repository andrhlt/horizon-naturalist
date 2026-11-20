"""Verify that the Capture Net round-trips both Java capture branches through survival interaction."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, fixture):
    with tempfile.TemporaryDirectory(prefix="nat-capture-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "capture.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def data(target, nbt):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {target} {nbt}")
                         if "following entity data: " in row), None)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Capture Net Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 7.5 100 6.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 7.5 100 6.5")
            cmd("gamemode survival @s")
            def round_trip(species, item, variant, full):
                cmd("clear @s")
                tag = f"capture_{species}"
                mob = f"@e[tag={tag},limit=1]"
                new_mob = f"@e[type=naturalist:{species},limit=1]"
                cmd(f'summon naturalist:{species} 7.5 100 8.5 {{Tags:[{tag}],NoAI:1b,Health:1.5f,'
                    f'CustomName:"Beacon",Variant:"naturalist:{variant}"}}')
                before_uuid = data(mob, "UUID")
                assert before_uuid, species
                cmd("item replace entity @s weapon.mainhand with naturalist:capture_net")
                ctl("wait", 4)
                ctl("look", 0, 32)
                ctl("mouse", "right", "click")
                ctl("wait", 4)
                saved = data("@s", "Inventory") or ""
                assert f"naturalist:{item}" in saved and '"minecraft:custom_data"' in saved, saved
                assert '"minecraft:damage": 1' in saved and data(mob, "UUID") is None, saved
                result = {
                    f"{species}_saves_id": 'id: "naturalist:' + species + '"' in saved.split('"minecraft:custom_data"', 1)[1],
                }
                if species == "butterfly":
                    result.update({
                        "butterfly_saves_uuid": before_uuid in saved,
                        "butterfly_saves_variant": f"naturalist:{variant}" in saved,
                        "butterfly_saves_no_ai": "NoAI: 1b" in saved or "NoAI: true" in saved,
                        "butterfly_saves_health": "Health: 1.5f" in saved,
                        "butterfly_saves_name_component": '"minecraft:custom_name"' in saved and "Beacon" in saved,
                    })
                ctl("select", f"naturalist:{item}")
                ctl("look-at-block", 7, 99, 8)
                ctl("mouse", "right", "click")
                ctl("wait", 4)
                uuid = data(new_mob, "UUID")
                assert uuid and ((uuid == before_uuid) == full), (species, before_uuid, uuid)
                assert variant in (data(new_mob, "Variant") or ""), species
                assert data(new_mob, "Health") == "1.5f", species
                assert data(new_mob, "PersistenceRequired") in ("1b", "true", "1"), species
                if not full:
                    assert "Beacon" in (data(new_mob, "CustomName") or ""), species
                    assert data(new_mob, "FromHand") in ("1b", "true", "1"), species
                cmd(f"kill @e[type=naturalist:{species}]")
                ctl("wait", 22)
                return result

            observations = {}
            observations.update(round_trip("butterfly", "butterfly", "red_admiral", False))
            observations.update(round_trip("dragonfly", "dragonfly", "green", True))
            print(json.dumps(observations, indent=2), flush=True)
            assert all(value == fixture[key] for key, value in observations.items()), observations
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
