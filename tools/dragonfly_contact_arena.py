"""Observe actual Dragonfly motion on clear air, solid and fluid contact."""
from firefly_grass_arena import stepper


def capture(cmd, wait, status=lambda: None):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("forceload add 0 0")
    cmd("gamerule minecraft:advance_time false")
    cmd("time set noon")
    cmd("weather clear 120")
    cmd("tick freeze")
    cmd("kill @e[tag=dragonfly_motion]")
    cmd("fill 1 99 1 14 99 14 minecraft:stone")
    cmd("fill 1 100 1 14 111 14 minecraft:air")
    cmd("fill 8 100 3 8 108 7 minecraft:stone")
    cmd("fill 10 100 2 14 102 10 minecraft:stone")
    cmd("fill 11 100 3 13 102 9 minecraft:water")
    wait(10)

    cases = {
        "air": ("3.5 105 3.5", "0.45d,-0.2d,0.2d"),
        "wall": ("7.5 105 5.5", "0.60d,0.1d,0.2d"),
        "submerged": ("12.5 101.05 4.5", "0.1d,-0.15d,0.15d"),
        "entry": ("12.5 103.5 7.5", "0.1d,-0.7d,0.1d"),
        "peer": ("5.5 104 11.5", "0.0d,0.0d,0.0d"),
        "ground": ("3.5 101 9.5", "0.2d,-0.8d,0.1d"),
    }
    for label, (position, motion) in cases.items():
        rows = cmd(f"summon naturalist:dragonfly {position} "
                   f"{{Tags:[dragonfly_motion,df_{label}],PersistenceRequired:1b,Motion:[{motion}]}}")
        assert any("Summoned" in row for row in rows), rows
    rows = cmd("summon minecraft:sheep 5.62 104 11.5 {Tags:[dragonfly_motion,df_sheep],NoGravity:1b,NoAI:1b}")
    assert any("Summoned" in row for row in rows), rows

    def data(label, field):
        rows = cmd(f"data get entity @e[tag=df_{label},limit=1] {field}")
        snbt = next(row.split("following entity data: ", 1)[1]
                    for row in rows if "following entity data: " in row)
        return [float(s.strip(" []d")) for s in snbt.split(",")]

    samples = []
    for tick in range(9):
        if tick:
            wait(1)
        samples.append({"tick": tick,
                        "cases": {label: {"position": data(label, "Pos"), "motion": data(label, "Motion")}
                                  for label in (*cases, "sheep")},
                        "effects": status()})
    return samples


def vanilla_entry(cmd, wait, status):
    cmd("kill @e[tag=dragonfly_motion]")
    before = status()
    rows = cmd("summon minecraft:pig 12.5 103.5 7.5 "
               "{Tags:[dragonfly_motion],NoGravity:1b,Motion:[0.0d,-0.7d,0.0d]}")
    assert any("Summoned" in row for row in rows), rows
    wait(10)
    return {"before": before, "after": status()}
