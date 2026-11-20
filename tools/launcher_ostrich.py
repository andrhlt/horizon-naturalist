"""Replay Ostrich feeding, hand taming, saddle riding, and eggs through headless Horizon."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-ostrich-", ignore_cleanup_errors=True) as temp:
        root = Path(temp)
        socket = root / "ostrich.sock"

        def ctl(*args):
            response = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                      capture_output=True, text=True, timeout=75)
            assert response.returncode == 0, (args, response.stdout, response.stderr)
            return json.loads(response.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def data(entity, path):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {entity} {path}") if "following entity data: " in row), None)

        def summon(pos, nbt):
            lines = cmd(f"summon naturalist:ostrich {pos} {nbt}")
            assert any("Summoned" in row for row in lines), lines

        def held(item):
            cmd("item replace entity @s weapon.mainhand with " + item)
            ctl("wait", 4)

        def click():
            targets = ctl("entities", 4, "naturalist:ostrich")["entities"]
            assert targets, targets
            ctl("look-at-entity", targets[0]["id"])
            ctl("mouse", "right", "click")
            ctl("wait", 5)

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        def exists(x, y, z, block):
            return any("Test passed" in row for row in cmd(f"execute if block {x} {y} {z} {block}"))

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ostrich", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 8.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 108 15 minecraft:air")
            cmd("tp @s 8.5 100 6.5 0 0")
            cmd("gamemode survival @s")
            actor = data("@s", "UUID")
            pet = "@e[type=naturalist:ostrich,tag=pet,limit=1]"

            summon("8.5 100 8.5", "{Tags:[pet],NoAI:1b}")
            stats = []
            for attr in ("max_health", "movement_speed", "attack_damage", "jump_strength", "step_height", "follow_range"):
                rows = cmd(f"attribute {pet} minecraft:{attr} base get")
                stats.append(next(float(row.rsplit(" is ", 1)[1].split()[0]) for row in rows if " is " in row))
            output = {"attributes": stats}
            held("minecraft:wheat_seeds 10")
            click()
            output["adult_seed_outcome"] = [bool(data(pet, "Owner")), int(data(pet, "InLove") or 0) > 0,
                                           int(data("@s", "SelectedItem.count") or 0)]
            cmd(f"data merge entity {pet} {{Age:-24000,InLove:0}}")
            click()
            output["juvenile_seed_outcome"] = [data(pet, "Owner") == actor, data(pet, "Sitting") == "1b",
                                               int(data("@s", "SelectedItem.count") or 0)]
            cmd("tick freeze")
            before = int(data(pet, "Age"))
            click()
            acceleration = int(data(pet, "Age")) - before
            print(json.dumps({"baby_age_before": before, "baby_age_advance": acceleration}), flush=True)
            expected_advance = int((-before // 20) * 0.1) * 20
            result_age_up = acceleration == expected_advance
            cmd("tick unfreeze")
            cmd(f"data merge entity {pet} {{Age:0,Sitting:0b}}")
            ctl("wait", 6)
            cmd(f"data merge entity {pet} {{Health:20f}}")
            print(json.dumps({"adult_health_before_feed": data(pet, "Health")}), flush=True)
            click()
            output["owner_heals"] = [float(data(pet, "Health").rstrip("f")), int(data("@s", "SelectedItem.count") or 0)]
            held("minecraft:saddle")
            click()
            output["saddles_and_consumes"] = data(pet, "Saddled") == "1b" and data("@s", "SelectedItem.id") is None
            click()
            riding = ctl("status")["riding"]
            output["owner_rides"] = riding is not None
            if riding is not None:
                ctl("key", "shift", "down")
                ctl("wait", 4)
                ctl("key", "shift", "up")
            cmd("tp @s 8.5 100 6.5 0 0")
            held("minecraft:shears")
            click()
            output["shears_remove_saddle"] = data(pet, "Saddled") == "0b" and data("@s", 'SelectedItem.components."minecraft:damage"') == "1"
            held("minecraft:purple_dye")
            click()
            output["owner_dyes"] = [int(data(pet, "DyeColor") or -1), int(data("@s", "SelectedItem.count") or 0)]
            held("minecraft:shears")
            click()
            output["owner_clears_dye"] = data(pet, "DyeColor") == "-1" and data(
                "@s", 'SelectedItem.components."minecraft:damage"') == "1"
            cmd(f"data merge entity {pet} {{Sitting:1b}}")
            held("naturalist:whistle")
            click()
            output["whistle_wander"] = [data(pet, "FollowingOwner") == "1b", data(pet, "Sitting") == "1b",
                                        int(data("@s", "SelectedItem.count") or 0)]
            print(json.dumps({"pet": output}), flush=True)

            cmd(f"kill {pet}")
            ctl("wait", 25)
            cmd("gamemode creative @s")
            cmd("fill 2 98 2 14 98 14 minecraft:stone")
            cmd("fill 2 99 2 14 99 14 minecraft:sand")
            cmd("tick freeze")
            summon("8.5 100 8.5", "{Tags:[brood],InLove:600}")
            summon("10.5 100 8.5", "{Tags:[mate],InLove:600}")
            brood = "@e[tag=brood,limit=1]"
            mate = "@e[tag=mate,limit=1]"
            step(65)
            output["breed_sets_egg"] = any(data(parent, "HasEgg") == "1b" for parent in (brood, mate))
            owner_candidates = lambda: [(parent, data(parent, "OwnedEggs") or "") for parent in (brood, mate)]
            for _ in range(10):
                owners = owner_candidates()
                details = [{key: data(parent, key) for key in ("HasEgg", "Pos", "Sitting", "InLove")}
                           for parent in (brood, mate)]
                print(json.dumps({"breeding": details, "owners": owners}), flush=True)
                if any(re.search(r"-?\d+L", eggs) for _, eggs in owners):
                    break
                step(30)
            egg_position = None
            for parent, eggs in owner_candidates():
                if match := re.search(r"(-?\d+)L", eggs):
                    packed = int(match[1])
                    def signed(value, bits):
                        value &= (1 << bits) - 1
                        return value if value < 1 << (bits - 1) else value - (1 << bits)
                    egg_position = [signed(packed >> 38, 26), signed(packed, 12), signed(packed >> 12, 26)]
                    break
            output["owns_laid_egg"] = egg_position is not None
            ex, ey, ez = egg_position or [8, 100, 8]
            output["laid_egg_fresh"] = exists(ex, ey, ez, "naturalist:ostrich_egg[hatch=0]")
            cmd(f"tp {brood} {ex+4.5} {ey} {ez+1.5}")
            cmd(f"tp {mate} {ex+4.5} {ey} {ez+2.5}")
            cmd("tick unfreeze")
            cmd(f"tp @s {ex+0.5} {ey} {ez-1.5}")
            cmd("gamemode survival @s")
            held("minecraft:diamond_pickaxe")
            ctl("look-at", ex+0.5, ey+0.48, ez+0.5)
            print(json.dumps({"aim_at_owned_egg": ctl("looking-at")}), flush=True)
            ctl("mouse", "left", "down")
            ctl("wait", 20)
            ctl("mouse", "left", "up")
            mined = output["laid_egg_fresh"] and exists(ex, ey, ez, "minecraft:air")
            anger = [data(parent, "angry_at") for parent in (brood, mate)]
            output["owned_egg_break_targets_player"] = mined and actor in anger
            print(json.dumps({"mined": mined, "angry_at": anger, "actor": actor}), flush=True)
            cmd("gamemode creative @s")
            cmd("tick freeze")
            cmd("setblock 13 99 11 minecraft:stone")
            cmd("setblock 13 100 11 naturalist:ostrich_egg[hatch=2]")
            cmd("gamerule minecraft:random_tick_speed 4096")
            for _ in range(16):
                step(2)
                if exists(13, 100, 11, "minecraft:air"):
                    break
            young = "@e[type=naturalist:ostrich,tag=!brood,tag=!mate,tag=!pet,limit=1]"
            age = int(data(young, "Age") or 0)
            output["hatched_untamed_on_stone"] = age < -23800 and data(young, "Owner") is None
            print(json.dumps({"egg": {key: value for key, value in output.items() if key in
                ("breed_sets_egg", "laid_egg_fresh", "owns_laid_egg", "owned_egg_break_targets_player", "hatched_untamed_on_stone")},
                "hatchling_age": age}), flush=True)
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tick unfreeze")
            cmd("tp @s 5.5 100 4.5")
            cmd("gamemode survival @s")
            cmd("setblock 5 100 6 naturalist:ostrich_egg")
            held("naturalist:ostrich_egg 2")
            ctl("look-at", 5.5, 100.48, 6.5)
            print(json.dumps({"aim_at_placement_egg": ctl("looking-at")}), flush=True)
            ctl("mouse", "right", "click")
            ctl("wait", 5)
            placed = exists(5, 100, 6, "naturalist:ostrich_egg[eggs=1]") and exists(
                5, 101, 6, "naturalist:ostrich_egg[eggs=1]")
            assert placed and data("@s", "SelectedItem.count") == "1", cmd("data get entity @s SelectedItem")
            print(json.dumps({"physical_adjacent_egg_placement": placed}), flush=True)
            assert result_age_up, (before, acceleration, expected_advance)
            return output
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("version", type=Path)
    cli.add_argument("java", type=Path)
    options = cli.parse_args()
    expected = json.loads(options.java.read_text())
    expected["hatched_untamed_on_stone"] = expected["hatchling_data"][0] == -24000 and not expected["hatchling_data"][1]
    actual = capture(options.version)
    print(json.dumps(actual, indent=2), flush=True)
    assert all(value == expected[key] for key, value in actual.items()), (actual, expected)
