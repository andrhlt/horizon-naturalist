"""Observe actual Firefly grass arrival and particles in a clear bright arena."""
import re
import time


def stepper(command):
    def now():
        return next(int(m[1]) for row in command("time query gametime")
                    if (m := re.search(r"The game time is (\d+)", row)))

    def step(ticks):
        goal = now() + ticks
        command(f"tick step {ticks}")
        deadline = time.monotonic() + max(8, ticks / 15)
        while time.monotonic() < deadline:
            position = now()
            if position >= goal:
                assert position == goal, (position, goal)
                return
            time.sleep(0.05)
        raise AssertionError(("step failed", goal, now()))

    return step


def capture(cmd, wait, particles=lambda: None, *, scene="noon", interval=4):
    if scene not in ("noon", "canopy"):
        raise ValueError(scene)
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("forceload add 0 0")
    cmd("kill @e[tag=firefly_grass]")
    cmd("fill 1 99 1 14 99 14 minecraft:grass_block")
    cmd("fill 1 100 1 14 110 14 minecraft:air")
    for x, z, species in [(8, 5, "short_grass"), (8, 8, "fern"), (8, 11, "tall_grass[half=lower]")]:
        cmd(f"setblock {x} 100 {z} minecraft:{species}")
    cmd("setblock 8 101 11 minecraft:tall_grass[half=upper]")
    cmd("gamerule minecraft:advance_time false")
    cmd("weather clear 120")
    if scene == "canopy":
        cmd("fill 4 103 3 14 103 13 minecraft:stone")
    cmd("time set noon" if scene == "noon" else "time set 12400")
    cmd("tick freeze")
    wait(10)

    def data(tag, key):
        return next((row.split("following entity data: ", 1)[1]
                     for row in cmd(f"data get entity @e[tag={tag},limit=1] {key}")
                     if "following entity data: " in row), None)

    def position(tag):
        val = data(tag, "Pos")
        return None if val is None else [float(s.strip(" []d")) for s in val.split(",")]

    labels = ["short", "fern", "tall", "named"]
    for tag, z, extra in [("short", 5, ""), ("fern", 8, ",Age:-24000"),
                          ("tall", 11, ""), ("named", 2, ',CustomName:"Beacon"')]:
        rows = cmd(f"summon naturalist:firefly 5.5 100.5 {z}.5 {{Tags:[firefly_grass,{tag}]{extra}}}")
        assert any("Summoned" in row for row in rows), rows

    observations = []
    initial = particles()
    for tick in range(0, 45, interval):
        if tick:
            wait(interval)
        positions = {tag: position(tag) for tag in labels}
        observations.append({"tick": tick, "positions": positions, "block_particles": particles()})
        if all(positions[tag] is None for tag in labels[:-1]):
            break
    assert observations[-1]["positions"]["named"] is not None, observations
    return {"scene": scene, "initial_block_particles": initial, "observations": observations}
