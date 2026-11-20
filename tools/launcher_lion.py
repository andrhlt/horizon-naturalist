"""Compare the Lion’s mane, cub speed, pride selection and predation headlessly."""
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile

native = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-lion-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run([native / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(text):
        return ctl("command", text)["chat"]

    def step(n):
        cmd(f"tick step {n}")
        ctl("wait", n + 2)

    def data(target, key):
        marker = " following entity data: "
        return next((row.split(marker, 1)[1] for row in cmd(f"data get entity {target} {key}") if marker in row), None)

    def stat(target, key):
        for row in cmd(f"attribute {target} minecraft:{key} base get"):
            match = re.search(r" is ([0-9.eE+-]+)", row)
            if match:
                return float(match[1])
        raise AssertionError((target, key))

    def inspect(query):
        marker = "Lion fixture "
        for row in cmd(f"naturalist_fixture_lion {query}"):
            if marker in row:
                return json.loads(row.split(marker, 1)[1])
        raise AssertionError(query)

    def spawn(x, z, tag, nbt=""):
        extra = f",{nbt}" if nbt else ""
        result = cmd(f"summon naturalist:lion {x} 92 {z} {{Tags:[{tag}]{extra}}}")
        assert any("Summoned" in row for row in result), result

    try:
        ctl("launch", "--headless", "--game", native / "horizon-client", "--resources",
            native / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist Lion", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("gamerule minecraft:advance_time false")
        cmd("tp @s 7.5 92 3.5")
        ctl("wait", 24)
        cmd("tick freeze")
        cmd("fill 2 91 2 35 91 15 minecraft:stone")
        cmd("fill 2 92 2 35 96 15 minecraft:air")
        cmd("tp @s 7.5 92 3.5")
        cmd("gamemode spectator @s")
        step(4)
        adult = "@e[type=naturalist:lion,tag=adult,limit=1]"
        spawn(6.5, 7.5, "adult", "NoAI:1b,Mane:1b,UUID:[I;0,0,0,4]")
        spawn(4.5, 8.5, "cub", "Age:-12")
        base, cub = inspect("adult"), inspect("cub")
        output = {
            "adult_dims": base["dims"], "baby_dims": cub["dims"],
            "ambient_interval": base["ambient_interval"], "food_ids": sorted(base["food_ids"]),
            "default_variant": data(adult, "Variant").strip('"'),
            "attributes": [stat(adult, field) for field in
                           ("max_health","movement_speed","attack_damage","follow_range","step_height","knockback_resistance")],
        }
        assert data(adult, "Mane") == "1b" and base["mane"], base
        if "pride_level_cub_rolls" in reference:
            output["pride_member_rolls"] = [
                [inspect(f"roll {seed*100+index} {index} {int(cub)}") for index, cub in enumerate(cases)]
                for seed, cases in enumerate(reference["pride_level_cub_rolls"], 1)
            ]
        step(1)
        boosted = inspect("cub")["movement"]
        step(20)
        cmd(f"data merge entity {adult} {{Mane:0b}}")
        # Read the same former cub at the clear x coordinate once it grows.
        cmd("kill " + adult)
        step(25)
        grown = inspect("adult")["movement"]
        output["cub_speed"] = [cub["movement"], boosted, grown]
        cmd("kill @e[type=naturalist:lion]")
        step(25)

        spawn(6.5, 7.5, "adult", "NoAI:1b,Mane:1b,UUID:[I;0,0,0,4]")
        spawn(18.5, 7.5, "lead1", "NoAI:1b,UUID:[I;0,0,0,1]")
        spawn(20.5, 7.5, "lead2", "NoAI:1b,UUID:[I;0,0,0,2]")
        step(3)
        first = inspect("follower")["leader"]
        cmd("effect give @e[tag=lead1,limit=1] minecraft:invisibility infinite 0 true")
        step(3)
        second = inspect("follower")["leader"]
        cmd("data merge entity @e[tag=lead2,limit=1] {Mane:1b}")
        step(3)
        output["leader_choices"] = [first, second, inspect("follower")["leader"]]
        cmd("data merge entity @e[tag=lead2,limit=1] {Mane:0b}")
        cmd(f"data merge entity {adult} {{NoAI:0b}}")
        cmd("time set 1000")
        followed = False
        for _ in range(45):
            step(4)
            state = inspect("follower")
            followed |= state["running_pride"] and state["pos"][0] > 7.3
            if followed:
                break
        assert followed, state
        print(json.dumps(output), flush=True)
        mismatch = {key: [reference[key], actual] for key, actual in output.items()
                    if actual != reference[key] and key != "cub_speed"}
        if any(abs(a-b) > 1e-7 for a,b in zip(output["cub_speed"],reference["cub_speed"])):
            mismatch["cub_speed"] = [reference["cub_speed"], output["cub_speed"]]
        assert not mismatch, mismatch
        print(f"Lion matched all {len(output)} Java groups and physically followed its selected pride leader.")
    finally:
        if socket.exists():
            ctl("quit")
