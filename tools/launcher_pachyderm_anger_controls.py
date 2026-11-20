"""Compare modern saved/legacy anger with an absent or dead attacker in both games."""
import argparse
import json
from pathlib import Path
import re

import launcher_pachyderm_family as controller


def capture(cmd, step):
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("gamerule minecraft:advance_time false")
    cmd("forceload add 0 0")
    cmd("tick freeze")
    cmd("fill 2 101 2 15 101 15 minecraft:stone")
    cmd("fill 2 102 2 15 107 15 minecraft:air")

    def data(label, key):
        marker = "following entity data: "
        rows = cmd(f"data get entity @e[tag={label},limit=1] {key}")
        return next((row.split(marker, 1)[1] for row in rows if marker in row), None)

    def game_time():
        rows = cmd("time query gametime")
        return next(int(match[1]) for row in rows if (match := re.search(r"The game time is (\d+)", row)))

    def sample():
        anger_end = data("anger_probe", "anger_end_time")
        return {
            "actor": data("anger_probe", "angry_at"),
            "end": int(anger_end.rstrip("L")) if anger_end is not None else None,
            "time": game_time(),
        }

    result = {}
    for species in ("elephant", "mammoth"):
        for condition in ("fresh", "early_expiry", "legacy", "dead"):
            cmd("kill @e[tag=anger_probe]")
            cmd("kill @e[tag=anger_source]")
            step(25)
            start = game_time()
            actor_id = f"[I; {start}, -777242387, -24886356, 945084990]"
            duration = 20
            payload = ""
            if condition in ("early_expiry", "dead"):
                absolute = start + duration
                payload = f",angry_at:{actor_id},anger_end_time:{absolute}L"
            elif condition == "legacy":
                payload = f",angry_at:{actor_id},AngerTime:{duration}"
            if condition == "dead":
                cmd(f"summon minecraft:pig 11.5 102 8.5 {{NoAI:1b,UUID:{actor_id},Tags:[anger_source]}}")
            cmd(f"summon naturalist:{species} 8.5 102 8.5 {{NoAI:1b,Tags:[anger_probe]{payload}}}")
            initial = sample()
            step(2)
            active = sample()
            if condition == "dead":
                cmd("kill @e[tag=anger_source]")
            step(3)
            middle = sample()
            step(duration + 1)
            final = sample()
            samples = [initial, active, middle, final]
            assert all(row["actor"] in (actor_id, None) for row in samples), (species, condition, samples)
            result[f"{species}_{condition}"] = samples
            print(species, condition, result[f"{species}_{condition}"], flush=True)
    return result


def compare(expected, actual):
    def summarize(rows):
        return [(row["actor"] is not None, row["end"] == -1) for row in rows]
    for name, gold in expected.items():
        rows = actual[name]
        assert summarize(gold[1:]) == summarize(rows[1:]), (name, gold, rows)
        if name.endswith(("early_expiry", "_legacy")):
            assert all(abs((g["end"]-g["time"]) - (r["end"]-r["time"])) <= 2
                       for g, r in zip(gold[1:3], rows[1:3], strict=True)), (name, gold, rows)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--java-home", type=Path)
    parser.add_argument("--reference", type=Path)
    args = parser.parse_args()
    controller.capture = capture
    if args.java_home:
        controller.capture_java(args.root, args.java_home, args.output)
    else:
        result = controller.capture_horizon(args.root)
        args.output.write_text(json.dumps(result, indent=2) + "\n")
        if args.reference:
            compare(json.loads(args.reference.read_text()), result)
