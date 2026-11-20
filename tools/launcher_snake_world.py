"""Exercise physical Snake rattle, poison, food pickup and spit in Horizon."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text()) if len(sys.argv) > 2 else None

with tempfile.TemporaryDirectory(prefix="nat-snake-world-", ignore_cleanup_errors=True) as folder:
    home = Path(folder)
    socket = home / "control.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    def state():
        prefix = "Snake fixture "
        for line in cmd("naturalist_fixture_snake adult"):
            if prefix in line:
                return json.loads(line.split(prefix, 1)[1])
        raise AssertionError("Snake marker missing")

    def effects():
        return " ".join(cmd("data get entity @s active_effects"))

    def place(kind, fields=None, x=6.5):
        tail = f',{fields}' if fields else ""
        rows = cmd(f'summon naturalist:snake {x} 92 5.5 '
                   f'{{Variant:"naturalist:{kind}",Tags:[under_test],Silent:1b{tail}}}')
        assert any("Summoned" in line for line in rows), rows

    def remove():
        cmd("kill @e[tag=under_test]")
        step(24)

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist Snake physical behavior", "--creative",
            "--test-terrain", "--render-distance", 4, "--wait")
        cmd("difficulty normal")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:advance_time false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("tp @s 7.5 92 5.5")
        ctl("wait", 24)
        cmd("tick freeze")
        cmd("fill 2 91 2 28 91 15 minecraft:stone")
        cmd("fill 2 92 2 28 99 15 minecraft:air")
        cmd("tp @s 8.5 92 5.5")
        cmd("time set 1000")
        step(25)
        checked = {}

        place("rattlesnake", "NoAI:1b")
        step(4)
        rattler = state()
        print("Rattler after creative close NoAI tick", json.dumps(rattler), flush=True)
        checked["creative_close_rattle"] = rattler["rattling"]
        cmd("setblock 7 92 5 minecraft:stone")
        cmd("setblock 7 93 5 minecraft:stone")
        step(4)
        checked["wall_blocks_rattle"] = not state()["rattling"]
        cmd("setblock 7 92 5 minecraft:air")
        cmd("setblock 7 93 5 minecraft:air")
        cmd("tp @s 12.5 92 5.5")
        step(4)
        checked["distance_blocks_rattle"] = not state()["rattling"]
        remove()

        cmd("tp @s 7.5 92 5.5")
        cmd("gamemode survival @s")
        place("rattlesnake", "CanPickUpLoot:1b")
        checked["survival_bite_poison"] = False
        for _ in range(10):
            step(4)
            if "minecraft:poison" in effects():
                checked["survival_bite_poison"] = True
                break
        remove()
        cmd("effect clear @s")
        cmd("gamemode spectator @s")
        cmd("tp @s 7.5 92 3.5")
        place("snake", "CanPickUpLoot:1b")
        cmd('summon minecraft:item 6.5 92 5.5 {Item:{id:"minecraft:chicken",count:3},PickupDelay:0s,Tags:[meal]}')
        step(4)
        checked["picks_up_complete_food_stack"] = state()["main_hand_count"] == 3
        cmd("damage @e[tag=under_test,limit=1] 1 minecraft:generic")
        step(1)
        observation = state()
        checked["hurt_spits_food_with_delay_and_thrower"] = (
            observation["main_hand_count"] == 0 and observation["spit"] is not None
            and observation["spit"][:2] == ["minecraft:chicken", 3]
            and 77 <= observation["spit"][2] <= 80
        )
        if reference is not None:
            cmd("kill @e[type=minecraft:item]")
            cmd('data merge entity @e[tag=under_test,limit=1] {Invulnerable:1b}')
            cmd('item replace entity @e[tag=under_test,limit=1] weapon.mainhand with minecraft:chicken 3')
            before = cmd("data get entity @e[tag=under_test,limit=1] Health")
            cmd("damage @e[tag=under_test,limit=1] 1 minecraft:generic")
            step(1)
            after = cmd("data get entity @e[tag=under_test,limit=1] Health")
            invulnerable = state()
            expected_damage, expected_empty, expected_count, expected_delay, expected_thrower, _ = (
                reference["invulnerable_hurt_spit"]
            )
            spit = invulnerable["spit"]
            checked["invulnerable_spit_matches_java"] = (
                (before != after) == expected_damage
                and (invulnerable["main_hand_count"] == 0) == expected_empty
                and (spit is not None) == expected_thrower
                and spit[1] == expected_count
                and expected_delay - 3 <= spit[2] <= expected_delay
            )
        print(json.dumps(checked), flush=True)
        assert all(checked.values()), (checked, observation)
    finally:
        if socket.exists():
            ctl("quit")
