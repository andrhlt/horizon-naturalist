"""Read the horizontal impulses after real overlapping entities tick."""
import re


def capture_contact(command, step):
    command("gamerule minecraft:spawn_mobs false")
    command("fill 2 92 2 18 99 17 minecraft:air")
    out = {}
    cases = (
        ("jellyfish_pair", "naturalist:jellyfish", "naturalist:jellyfish", 9.1, 6.5),
        ("shark_cow", "naturalist:great_white_shark", "minecraft:cow", 9.25, 6.95),
        ("whale_cow", "naturalist:whale", "minecraft:cow", 9.7, 6.5),
        ("starfish_cow", "naturalist:starfish", "minecraft:cow", 9.1, 6.5),
        ("clam_cow", "naturalist:clam", "minecraft:cow", 9.2, 6.5),
        ("cow_pair", "minecraft:cow", "minecraft:cow", 9.1, 6.5),
    )
    for name, first, second, x, z in cases:
        command("kill @e[tag=AquaticContactFixture]")
        step(21)
        for label, species, cx, cz in (("One", first, 8.5, 6.5), ("Two", second, x, z)):
            rows = command(f"summon {species} {cx} 94.0 {cz} "
                           f"{{NoAI:1b,NoGravity:1b,Invulnerable:1b,Rotation:[0f,0f],"
                           f"Tags:[AquaticContactFixture,AquaticContact{label}]}}")
            assert any("Summoned" in row for row in rows), rows
        step(2)
        impulses = []
        for label in ("One", "Two"):
            rows = command(f"data get entity @e[tag=AquaticContact{label},limit=1] Motion")
            velocity = next([float(raw) for raw in re.findall(r"([-+0-9.eE]+)d", row)]
                            for row in rows if "following entity data:" in row)
            impulses.append(round(velocity[0], 6))
        out[name] = impulses
    return out
