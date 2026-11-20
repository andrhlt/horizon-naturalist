"""Compare the fawn's Java grazing effect under controlled world ticks."""
import re
import time


def capture(command, y):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:mob_griefing true")
    command("gamerule minecraft:random_tick_speed 0")
    command("forceload add 0 0")
    time.sleep(2.0)
    command("kill @e[tag=grazing_fawn]")
    command("tick freeze")
    command(f"fill 7 {y} 7 9 {y + 3} 9 minecraft:stone")
    command(f"fill 8 {y} 8 8 {y + 3} 8 minecraft:air")
    command(f"setblock 8 {y - 1} 8 minecraft:grass_block")
    command(f"summon naturalist:deer 8.5 {y} 8.5 {{Age:-24000,Tags:[grazing_fawn]}}")

    def number(query, pattern):
        rows = command(query)
        for row in rows:
            if (value := re.search(pattern, row)) is not None:
                return int(value[1])
        raise AssertionError((query, rows))

    age_query = "data get entity @e[tag=grazing_fawn,limit=1] Age"
    age_pattern = r"following entity data: (-?\d+)"
    time_query = "time query gametime"
    time_pattern = r"The (?:game )?time is (\d+)"
    initial = number(age_query, age_pattern)
    start = number(time_query, time_pattern)
    target_ticks = 750
    command(f"tick sprint {target_ticks}")
    deadline = time.monotonic() + 45
    while (end := number(time_query, time_pattern)) - start < target_ticks:
        assert time.monotonic() < deadline, (start, end)
        time.sleep(0.2)
    command("tick freeze")
    end = number(time_query, time_pattern)
    age = number(age_query, age_pattern)
    growth = age - initial - (end - start)
    dirt = command(f"execute if block 8 {y - 1} 8 minecraft:dirt")
    result = {"extra_age_ticks": growth,
              "grass_transformed_to_dirt": any("Test passed" in row for row in dirt)}
    assert result == {"extra_age_ticks": 1200, "grass_transformed_to_dirt": True}, (result, initial, age, start, end, dirt)
    return result
