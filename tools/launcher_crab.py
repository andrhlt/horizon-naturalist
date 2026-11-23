"""Compare Crab defense, equipment, pets, and Capture Net with pinned Java."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-crab-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def data(target, key):
        return next(row.split("following entity data: ", 1)[1]
                    for row in cmd(f"data get entity {target} {key}") if "following entity data: " in row)

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(__file__).with_name("play.log")
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Crab {home.name}", "--creative", "--arg=--control-socket",
                    f"--arg={socket}", "--arg=--headless", "--arg=--test-terrain", "--arg=--render-distance", "--arg=4"],
                    stdout=handle, stderr=subprocess.STDOUT)
            for _ in range(50):
                if child.poll() is not None:
                    raise RuntimeError(f"Horizon Play exited with code {child.returncode}")
                if socket.exists() and ctl("state")["screen"] == "playing":
                    break
                time.sleep(1)
            else:
                raise RuntimeError("Timed out waiting for Horizon Play to load the fixture world")
        else:
            manifest = json.loads((version / "native-mods.json").read_text())
            resources = Path.home() / "Library/Application Support/horizon-launcher/versions" / manifest["base"] / ".local/horizon-client/26.3"
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                resources, "--home", home, "--game-dir", home,
                "--world", "Crab", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:mob_griefing true")
        cmd("tp @s 8.5 92 7.5")
        ctl("wait", 18)
        cmd("tick freeze")
        cmd("fill 4 91 3 18 91 16 minecraft:stone")
        cmd("fill 4 92 3 18 98 16 minecraft:air")
        cmd("tp @s 8.5 92 7.5")
        ctl("wait", 5)
        cmd('summon naturalist:crab 8.5 92 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[adult]}')
        cmd('summon naturalist:crab 9.7 92 8.5 {Age:-24000,NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[baby]}')
        adult = "@e[type=naturalist:crab,tag=adult,limit=1]"
        output = {}

        def probe(query="adult"):
            marker = "Crab fixture "
            rows = cmd("naturalist_fixture_crab " + query)
            return next(json.loads(row.split(marker, 1)[1]) for row in rows if marker in row)

        def click():
            near = ctl("entities", 5, "naturalist:crab")["entities"]
            chosen = min(near, key=lambda entity: abs(entity["pos"][0] - 8.5))
            looked = ctl("look-at-entity", chosen["id"])
            assert looked["target"]["kind"] == "entity" and looked["target"]["entity"]["id"] == chosen["id"], looked
            ctl("mouse", "right", "click")
            ctl("wait", 3)

        def optional_data(target, key):
            try:
                return data(target, key)
            except StopIteration:
                return None

        for attribute, key in (("max_health", "health"), ("movement_speed", "speed"), ("attack_damage", "attack")):
            rows = cmd(f"attribute {adult} minecraft:{attribute} base get")
            value = next((m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
            assert value is not None, (attribute, rows)
            output[key] = float(value)
        output["variant"] = data(adult, "Variant").strip('"')
        initial = probe()
        for key in ("cod_food", "crab_food", "weapon_wanted", "stick_wanted", "portal", "cluster"):
            output[key] = initial[key]
        output["adult_dims"] = initial["dims"]
        output["creative_hides"] = initial["hides"]
        cmd("gamemode survival @s")
        ctl("wait", 4)
        step(2)
        status = probe()
        output["adult_hides"] = status["hides"]
        output["hidden_knockback"] = status["knock"]
        baby = probe("baby")
        output["baby_hides"] = baby["hides"]
        output["baby_dims"] = baby["dims"]

        cmd(f"damage {adult} 4 minecraft:generic")
        output["hidden_health"] = round(float(data(adult, "Health").rstrip("fF")), 2)
        cmd(f"item replace entity {adult} weapon.mainhand with minecraft:stone_sword")
        step(1)
        status = probe()
        output["armed_hides"] = status["hides"]
        output["armed_wants_more"] = status["weapon_wanted"]
        cmd(f"item replace entity {adult} weapon.mainhand with minecraft:air")
        ctl("key", "shift", "down")
        ctl("wait", 8)
        step(2)
        output["sneaker_hides"] = probe()["hides"]
        ctl("key", "shift", "up")
        cmd("item replace entity @s weapon.offhand with minecraft:tropical_fish")
        step(2)
        output["food_holder_hides"] = probe()["hides"]
        cmd("item replace entity @s weapon.offhand with minecraft:air")
        cmd("tp @s 8.5 92 6.0")
        ctl("wait", 4)

        cmd('summon minecraft:item 11.65 92 8.5 {PickupDelay:0s,Item:{id:"minecraft:stone_sword",count:1},Tags:[sword]}')
        cmd('summon minecraft:item 8.5 92 8.6 {PickupDelay:0s,Item:{id:"minecraft:stick",count:1},Tags:[stick]}')
        step(1)
        output["nearby_weapon_hides"] = probe()["hides"]
        cmd("tp @e[tag=sword,limit=1] 8.65 92 8.5")
        cmd(f"data merge entity {adult} {{NoAI:0b}}")
        step(2)
        output["weapon_picked"] = probe()["wearing"] == "minecraft:stone_sword"
        output["nonweapon_left"] = optional_data("@e[tag=stick,limit=1]", "Item") is not None
        cmd('summon minecraft:zombie 9.5 92 8.5 {NoAI:1b,NoGravity:1b,Tags:[dummy]}')
        output.update(probe("attack"))
        cmd("kill @e[tag=dummy]")
        cmd("kill @e[tag=baby]")
        cmd(f"data merge entity {adult} {{NoAI:1b}}")
        step(22)

        cmd("item replace entity @s weapon.mainhand with minecraft:tropical_fish 32")
        cmd("tp @s 8.5 92 7.5")
        for _ in range(32):
            click()
            if optional_data(adult, "Owner") is not None:
                break
        output["can_tame"] = optional_data(adult, "Owner") is not None and data(adult, "FollowingOwner") == "1b"
        output["tame_hides"] = probe()["hides"]
        cmd(f"data merge entity {adult} {{Health:4.0f}}")
        click()
        output["food_heals"] = float(data(adult, "Health").rstrip("fF")) == 6.0
        cmd("item replace entity @s weapon.mainhand with minecraft:air")
        click()
        output["empty_hand_sits"] = data(adult, "Sitting") == "1b"
        click()
        cmd("item replace entity @s weapon.mainhand with naturalist:whistle")
        click()
        output["whistle_stops_following"] = data(adult, "FollowingOwner") == "0b"
        cmd("setblock 9 92 9 minecraft:jukebox[has_record=true]")
        step(20)
        output["jukebox_dance"] = probe()["dances"]
        cmd("setblock 9 92 9 minecraft:air")
        step(20)
        output["no_record_stops_dance"] = not probe()["dances"]

        cmd(f'data merge entity {adult} {{Variant:"naturalist:yellow",Age:-900,Sitting:1b}}')
        cmd(f"item replace entity {adult} weapon.mainhand with minecraft:iron_axe[minecraft:damage=7]")
        cmd("item replace entity @s weapon.mainhand with naturalist:capture_net")
        click()
        raw = data("@s", "Inventory")
        output["capture_removes_crab"] = not ctl("entities", 5, "naturalist:crab")["entities"]
        output["capture_uses_one_durability"] = bool(re.search(r'"minecraft:damage": 1[,}]', raw))
        output["capture_gives_crab"] = 'id: "naturalist:crab"' in raw
        assert output["capture_gives_crab"], raw
        output["caught_item"] = "naturalist:crab"
        output["caught_variant"] = re.search(r'Variant: "([^"]+)"', raw)[1]
        output["caught_age"] = int(re.search(r'Age: (-?\d+)', raw)[1])
        output["caught_following"] = re.search(r'FollowingOwner: ([01])b', raw)[1] == "1"
        output["caught_sitting"] = re.search(r'Sitting: ([01])b', raw)[1] == "1"

        ctl("select", "naturalist:crab")
        ctl("look-at-block", 8, 91, 8)
        ctl("mouse", "right", "click")
        ctl("wait", 5)
        released = "@e[type=naturalist:crab,nbt={FromHand:1b},limit=1]"
        output["released_variant"] = data(released, "Variant").strip('"')
        output["released_age"] = int(data(released, "Age"))
        output["released_owner"] = optional_data(released, "Owner") is not None
        cmd(f"data merge entity {released} {{Health:7.0f}}")
        cmd("item replace entity @s weapon.mainhand with minecraft:tropical_fish")
        ctl("wait", 3)
        click()
        output["released_max_health"] = float(data(released, "Health").rstrip("fF"))
        weapon = data(released, "equipment.mainhand")
        output["released_weapon"] = 'id: "minecraft:iron_axe"' in weapon and '"minecraft:damage": 7' in weapon

        cmd("item replace entity @s weapon.mainhand with naturalist:crab[minecraft:custom_data={Variant:-1}]")
        ctl("look-at-block", 7, 91, 8)
        ctl("mouse", "right", "click")
        ctl("wait", 5)
        legacy = '@e[type=naturalist:crab,nbt={FromHand:1b,Age:0},limit=1]'
        output["legacy_variant"] = data(legacy, "Variant").strip('"')
        print(json.dumps(output), flush=True)
        diff = {key: (expected, output.get(key)) for key, expected in reference.items() if output.get(key) != expected}
        assert not diff, diff
        print(f"Matched all {len(reference)} Crab Java behavior fields")
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
