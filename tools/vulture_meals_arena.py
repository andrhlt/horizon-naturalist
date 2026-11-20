"""Measure which effects and containers actually follow a Vulture meal."""
from firefly_grass_arena import stepper


def capture(cmd, wait):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:mob_griefing false")
    cmd("forceload add 0 0")
    cmd("gamerule minecraft:advance_time false")
    cmd("time set noon")
    cmd("weather clear 120")
    cmd("tick freeze")
    cmd("kill @e[tag=vulture_meals]")
    cmd("fill 1 99 1 14 104 14 minecraft:stone")
    cases = {
        "gold": (2, 2, "minecraft:golden_apple", 2, 0),
        "stew": (7, 2, "minecraft:mushroom_stew", 1, 0),
        "honey": (12, 2, "minecraft:honey_bottle", 2, 0),
        "paused": (2, 8, "minecraft:golden_apple", 1, 1),
        "rotten": (7, 8, "minecraft:rotten_flesh", 2, 0),
        "honey_cure": (12, 8, "minecraft:honey_bottle", 1, 0),
        "no_consumable": (2, 12, "minecraft:golden_apple[!minecraft:consumable]", 1, 0),
        "no_food": (7, 12, "minecraft:golden_apple[!minecraft:food]", 1, 0),
        "stew_effect": (12, 12, 'minecraft:suspicious_stew[minecraft:suspicious_stew_effects='
                        '[{id:"minecraft:night_vision",duration:200}]]', 1, 0),
    }
    for x, z, *_ in cases.values():
        cmd(f"fill {x} 100 {z} {x+1} 102 {z+1} minecraft:air")
    wait(30)
    for label, (x, z, item, count, no_ai) in cases.items():
        rows = cmd(f"summon naturalist:vulture {x+1} 100 {z+1} "
                   f"{{Tags:[vulture_meals,meal_{label}],PersistenceRequired:1b,NoAI:{no_ai}b}}")
        assert any("Summoned" in row for row in rows), rows
        cmd(f"item replace entity @e[tag=meal_{label},limit=1] weapon.mainhand with {item} {count}")

    def data(label, field):
        rows = cmd(f"data get entity @e[tag=meal_{label},limit=1] {field}")
        return next((row.split("following entity data: ", 1)[1]
                     for row in rows if "following entity data: " in row), None)

    def snapshot():
        return {label: {"hand": data(label, "equipment.mainhand"), "effects":data(label, "active_effects"),
                        "health":data(label, "Health"), "absorption":data(label, "AbsorptionAmount")}
                for label in cases}
    before = snapshot()
    wait(600)
    cmd("effect give @e[tag=meal_honey_cure,limit=1] minecraft:poison 5")
    at_threshold = snapshot()
    wait(1)
    after = snapshot()
    wait(6)
    return {"before":before,"at_threshold":at_threshold,"after":after,"later":snapshot()}
