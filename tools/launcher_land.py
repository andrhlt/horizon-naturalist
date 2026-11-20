"""Check boar and turkey behavior in a launcher-built Horizon client."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


ATTRIBUTES = ("max_health", "movement_speed", "attack_damage", "attack_knockback",
              "knockback_resistance", "follow_range", "step_height")


def capture(version, reference, capybara_reference=None):
    with tempfile.TemporaryDirectory(prefix="nat-land-", ignore_cleanup_errors=True) as path:
        home = Path(path)
        socket = home / "land.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            if result.returncode:
                raise RuntimeError((args, result.stdout, result.stderr))
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            rows = ctl("command", line)
            if not rows["ok"]:
                raise RuntimeError((line, rows))
            return rows["chat"]

        def read(entity, key):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {entity} {key}")
                         if "following entity data: " in row), None)

        def summon(species, x, y, z, nbt):
            rows = cmd(f"summon {species} {x} {y} {z} {nbt}")
            assert any("Summoned" in row for row in rows), rows

        def tag(name):
            return f"@e[tag={name},limit=1]"

        def interact(x, eye, z, item, count=2):
            cmd(f"item replace entity @s weapon.mainhand with {item} {count}")
            ctl("look-at", x, eye, z)
            ctl("mouse", "right", "click")
            ctl("wait", 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Land Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("forceload add 0 0")
            cmd("tp @s 8.5 101 5.5")
            ctl("wait", 20)
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 122 15 minecraft:air")
            cmd("tp @s 8.5 101 5.5")
            ctl("wait", 4)
            cmd("difficulty normal")
            cmd("gamemode survival @s")
            cmd("effect give @s minecraft:resistance infinite 4 true")
            output = {}
            for species, x, eye in (("boar", 5.5, 101.75), ("turkey", 10.5, 101.67)):
                identifier = f"naturalist:{species}"
                adult, mate = tag(species), tag(f"{species}_mate")
                summon(identifier, x, 101, 7.5, f'{{NoAI:1b,Tags:[{species}]}}')
                summon(identifier, x + 1, 101, 7.5, f'{{Age:-24000,NoAI:1b,Tags:[{species}_baby]}}')
                attrs = {}
                for attribute in ATTRIBUTES:
                    rows = cmd(f"attribute {adult} minecraft:{attribute} base get")
                    attrs[attribute] = next((float(match[1]) for row in rows
                                             if (match := re.search(r" is ([0-9.eE+-]+)", row))), None)
                result = {
                    "attributes": attrs,
                    "variant": json.loads(read(adult, "Variant") or "null"),
                    "juvenile_age": int(read(tag(f"{species}_baby"), "Age") or 0),
                }
                cmd(f"kill {tag(f'{species}_baby')}")
                cmd(f"tp @s {x} 101 5.5")
                accepted = []
                food_candidates = set(reference[species]["food"]) | {
                    "wheat", "carrot", "potato", "beetroot", "apple", "wheat_seeds",
                }
                for item in sorted(food_candidates):
                    cmd(f"data merge entity {adult} {{InLove:0,Age:0}}")
                    interact(x, eye, 7.5, f"minecraft:{item}")
                    if int(read(adult, "InLove") or 0) > 0:
                        accepted.append(item)
                result["food"] = accepted
                result["fed_adult"] = bool(accepted)
                cmd(f"data merge entity {adult} {{InLove:0,Age:0}}")
                interact(x, eye, 7.5, "minecraft:" + reference[species]["food"][0])
                raw = read("@s", "SelectedItem") or ""
                result["remaining_feed"] = int(match[1]) if (match := re.search(r"count:\s*(\d+)", raw)) else None
                summon(identifier, x + 1, 101, 7.5, f'{{InLove:600,NoAI:1b,Tags:[{species}_mate]}}')
                cmd(f"data merge entity {adult} {{InLove:600,NoAI:0b}}")
                cmd(f"data merge entity {mate} {{NoAI:0b}}")
                ctl("wait", 115)
                nearby = ctl("entities", 15, identifier)["entities"]
                result["same_species_mate"] = any(row["baby"] and row["health"] > 0 for row in nearby)
                cmd(f"kill @e[type={identifier}]")
                summon(identifier, x, 111, 10.5, f'{{Tags:[{species}_fall]}}')
                ctl("wait", 105)
                fall_health = read(tag(f"{species}_fall"), "Health")
                result["fall_damage"] = fall_health is not None and float(fall_health.rstrip("fd")) < attrs["max_health"]
                cmd(f"kill @e[type={identifier}]")
                output[species] = result
            cmd("gamerule minecraft:fire_spread_radius_around_player 0")
            summon("naturalist:boar", 6.5, 101, 11.5,
                   '{Age:-24000,NoAI:1b,CustomName:"Fixture Boar",CustomNameVisible:1b,Tags:[lightning_boar]}')
            rows = cmd("summon minecraft:lightning_bolt 6.5 101 11.5")
            assert any("Summoned" in row for row in rows), rows
            ctl("wait", 15)
            zoglin = '@e[type=minecraft:zoglin,name="Fixture Boar",limit=1]'
            name = read(zoglin, "CustomName")
            output["boar_behavior"] = {
                "lightning_removed": read(tag("lightning_boar"), "Health") is None,
                "lightning_baby": any(mob["baby"] for mob in ctl("entities", 20, "minecraft:zoglin")["entities"]),
                "lightning_no_ai": read(zoglin, "NoAI") in ("1b", "1", "true"),
                "lightning_name_visible": read(zoglin, "CustomNameVisible") in ("1b", "1", "true"),
                "lightning_name": "Fixture Boar" if name and "Fixture Boar" in name else name,
            }
            if capybara_reference is not None:
                cmd("kill @e[type=minecraft:zoglin]")
                output["capybara"] = replay_capybara(ctl, cmd, read, summon, tag, interact)
            return output
        finally:
            if socket.exists():
                ctl("quit")


def replay_capybara(ctl, cmd, read, summon, tag, interact):
    species = "naturalist:capybara"
    animal = tag("capybara_adult")
    mate = tag("capybara_mate")
    food = "minecraft:melon_slice"
    cmd("clear @s")
    cmd("tp @s 7.5 101 7.5")
    ctl("wait", 4)
    summon(species, 7.5, 101, 9.5, "{NoAI:1b,Tags:[capybara_adult]}")
    summon(species, 8.5, 101, 9.5, "{NoAI:1b,Tags:[capybara_mate]}")

    result = {"variant": json.loads(read(animal, "Variant") or "null")}
    for attribute in ("max_health", "movement_speed"):
        rows = cmd(f"attribute {animal} minecraft:{attribute} base get")
        result[attribute] = next((float(match[1]) for row in rows
                                 if (match := re.search(r" is ([0-9.eE+-]+)", row))), None)
    for who, x in ((animal, 7.5), (mate, 8.5)):
        for _ in range(36):
            interact(x, 101.75, 9.5, food)
            owner = read(who, "Owner")
            if owner:
                break
        else:
            raise AssertionError(f"Capybara {who} did not tame")
    result["tame_succeeded"] = read(animal, "Owner") is not None
    result["default_dye"] = "red" if read(animal, "DyeColor") == "14" else read(animal, "DyeColor")

    cmd("clear @s")
    ctl("look-at", 7.5, 101.75, 9.5)
    ctl("mouse", "right", "click")
    ctl("wait", 4)
    result["ordered_sit"] = read(animal, "Sitting") in ("1", "1b", "true")
    for _ in range(2):
        interact(7.5, 101.75, 9.5, "naturalist:whistle", 1)
        following = read(animal, "FollowingOwner")
        if following in ("0b", "0", "false"):
            break
        ctl("wait", 20)
    result["whistle_flipped_following"] = following in ("0b", "0", "false")
    result["whistle_stood_pet_up"] = read(animal, "Sitting") in ("0b", "0", "false")

    cmd(f"data merge entity {animal} {{Health:4f}}")
    interact(7.5, 101.75, 9.5, food)
    result["healed_health"] = float((read(animal, "Health") or "0").rstrip("fd"))
    result["heal_consumed"] = bool(re.search(r"count:\s*1\b", read("@s", "SelectedItem") or ""))

    for who in (animal, mate):
        cmd(f"data merge entity {who} {{NoAI:0b,InLove:600}}")
    ctl("wait", 110)
    young = "@e[type=naturalist:capybara,tag=!capybara_adult,tag=!capybara_mate,limit=1]"
    result["tamed_can_mate"] = any(mob["baby"] for mob in ctl("entities", 20, species)["entities"])
    result["offspring_tamed"] = read(young, "Owner") is not None
    result["offspring_owner"] = result["offspring_tamed"] and read(young, "Owner") == read(animal, "Owner")
    result["offspring_dye"] = "red" if read(young, "DyeColor") == "14" else read(young, "DyeColor")
    cmd(f"kill @e[type={species}]")
    summon(species, 7.5, 111, 9.5, "{Tags:[capybara_fall]}")
    ctl("wait", 60)
    result["fall_damage"] = float((read(tag("capybara_fall"), "Health") or "0").rstrip("fd")) < 14.0
    return result


def compare(output, reference, capybara_reference=None):
    for species in ("boar", "turkey"):
        result = output[species]
        for name, actual in result["attributes"].items():
            assert actual is not None and abs(reference[species][name] - actual) < 1e-6, (species, name, actual)
        for key in ("variant", "food", "fed_adult", "remaining_feed", "same_species_mate", "fall_damage"):
            assert result[key] == reference[species][key], (species, key, result[key], reference[species][key])
        assert -24000 <= result["juvenile_age"] < 0, (species, result["juvenile_age"])
    for key, actual in output["boar_behavior"].items():
        assert actual == reference["boar_behavior"][key], (key, actual, reference["boar_behavior"][key])
    if capybara_reference is not None:
        for key, actual in output["capybara"].items():
            expected = capybara_reference[key]
            if type(expected) is float:
                assert actual is not None and abs(actual - expected) < 1e-6, (key, actual, expected)
            else:
                assert actual == expected, (key, actual, expected)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--capybara-fixture", type=Path)
    options = parser.parse_args()
    reference = json.loads(options.fixture.read_text())
    capybara_reference = json.loads(options.capybara_fixture.read_text()) if options.capybara_fixture else None
    result = capture(options.version, reference, capybara_reference)
    print(json.dumps(result, indent=2), flush=True)
    compare(result, reference, capybara_reference)
