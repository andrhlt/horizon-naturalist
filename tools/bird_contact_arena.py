"""Compare actual Bird contact impulses with another Bird and with a sheep."""
from firefly_grass_arena import stepper


def capture(cmd, wait):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:max_entity_cramming 24")
    cmd("forceload add 0 0")
    cmd("gamerule minecraft:advance_time false")
    cmd("time set noon")
    cmd("weather clear 120")
    cmd("tick freeze")
    cmd("kill @e[tag=bird_contact]")
    cmd("fill 1 99 1 14 99 14 minecraft:stone")
    cmd("fill 1 100 1 14 112 14 minecraft:air")
    wait(30)
    cases = {
        "a": ("naturalist:bird", 5.3, 5.5),
        "b": ("naturalist:bird", 5.55, 5.5),
        "mixed_bird": ("naturalist:bird", 10.3, 10.5),
        "sheep": ("minecraft:sheep", 10.55, 10.5),
    }
    for label, (kind, x, z) in cases.items():
        rows = cmd(f"summon {kind} {x} 105 {z} "
                   f"{{Tags:[bird_contact,c_{label}],PersistenceRequired:1b,NoAI:1b,NoGravity:1b}}")
        assert any("Summoned" in row for row in rows), rows

    def data(label, field):
        rows = cmd(f"data get entity @e[tag=c_{label},limit=1] {field}")
        snbt = next(row.split("following entity data: ", 1)[1] for row in rows
                    if "following entity data: " in row)
        return [float(s.strip(" []d")) for s in snbt.split(",")]

    positions = []
    for step in range(7):
        if step:
            wait(1)
        positions.append({"tick": step,
                          "cases": {label: {"pos": data(label, "Pos"), "motion": data(label, "Motion")}
                                    for label in cases}})
    cmd("kill @e[tag=bird_contact]")
    cmd("gamerule minecraft:max_entity_cramming 1")
    for label, invulnerable in (("vulnerable", 0), ("protected", 1)):
        rows = cmd(f"summon naturalist:bird 8.5 105 8.5 {{Tags:[bird_contact,c_{label}],"
                   f"PersistenceRequired:1b,NoAI:1b,NoGravity:1b,Invulnerable:{invulnerable}b}}")
        assert any("Summoned" in row for row in rows), rows

    def health(label):
        rows = cmd(f"data get entity @e[tag=c_{label},limit=1] Health")
        return next((float(row.split("following entity data: ", 1)[1].removesuffix("f"))
                     for row in rows if "following entity data: " in row), None)

    health_before = {label: health(label) for label in ("vulnerable", "protected")}
    wait(72)
    health_after = {label: health(label) for label in health_before}
    return {"motion": positions, "cramming": {"before": health_before, "after": health_after}}
