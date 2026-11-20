def arena(cmd, step):
    def nether(command):
        return cmd(f"execute in minecraft:the_nether run {command}")

    cmd("gamerule minecraft:spawn_mobs false")
    nether("forceload add 0 0 31 15")
    cmd("tick freeze")
    nether("kill @e[tag=fixed_light]")
    nether("fill 3 99 3 14 99 13 minecraft:glowstone")
    nether("fill 3 100 3 28 105 13 minecraft:air")
    nether("fill 18 96 1 31 104 15 minecraft:stone")
    nether("fill 19 100 2 30 102 14 minecraft:air")
    step(20)
    for group, origin in [("bright", 5), ("dark", 22)]:
        for slot in range(16):
            x = origin + slot % 4
            z = 5 + slot // 4
            rows = nether(f"summon naturalist:firefly {x}.5 100 {z}.5 "
                          f"{{Tags:[fixed_light,{group}_group],NoAI:1b,NoGravity:1b,PersistenceRequired:1b}}")
            assert any("Summoned" in row for row in rows), rows
