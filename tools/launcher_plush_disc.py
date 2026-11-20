"""Verify live Plush Bear facing, Fox disc drops and jukebox use in headless Horizon."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, reference, seeded, record_only):
    with tempfile.TemporaryDirectory(prefix="nat-plush-disc-", ignore_cleanup_errors=True) as folder:
        home = Path(folder)
        socket = home / "control.sock"

        def ctl(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=75)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def aim_and_click(x, y, z):
            ctl("look-at", x, y, z)
            target = ctl("looking-at")
            assert target["kind"] == "block", target
            ctl("mouse", "right", "click")
            ctl("wait", 3)

        def entity_data(target, field):
            marker = " following entity data: "
            return next((line.split(marker, 1)[1] for line in command(f"data get entity {target} {field}")
                         if marker in line), None)

        disc = "naturalist:music_disc_wild_ones"
        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist Plush and Record", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:mob_drops false")
            command("tp @s 8.5 102 6.5")
            ctl("wait", 25)
            command("fill 2 101 2 15 101 15 minecraft:stone")
            command("fill 2 102 2 15 108 15 minecraft:air")
            for yaw, x, z in ((0,8.5,6.5), (90,10.5,8.5), (180,8.5,10.5), (270,6.5,8.5)):
                command("setblock 8 102 8 minecraft:air")
                command(f"tp @s {x} 102 {z}")
                command("item replace entity @s weapon.mainhand with naturalist:plush_bear")
                aim_and_click(8.5, 101.99, 8.5)
                facing, outline, collision = reference["plush"]["placed"][str(yaw)]
                reply = command(f"execute if block 8 102 8 naturalist:plush_bear[facing={facing}] run say plush-faced-correctly")
                assert any("plush-faced-correctly" in line for line in reply), (yaw, facing, reply, ctl("block", 8, 102, 8))
                ctl("look-at", 8.5, 102.55, 8.5)
                hit = ctl("looking-at")["block"]
                axis, edge = {0: (2, 2), 90: (0, 3), 180: (2, 5), 270: (0, 0)}[yaw]
                assert hit["block"] == "naturalist:plush_bear" and hit["face"] == facing, hit
                assert abs(hit["point"][axis] - (8 + outline[edge] / 16)) < 1e-5, hit
                command("gamemode survival @s")
                command("tp @s 8.5 104 8.5")
                ctl("wait", 25)
                feet = ctl("state")["player"]["position"][1]
                assert abs(feet - (102 + collision[4] / 16)) < 1e-5, feet
                command("gamemode creative @s")
                print(f"Plush Bear yaw {yaw} faced {facing}, exact outline and collision", flush=True)

            command("setblock 8 102 8 minecraft:jukebox")
            command("tp @s 8.5 102 6.5")
            command(f"item replace entity @s weapon.mainhand with {disc}")
            aim_and_click(8.5, 102.5, 8.5)
            actual = command("data get block 8 102 8 RecordItem.id")
            assert any(f'"{disc}"' in line for line in actual), actual
            clock = command("data get block 8 102 8 ticks_since_song_started")
            assert any(int(line.rsplit(" ", 1)[-1].rstrip("Ll")) > 0
                       for line in clock if "following block data" in line), clock
            print("Jukebox accepted Wild Ones and advanced its playback counter", flush=True)
            command("setblock 8 102 7 minecraft:comparator[facing=south,mode=compare,powered=false]")
            command("setblock 8 102 6 minecraft:redstone_wire")
            ctl("wait", 8)
            powered = command("execute if block 8 102 6 minecraft:redstone_wire[power=15] run say record-strength")
            assert any("record-strength" in row for row in powered), (powered, ctl("block", 8, 102, 7), ctl("block", 8, 102, 6))
            print("Jukebox delivered the original comparator strength 15", flush=True)
            command("setblock 8 102 7 minecraft:air")
            command("setblock 8 102 6 minecraft:air")
            if record_only:
                return
            command("setblock 8 102 8 minecraft:air")

            command("summon minecraft:fox 13.5 102 8.5 {NoAI:1b,Tags:[command_fox_fixture]}")
            controlled = "@e[type=minecraft:fox,tag=command_fox_fixture,limit=1]"
            assert entity_data(controlled, "equipment.mainhand.id") != f'"{disc}"'
            if seeded:
                marker = "Wild Ones fixture "
                replies = command("naturalist_fixture_wild_ones")
                result = next(json.loads(line.split(marker, 1)[1]) for line in replies if marker in line)
                assert result == reference["disc"]["SPAWN_ITEM_USE"], (result, reference["disc"]["SPAWN_ITEM_USE"])
                print(f"Exact Java Fox chance matched all 1000 seeds, {len(result[0])} adult wins", flush=True)
            command(f"kill {controlled}")
            command("setblock 8 102 8 minecraft:air")
            command("tp @s 8.5 102 6.5")
            command('item replace entity @s weapon.mainhand with minecraft:fox_spawn_egg'
                    '[minecraft:entity_data={id:"minecraft:fox",NoAI:1b,Silent:1b,Tags:[wild_ones_egg_fixture]}]')
            fox = "@e[type=minecraft:fox,tag=wild_ones_egg_fixture,x=8.5,y=102,z=8.5,distance=..1,limit=1]"
            for trial in range(1,301):
                aim_and_click(8.5, 101.99, 8.5)
                if entity_data(fox, "equipment.mainhand.id") == f'"{disc}"':
                    break
                assert entity_data(fox, "UUID"), (trial, command("data get entity @s Inventory"))
                command(f"kill {fox}")
                ctl("wait", 22)
            else:
                raise AssertionError("No Wild Ones disc appeared in 300 real Fox spawn eggs")
            chance = entity_data(fox, "drop_chances.mainhand")
            assert chance is not None and float(chance.rstrip("f")) == 2.0, chance
            assert entity_data(fox, "CanPickUpLoot") in (None, "0b")
            command("gamerule minecraft:mob_drops true")
            command(f"kill {fox}")
            ctl("wait", 3)
            dropped = command(f'execute if entity @e[type=minecraft:item,nbt={{Item:{{id:"{disc}",count:1}}}}] run say disc-dropped')
            assert any("disc-dropped" in line for line in dropped), dropped
            print(f"Fox egg {trial} carried and reliably dropped Wild Ones", flush=True)

        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--seeded", action="store_true")
    parser.add_argument("--record-only", action="store_true")
    args = parser.parse_args()
    capture(args.version, json.loads(args.reference.read_text()), args.seeded, args.record_only)
