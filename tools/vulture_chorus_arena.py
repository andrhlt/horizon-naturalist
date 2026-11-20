"""Observe Vultures finishing ordinary chorus fruit after real server ticks."""
import re
from firefly_grass_arena import stepper


def capture(cmd, wait):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("forceload add -16 -16 31 31")
    cmd("gamerule minecraft:advance_time false")
    cmd("time set noon")
    cmd("weather clear 120")
    cmd("tick freeze")
    cmd("kill @e[tag=vulture_chorus]")
    wait(30)
    cmd("fill -8 99 -8 23 99 23 minecraft:stone")
    cmd("fill -8 100 -8 23 109 23 minecraft:air")
    cmd("fill 3 100 3 13 104 13 minecraft:stone")
    cases = {"left": (5, 5), "center": (8, 8), "right": (11, 11)}
    for x, z in cases.values():
        cmd(f"fill {x-1} 100 {z-1} {x} 102 {z} minecraft:air")
    wait(30)
    for key, (x, z) in cases.items():
        target = f"@e[tag=vulture_chorus,tag=chorus_{key},limit=1]"
        rows = cmd(f"summon naturalist:vulture {x} 100 {z} {{Tags:[vulture_chorus,chorus_{key}],PersistenceRequired:1b}}")
        assert any("Summoned" in row for row in rows), rows
        cmd(f"item replace entity {target} weapon.mainhand with minecraft:chorus_fruit")

    def read(key, field):
        target = f"@e[tag=vulture_chorus,tag=chorus_{key},limit=1]"
        return next((msg.split("following entity data: ", 1)[-1]
                     for msg in cmd(f"data get entity {target} {field}")
                     if "following entity data: " in msg), None)

    def snapshot():
        return {key: {"hand": read(key, "equipment.mainhand"),
                      "position": [float(value) for value in
                                   re.findall(r"(-?[\d.]+(?:e[+-]?\d+)?)d", read(key, "Pos"))]}
                for key in cases}

    wait(600)
    cmd("fill 3 100 3 13 104 13 minecraft:air")
    for key, (x, z) in cases.items():
        cmd(f"tp @e[tag=vulture_chorus,tag=chorus_{key},limit=1] {x} 105 {z}")
    before = snapshot()
    wait(1)
    return {"before": before, "after": snapshot()}
