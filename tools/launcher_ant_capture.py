"""Verify wild and owned Ant capture/release against Java-selected hand traits."""
import argparse
import hashlib
import shutil
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, jar):
    with tempfile.TemporaryDirectory(prefix="nat-capture-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "capture.sock"
        if jar is not None:
            from prepare import JAR_SHA
            if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
                raise ValueError("expected the pinned Naturalist 2.0.7 NeoForge 26.3 jar")
            (directory / "jars").mkdir()
            shutil.copyfile(jar, directory / "jars" / f"{JAR_SHA}.jar")
            (directory / "horizon-mods.txt").write_text("mod.naturalist:true\n")

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
            owner = data("@s", "UUID")
            def round_trip(species, item, variant, full, tame=False):
                cmd("clear @s")
                tag = f"capture_{species}"
                mob = f"@e[tag={tag},limit=1]"
                new_mob = f"@e[type=naturalist:{species},x=7.5,y=100,z=8.5,distance=..1,limit=1]"
                cmd(f'summon naturalist:{species} 7.5 100 8.5 {{Tags:[{tag}],NoAI:1b,Health:1.5f,'
                    f'HillCooldown:777,InLove:300,CustomName:"Beacon",Variant:"naturalist:{variant}"' + (f',Owner:{owner}' if tame else '') + '}')
                before_uuid = data(mob, "UUID")
                assert before_uuid, species
                cmd("item replace entity @s weapon.mainhand with naturalist:capture_net")
                ctl("wait", 4)
                target_id = ctl("entities", 12, f"naturalist:{species}")["entities"][-1]["id"]
                ctl("look-at", "entity", target_id)
                ctl("mouse", "right", "click")
                ctl("wait", 4)
                saved = data("@s", "Inventory") or ""
                assert f"naturalist:{item}" in saved and '"minecraft:custom_data"' in saved, saved
                assert '"minecraft:damage": 1' in saved and data(mob, "UUID") is None, saved
                saved_path = f'Inventory[{{id:"naturalist:{item}"}}].components."minecraft:custom_data"'
                result = {
                    f"{species}_saves_id": data("@s", saved_path + ".id") is not None,
                }
                if species == "ant":
                    assert data("@s", saved_path + ".UUID") is None
                    assert data("@s", saved_path + ".Age") is None
                    assert data("@s", saved_path + ".HillCooldown") is None
                    assert data("@s", saved_path + ".InLove") is None
                    assert data("@s", saved_path + ".Tame") == ("1b" if tame else None)
                    assert data("@s", saved_path + ".Owner") == (owner if tame else None)
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
                if species == "ant":
                    assert data(new_mob, "Owner") == (owner if tame else None)
                    assert data(new_mob, "Sitting") in ("0b", "false")
                cmd(f"kill @e[type=naturalist:{species}]")
                ctl("wait", 22)
                return result

            for tame in (False, True):
                result = round_trip("ant", "ant", "ant", False, tame)
                print(json.dumps({"tame":tame,"capture_release":result}), flush=True)
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--jar", type=Path, help="original jar required by the current external-jar package")
    args = parser.parse_args()
    capture(args.version, args.jar)
