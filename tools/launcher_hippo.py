"""Compare Java Hippo pet interactions, water breeding and boat attacks headlessly."""
import json
from pathlib import Path
import re
import sys
import time
import subprocess

from launcher_selected_headless import LAUNCHER, SelectedHeadless

version = Path(sys.argv[1])
java = json.loads(Path(sys.argv[2]).read_text())
spawn_java = json.loads(Path(sys.argv[3]).read_text()) if len(sys.argv) > 3 else None
world = f"APP2129 Hippo {time.time_ns()}"
socket = Path("/private/tmp") / f"{world.replace(' ', '-')}.sock"
client = SelectedHeadless(version, socket, world, "--creative")
ctl = client.call
cmd = client.command

def data(target, name):
    return next(row.split("following entity data: ", 1)[1] for row in
        cmd(f"data get entity {target} {name}") if "following entity data: " in row)

def step(n):
    cmd(f"tick step {n}")
    ctl("wait", n + 2)

def inspect(query):
    marker = "Hippo fixture "
    return next(json.loads(row.split(marker, 1)[1]) for row in
                cmd(f"naturalist_fixture_hippo {query}") if marker in row)["mobs"]

def block(x, y, z, name):
    return any("Test passed" in row or "test passed" in row or "succeeded" in row
               for row in cmd(f"execute if block {x} {y} {z} {name}"))

