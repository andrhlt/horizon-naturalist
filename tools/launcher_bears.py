"""Compare bear taming, feeding, breeding and shearing in headless Horizon."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, reference, mode):
    with tempfile.TemporaryDirectory(prefix="nat-bears-", ignore_cleanup_errors=True) as path:
        home = Path(path)
        socket = home / "bears.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     capture_output=True, text=True, timeout=75)
            if process.returncode:
                raise RuntimeError((args, process.stdout, process.stderr))
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        def read(mob, key):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {mob} {key}")
                         if "following entity data: " in row), None)

        def attr(mob, name):
            rows = cmd(f"attribute {mob} minecraft:{name} base get")
            return next((float(match[1]) for row in rows if (match := re.search(r" is ([0-9.eE+-]+)", row))), None)

        def summon(species, tag, nbt="", x=8.5, z=8.5):
            result = cmd(f"summon naturalist:{species} {x} 101 {z} {{Tags:[{tag}]{nbt}}}")
            assert any("Summoned" in row for row in result), result
            return f"@e[tag={tag},limit=1]"

        def feed(item, eye):
            cmd(f"item replace entity @s weapon.mainhand with {item} 2")
            ctl("look-at", 8.5, eye, 8.5)
            ctl("mouse", "right", "click")
            ctl("wait", 3)
            return bool(re.search(r"count:\s*1\b", read("@s", "SelectedItem") or ""))

        def shear(eye):
            cmd("item replace entity @s weapon.mainhand with minecraft:shears")
            ctl("look-at", 8.5, eye, 8.5)
            ctl("mouse", "right", "click")
            ctl("wait", 3)

        def truth(mob, key):
            return read(mob, key) in ("1b", "1", "true")

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Bears Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 105 15 minecraft:air")
            cmd("tp @s 8.5 101 5.75")
            ctl("wait", 10)
            cmd("gamemode survival @s")
            cmd("effect give @s minecraft:resistance infinite 4 true")
            if mode == "audio":
                return {species: replay_audio(ctl, cmd, read, summon, species) for species in reference}
            if mode == "foraging":
                cmd("gamemode spectator @s")
                return {species: replay_harvest(ctl, cmd, read, summon, species) for species in reference}
            outputs = {}
            for species, expected in reference.items():
                eye = 101 + expected["adult"][2] * 0.7
                cub_eye = 101 + expected["juvenile"][2] * 0.7
                adult = summon(species, "adult_fixture", ",NoAI:1b")
                output = {name: attr(adult, name) for name in (
                    "max_health", "movement_speed", "attack_damage", "knockback_resistance",
                    "follow_range", "step_height",
                )}
                output["variant"] = json.loads(read(adult, "Variant") or "null")
                accepted = []
                for food in expected["food"]:
                    cmd(f"data merge entity {adult} {{InLove:0,Age:0}}")
                    item = f"{'naturalist' if food == 'venison' else 'minecraft'}:{food}"
                    if feed(item, eye) and int(read(adult, "InLove") or 0) > 0:
                        accepted.append(food)
                output["food"] = accepted
                output["adult_wont_tame"] = read(adult, "Owner") is None
                shear(eye)
                output["adult_shearable"] = output["sheared"] = truth(adult, "Sheared")
                fur = read("@e[type=minecraft:item,sort=nearest,limit=1]", "Item.id")
                output["drops_fur"] = fur == '"naturalist:fur"'
                cmd(f"tp {adult} 12.5 101 9.5")
                cub = summon(species, "cub_fixture", ",NoAI:1b,Age:-24000")
                shear(cub_eye)
                output["cub_shearable"] = truth(cub, "Sheared")
                consumed = []
                for _ in range(35):
                    consumed.append(feed("minecraft:honeycomb", cub_eye))
                    if read(cub, "Owner"):
                        break
                output["cub_tamed"] = read(cub, "Owner") is not None
                output["tame_consumed"] = all(consumed)
                cmd(f"data merge entity {cub} {{Health:4f}}")
                output["heal_consumed"] = feed("minecraft:honeycomb", cub_eye)
                output["healed_health"] = float((read(cub, "Health") or "0").rstrip("fd"))
                cmd("kill @e[tag=cub_fixture]")
                cmd("kill @e[type=minecraft:item]")
                cmd(f"kill {adult}")
                adult = summon(species, "breed_fixture", ",InLove:600,PersistenceRequired:1b", x=9.5, z=8.5)
                other = "black_bear" if species == "bear" else "bear"
                opposite = summon(other, "cross_fixture", ",InLove:600,PersistenceRequired:1b", x=11.5, z=8.5)
                cmd(f"data merge entity {adult} {{NoAI:0b,InLove:600,Age:0,PersistenceRequired:1b}}")
                cmd("time set 0")
                cmd("gamemode creative @s")
                ctl("wait", 125)
                output["cross_species_mate"] = int(read(adult, "Age") or 0) > 0
                cmd(f"kill {opposite}")
                mate = summon(species, "mate_fixture", ",InLove:600,PersistenceRequired:1b", x=11.5, z=8.5)
                cmd(f"data merge entity {adult} {{InLove:600,Age:0}}")
                ctl("wait", 150)
                output["same_species_mate"] = int(read(mate, "Age") or 0) > 0
                cmd("kill @e[tag=adult_fixture]")
                cmd("kill @e[tag=mate_fixture]")
                cmd("kill @e[type=naturalist:bear]")
                cmd("kill @e[type=naturalist:black_bear]")
                cmd("kill @e[type=minecraft:item]")
                outputs[species] = output
                cmd("gamemode survival @s")
            return outputs
        finally:
            if socket.exists():
                ctl("quit")


def replay_harvest(ctl, cmd, read, summon, species):
    cmd("item replace entity @s weapon.mainhand with minecraft:air")
    cmd("gamerule minecraft:random_tick_speed 0")
    cmd("gamerule minecraft:mob_griefing true")
    cmd("gamerule minecraft:advance_time false")
    cmd("time set 0")
    cmd("setblock 8 100 8 minecraft:dirt")
    for axis in (
        "6 101 6 11 103 6", "6 101 11 11 103 11", "6 101 7 6 103 10", "11 101 7 11 103 10",
    ):
        cmd(f"fill {axis} minecraft:glass")

    def check(block):
        rows = cmd(f"execute if block 8 101 8 {block} run say harvest-matched")
        return any("harvest-matched" in row for row in rows)

    def raid(changed):
        bear = summon(species, "harvest_fixture", ",PersistenceRequired:1b", x=8.5, z=10.25)
        for _ in range(80):
            ctl("wait", 5)
            if changed():
                break
        return bear

    def drop_or_held(item, bear):
        matching_item = f'@e[type=minecraft:item,nbt={{Item:{{id:"{item}"}}}},limit=1]'
        if read(matching_item, "Item.id") == f'"{item}"':
            return True
        hand = read(bear, "equipment.mainhand.id")
        return hand == f'"{item}"'

    cmd("setblock 8 101 8 minecraft:sweet_berry_bush[age=3]")
    bear = raid(lambda: check("minecraft:sweet_berry_bush[age=1]"))
    output = {
        "harvest_berry_age": 1 if check("minecraft:sweet_berry_bush[age=1]") else None,
        "harvest_berries_dropped": drop_or_held("minecraft:sweet_berries", bear),
    }
    cmd(f"kill {bear}")
    ctl("wait", 25)
    cmd("kill @e[type=minecraft:item]")
    cmd("setblock 8 101 8 minecraft:beehive[honey_level=5]")
    bear = raid(lambda: check("minecraft:beehive[honey_level=0]"))
    output.update(
        harvest_honey_level=0 if check("minecraft:beehive[honey_level=0]") else None,
        harvest_honeycomb_dropped=drop_or_held("minecraft:honeycomb", bear),
    )
    cmd(f"kill {bear}")
    ctl("wait", 25)
    cmd("kill @e[type=minecraft:item]")
    cmd("setblock 8 101 8 minecraft:campfire[lit=false]")
    cmd('data merge block 8 101 8 {Items:[{Slot:0b,id:"minecraft:salmon",count:1},'
        '{Slot:1b,id:"minecraft:beef",count:1}],CookingTimes:[I;0,0,0,0],'
        'CookingTotalTimes:[I;600,600,0,0]}')
    read_block = lambda key: cmd(f"data get block 8 101 8 {key}")
    before = str(read_block("Items"))
    assert "minecraft:salmon" in before and "minecraft:beef" in before, before
    bear = raid(lambda: "minecraft:salmon" not in str(read_block("Items")))
    after = str(read_block("Items"))
    output.update(
        harvest_salmon_stolen="minecraft:salmon" not in after,
        harvest_beef_left="minecraft:beef" in after,
    )
    cmd(f"kill {bear}")
    cmd("setblock 8 101 8 minecraft:air")
    cmd("kill @e[type=minecraft:item]")
    for axis in (
        "6 101 6 11 103 6", "6 101 11 11 103 11", "6 101 7 6 103 10", "11 101 7 11 103 10",
    ):
        cmd(f"fill {axis} minecraft:air")
    return output


def replay_audio(ctl, cmd, read, summon, species):
    cmd("time set 2000")
    cmd("gamemode survival @s")
    cmd("effect clear @s")
    cmd("effect give @s minecraft:resistance infinite 3 true")
    cmd("tp @s 8.5 101 6.5")
    bear = summon(species, "audio_fixture", ",PersistenceRequired:1b", z=8.0)
    attack = "naturalist:entity.black_bear.attack"
    hurt = f"naturalist:entity.{species}.hurt"
    death = f"naturalist:entity.{species}.death"
    before = ctl("status")["heard"]
    health_before = float(read("@s", "Health").rstrip("Ff"))
    cmd(f"damage {bear} 1 minecraft:player_attack by @s")
    for _ in range(20):
        ctl("wait", 3)
        if float(read("@s", "Health").rstrip("Ff")) < health_before:
            break
    else:
        raise AssertionError(f"{species} did not land a melee attack")
    dying = summon(species, "death_audio_fixture", ",NoAI:1b", x=11.5, z=8.0)
    cmd(f"damage {dying} 100 minecraft:generic")
    ctl("wait", 2)
    after = ctl("status")["heard"]
    cmd(f"kill {bear}")
    return {name: sound if after.get(sound, 0) > before.get(sound, 0) else None
            for name, sound in (("swing_sound", attack), ("hurt_sound", hurt), ("death_sound", death))}


def compare(output, reference):
    failures = []
    for species, observations in output.items():
        for key, actual in observations.items():
            expected = reference[species][key]
            if type(expected) is float:
                equal = actual is not None and abs(actual - expected) < 1e-5
            else:
                equal = actual == expected
            if not equal:
                failures.append((species, key, actual, expected))
    assert not failures, failures


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    scenario = parser.add_mutually_exclusive_group()
    scenario.add_argument("--audio-only", action="store_true")
    scenario.add_argument("--foraging-only", action="store_true")
    options = parser.parse_args()
    reference = json.loads(options.fixture.read_text())
    if options.audio_only:
        output = capture(options.version, reference, "audio")
    elif options.foraging_only:
        output = capture(options.version, reference, "foraging")
    else:
        output = capture(options.version, reference, "behavior")
        for species, results in capture(options.version, reference, "foraging").items():
            output[species].update(results)
    print(json.dumps(output, indent=2), flush=True)
    compare(output, reference)
