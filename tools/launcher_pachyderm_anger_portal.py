"""Measure the saved anger reference when its tracked attacker leaves the dimension."""
import argparse
import json
from pathlib import Path
import re
import time

import launcher_pachyderm_family as controller


def capture(cmd, step, species):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:advance_time false")
    cmd("forceload add 0 0 15 15")
    cmd("execute in minecraft:the_nether run forceload add 0 0 15 15")
    cmd("tick freeze")
    cmd("fill 2 101 2 15 101 15 minecraft:stone")
    cmd("fill 2 102 2 15 107 15 minecraft:air")
    cmd("execute in minecraft:overworld positioned 8.5 102 8.5 run kill @e[tag=anger_target_world,distance=..40]")
    cmd("execute in minecraft:the_nether positioned 8.5 105 8.5 run kill @e[tag=anger_target_world,distance=..40]")
    step(25)

    def data(label, field, dimension=None):
        marker = "following entity data: "
        route = f"execute in minecraft:{dimension or 'overworld'} positioned 8.5 102 8.5 run "
        rows = cmd(f"{route}data get entity @e[tag={label},distance=..32,limit=1] {field}")
        return next((row.split(marker, 1)[1] for row in rows if marker in row), None)

    def sample():
        clock = next(int(match[1]) for row in cmd("time query gametime")
                     if (match := re.search(r"The game time is (\d+)", row)))
        return {"source_overworld": data("cross_attacker", "UUID"),
                "source_nether": data("cross_attacker", "UUID", "the_nether"),
                "actor": data("cross_animal", "angry_at"),
                "end": data("cross_animal", "anger_end_time"),
                "time": clock, "health": data("cross_animal", "Health")}

    cmd(f"summon naturalist:{species} 7.5 102 8.5 {{Age:0,OnGround:1b,Tags:[cross_animal,anger_target_world]}}")
    cmd('summon minecraft:pig 11.5 102 8.5 {NoAI:1b,Health:1000f,Tags:[cross_attacker,anger_target_world],'
        'attributes:[{id:"minecraft:max_health",base:1000},{id:"minecraft:knockback_resistance",base:1}]}')
    step(8)
    cmd("damage @e[tag=cross_animal,limit=1] 1 minecraft:mob_attack by @e[tag=cross_attacker,limit=1]")
    step(8)
    before = sample()
    print("before physical transfer", before, flush=True)
    assert before["source_overworld"] and before["actor"] == before["source_overworld"], before
    cmd("tp @e[tag=cross_attacker,limit=1] 13.5 102 8.5")
    frame = cmd("fill 12 101 8 15 105 8 minecraft:obsidian outline")
    eye = cmd("fill 13 102 8 14 104 8 minecraft:nether_portal[axis=x]")
    print("physical portal", frame, eye, flush=True)
    for _ in range(12):
        step(1)
        after = sample()
        if after["source_overworld"] is None:
            break
    step(3)
    later = sample()
    assert after["source_overworld"] is None and later["source_overworld"] is None, (after, later)
    assert later["health"] == before["health"], (before, later)
    result = {"before": before, "after": after, "later": later}
    print(json.dumps(result, indent=2), flush=True)
    return result


def compare(result, reference):
    for data in (reference, result):
        before, after, later = (data[key] for key in ("before", "after", "later"))
        source = before["source_overworld"]
        assert source and before["actor"] == source, data
        assert not after["source_overworld"] and not later["source_overworld"], data
        assert after["source_nether"] == later["source_nether"] == source, data
        assert after["actor"] == later["actor"] == source, data
        assert after["end"] and after["end"] == later["end"], data
        assert before["health"] == after["health"] == later["health"], data


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--java-home", type=Path)
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--species", choices=("elephant", "mammoth"), default="elephant")
    args = parser.parse_args()
    controller.capture = lambda cmd, step: capture(cmd, step, args.species)
    if args.java_home:
        import java_reference

        class Args:
            behavior = True
            variants = grazing = False
            server, java_home, output = args.root, args.java_home, args.output

        def run(cmd, _):
            def step(ticks):
                cmd(f"tick step {ticks}")
                time.sleep(ticks / 20 + 0.15)
            return capture(cmd, step, args.species)
        java_reference.deer_behavior.capture = run
        java_reference.run_server(Args(), ["sh", "run.sh", "nogui", "--world", f"pachyderm-portal-target-{args.species}-fixture"])
    else:
        result = controller.capture_horizon(args.root)
        args.output.write_text(json.dumps(result, indent=2) + "\n")
        if args.reference:
            compare(result, json.loads(args.reference.read_text()))