try:
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:mob_griefing true")
    cmd("tp @s 6.5 92 3.5")
    ctl("wait", 20)
    cmd("tick freeze")
    cmd("fill 3 91 1 15 91 16 minecraft:stone")
    cmd("fill 3 92 1 15 97 16 minecraft:air")
    cmd("fill 5 92 8 14 95 15 minecraft:water")
    cmd("tp @s 6.5 92 3.5")
    cmd("gamemode survival @s")
    cmd('summon naturalist:hippo 6.5 92 5.5 {NoAI:1b,NoGravity:1b,Tags:[adult]}')
    cmd('summon naturalist:hippo 8.5 92 5.5 {NoAI:1b,NoGravity:1b,Age:-24000,Tags:[baby]}')
    step(2)
    keys = {"adult": "@e[tag=adult,limit=1]", "baby": "@e[tag=baby,limit=1]"}
    adult, baby = keys["adult"], keys["baby"]
    [a], [b] = inspect("adult"), inspect("baby")
    output = {
        "variant": data(adult, "Variant").strip('"'),
        "adult_dims": a["dims"], "baby_dims": b["dims"], "food": a["food"],
        "adult_float_threshold": a["threshold"], "baby_float_threshold": b["threshold"],
        "knockback": [a["knockback"], b["knockback"]],
        "portal": a["portal"], "cluster": a["cluster"],
    }
    for attr, key in (
        ("max_health", "health"), ("movement_speed", "speed"), ("attack_damage", "attack"),
        ("follow_range", "follow_range"), ("knockback_resistance", "knockback_resistance"),
        ("step_height", "step"),
    ):
        rows = cmd(f"attribute {adult} minecraft:{attr} base get")
        value = next((m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
        assert value is not None, (attr, rows)
        output[key] = float(value)

    def click(query):
        [mob] = inspect(query)
        looked = ctl("look-at-entity", mob["id"])
        assert looked["target"]["kind"] == "entity" and looked["target"]["entity"]["id"] == mob["id"], looked
        ctl("mouse", "right", "click")
        ctl("wait", 5)

    def held(item, n=1):
        cmd(f"item replace entity @s weapon.mainhand with {item} {n}")
        ctl("wait", 3)

    held("minecraft:melon", 8)
    click("adult")
    [a] = inspect("adult")
    output["adult_tame"] = a["tame"]
    output["adult_in_love"] = int(data(adult, "InLove"))
    output["adult_carrying_melon"] = a["occupied"][0] == "minecraft:melon"
    output["adult_carried_count"] = int(a["occupied"][1])
    step(10)
    output["melon_cleared_at_10"] = inspect("adult")[0]["occupied"] is None
    step(1)
    output["melon_cleared_at_11"] = inspect("adult")[0]["occupied"] is None

    cmd("tp @s 8.5 92 3.5")
    ctl("wait", 4)
    prior_age = inspect("baby")[0]["age"]
    click("baby")
    [b] = inspect("baby")
    expected_owner = [int(part) for part in re.findall(r"-?\d+", data("@s", "UUID"))]
    output["baby_tamed"] = b["tame"] and b["owner"] == expected_owner
    output["baby_ordered_to_sit"] = b["sitting"]
    output["taming_did_not_age"] = b["age"] == prior_age
    output["following_at_tame"] = b["following"]
    cmd(f"damage {baby} 7 minecraft:generic")
    hurt_health = float(data(baby, "Health").rstrip("fF"))
    click("baby")
    output["melon_heal"] = round(float(data(baby, "Health").rstrip("fF")) - hurt_health)
    cmd(f"data merge entity {baby} {{Health:70f,Age:-24000}}")
    click("baby")
    output["healthy_baby_age"] = inspect("baby")[0]["age"]
    held("naturalist:whistle")
    click("baby")
    [b] = inspect("baby")
    output["whistle_unseats_and_stops_follow"] = not b["sitting"] and not b["following"]
    held("minecraft:air")
    click("baby")
    output["plain_empty_sits"] = inspect("baby")[0]["sitting"]
    ctl("key", "shift", "down")
    ctl("wait", 8)
    try:
        click("baby")
    finally:
        ctl("key", "shift", "up")
    output["crouch_empty_sits"] = inspect("baby")[0]["sitting"]

    def clean():
        cmd("kill @e[type=naturalist:hippo]")
        cmd("kill @e[tag=hippo_boat]")
        step(22)

    cmd("gamemode creative @s")
    cmd("tp @s 6.5 110 3.5")
    cmd("fill 3 109 1 15 109 16 minecraft:stone")
    for phase, first, second in (
        ("can_mate_dry", "6.5 110 5.5", "8.5 110 5.5"),
        ("can_mate_one_wet", "6.5 109.2 8.5", "6.5 110 6.5"),
        ("can_mate_both_wet", "8.5 112.6 10.5", "10.5 112.6 10.5"),
    ):
        clean()
        cmd("fill 3 110 1 15 116 16 minecraft:air")
        cmd("fill 5 108 8 14 108 15 minecraft:stone")
        floor = "stone" if phase == "can_mate_dry" else "water"
        cmd(f"fill 5 109 8 14 109 15 minecraft:{floor}")
        if phase == "can_mate_both_wet":
            cmd("fill 5 110 8 14 113 15 minecraft:water")
        cmd("summon naturalist:hippo " + first + " {InLove:600,Tags:[parent1]}")
        fixed_partner = "NoAI:1b,"
        cmd("summon naturalist:hippo " + second + " {" + fixed_partner + "NoGravity:1b,InLove:600,Tags:[parent2]}")
        step(3)
        contacts = [entry["wet"] for entry in inspect("adult")]
        assert sum(contacts) == {"can_mate_dry": 0, "can_mate_one_wet": 1, "can_mate_both_wet": 2}[phase], (phase, contacts)
        if phase == "can_mate_both_wet":
            output["both_in_water"] = all(contacts)
        step(75)
        output[phase] = len(inspect("baby")) > 0
        print(phase, contacts, output[phase], flush=True)

    cmd("tp @s 6.5 92 3.5")
    for phase, boat, juvenile in (
        ("boat_adult_goal", "minecraft:oak_boat", False),
        ("chest_boat_adult_goal", "minecraft:oak_chest_boat", False),
        ("raft_adult_goal", "minecraft:bamboo_raft", False),
        ("boat_baby_goal", "minecraft:oak_boat", True),
    ):
        clean()
        age = "Age:-24000," if juvenile else ""
        cmd("summon naturalist:hippo 6.5 92 5.5 {" + age + "NoGravity:1b,Tags:[boat_hunter]}")
        boat_pos = "10.5 92 8.5" if phase == "boat_adult_goal" else "7.4 92 6.4"
        cmd("summon " + boat + " " + boat_pos + " {NoGravity:1b,Tags:[hippo_boat]}")
        destroyed = False
        if phase == "boat_adult_goal":
            before = inspect("boat_hunter")[0]["position"]
            step(12)
            after = inspect("boat_hunter")[0]["position"]
            boat_xyz = [float(value) for value in boat_pos.split()]
            before_distance = sum((before[i] - boat_xyz[i]) ** 2 for i in range(3))
            after_distance = sum((after[i] - boat_xyz[i]) ** 2 for i in range(3))
            output["adult_boat_approach"] = after_distance < before_distance
            assert output["adult_boat_approach"], {"before": before, "after": after, "boat": boat_xyz}
        for _ in range(10):
            step(6)
            if not any("Test passed" in row for row in cmd("execute if entity @e[tag=hippo_boat]")):
                destroyed = True
                break
        output[phase] = destroyed
        if phase == "boat_adult_goal":
            output["boat_destroyed"] = destroyed
    print(json.dumps(output), flush=True)
    mismatch = {key: [expected, output.get(key)] for key, expected in java.items()
                if output.get(key) != expected}
    assert not mismatch, mismatch
    print(f"Hippo matched all {len(java)} pinned Java readings.", flush=True)

    if spawn_java:
        cmd("tp @s 8.5 132 8.5")
        step(6)
        cmd("time set 6000")
        cmd("weather clear")
        cmd("setblock 8 131 8 minecraft:grass_block")
        def placement(generation=False):
            kind = "generation" if generation else "natural"
            marker = "Hippo fixture "
            return next(json.loads(row.split(marker, 1)[1])["allowed"]
                for row in cmd(f"naturalist_fixture_hippo spawn-{kind}") if marker in row)

        checked = {"no_water": placement()}
        cmd("setblock 24 132 24 minecraft:water")
        checked["natural_corner_16"] = placement()
        checked["generated_cross_chunk"] = placement(True)
        cmd("setblock 24 132 24 minecraft:air")
        for case, pos in (("horizontal_17", "25 132 8"), ("two_above", "11 134 8"), ("two_below", "11 130 8")):
            cmd("setblock " + pos + " minecraft:water")
            checked[case] = placement()
            cmd("setblock " + pos + " minecraft:air")
        cmd("setblock 15 131 15 minecraft:oak_slab[waterlogged=true]")
        checked["waterlogged_one_below"] = placement()
        checked["generated_same_chunk"] = placement(True)
        cmd("setblock 8 131 8 minecraft:stone")
        checked["stone_rejected"] = placement()
        print("Spawn positions", json.dumps(checked), flush=True)
        assert checked == spawn_java, (checked, spawn_java)
        print(f"Hippo matched all {len(checked)} pinned Java spawn placements.", flush=True)

finally:
    client.close()
    subprocess.run([LAUNCHER, "worlds", "delete", "--instance", "horizon", world],
                   capture_output=True, text=True, env=client.env)
