"""Replay basic scorpion behavior against the pinned Java result."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, *, targeting=False):
    with tempfile.TemporaryDirectory(prefix="nat-scorpion-", ignore_cleanup_errors=True) as path:
        home = Path(path)
        socket = home / "scorpion.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75)
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            response = ctl("command", line)
            if not response["ok"]:
                raise RuntimeError((line, response))
            return response["chat"]

        def read(entity, key):
            return next((line.split("following entity data: ", 1)[1]
                         for line in cmd(f"data get entity {entity} {key}")
                         if "following entity data: " in line), None)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Scorpion Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 105 15 minecraft:air")
            cmd("tp @s 8.5 101 5.5")
            ctl("wait", 10)
            cmd("gamemode survival @s")
            cmd("effect give @s minecraft:resistance infinite 1 true")
            cmd("time set noon")
            if targeting:
                return replay_targeting(ctl, cmd, read)
            output = {}
            for kind in ("desert_scorpion", "jungle_scorpion"):
                selector = f"@e[type=naturalist:{kind},tag=scorpion,limit=1]"
                rows = cmd(f"summon naturalist:{kind} 8.5 101 7.5 {{Health:3f,NoAI:1b,Tags:[scorpion]}}")
                assert any("Summoned" in row for row in rows), rows
                result = {"variant": json.loads(read(selector, "Variant") or "null")}
                for name in ("max_health", "movement_speed", "attack_damage"):
                    rows = cmd(f"attribute {selector} minecraft:{name} base get")
                    result[name] = next((float(match[1]) for row in rows
                                         if (match := re.search(r" is ([0-9.eE+-]+)", row))), None)
                cmd("item replace entity @s weapon.mainhand with naturalist:lizard_tail 2")
                ctl("look-at", 8.5, 101.2, 7.5)
                ctl("mouse", "right", "click")
                ctl("wait", 4)
                result["healed"] = float((read(selector, "Health") or "0").rstrip("fd"))
                stack = read("@s", "SelectedItem") or ""
                result["remaining_tail"] = int(match[1]) if (match := re.search(r"count:\s*(\d+)", stack)) else 0

                cmd("clear @s")
                cmd(f"data merge entity {selector} {{NoAI:0b,Health:{result['max_health']}f}}")
                cmd("time set midnight")
                initial_health = ctl("state")["player"]["health"]
                for _ in range(28):
                    ctl("wait", 4)
                    player = ctl("state")["player"]
                    effect = next((effect for effect in player["effects"] if effect["id"] == "poison"), None)
                    if effect is not None:
                        break
                result["attack_landed"] = effect is not None and player["health"] < initial_health
                result["poison_duration"] = effect["duration_ticks"] if effect else 0
                result["poison_amplifier"] = effect["amplifier"] if effect else None
                cmd(f"kill {selector}")
                cmd("effect clear @s minecraft:poison")
                cmd("effect give @s minecraft:instant_health 1 4 true")
                cmd("tp @s 8.5 101 5.5")
                cmd("time set noon")
                output[kind] = result
            return output
        finally:
            if socket.exists():
                ctl("quit")


def replay_targeting(ctl, cmd, read):
    cmd("gamerule minecraft:advance_time false")
    cmd("gamerule minecraft:universal_anger false")
    cmd("difficulty normal")
    cmd("effect clear @s")
    cmd("effect give @s minecraft:resistance infinite 3 true")

    def player_health():
        return ctl("state")["player"]["health"]

    def poisoned():
        return any(effect["id"] == "poison" for effect in ctl("state")["player"]["effects"])

    def reset():
        cmd("kill @e[tag=targeting_scorpion]")
        cmd("kill @e[tag=targeting_zombie]")
        cmd("effect clear @s minecraft:poison")
        cmd("effect give @s minecraft:instant_health 1 4 true")
        cmd("tp @s 8.5 101 5.5")
        ctl("wait", 7)

    def summon(kind, nbt=""):
        target = f"@e[type=naturalist:{kind},tag=targeting_scorpion,limit=1]"
        rows = cmd(f"summon naturalist:{kind} 8.5 101 7.4 {{Tags:[targeting_scorpion],PersistenceRequired:1b{nbt}}}")
        assert any("Summoned" in row for row in rows), rows
        return target

    outputs = {}
    for kind in ("desert_scorpion", "jungle_scorpion"):
        reset()
        cmd("time set noon")
        initial = player_health()
        summon(kind)
        ctl("wait", 65)
        sample = {"sunlit_attacks_player": player_health() < initial or poisoned()}
        reset()
        cmd("difficulty peaceful")
        cmd("time set midnight")
        initial = player_health()
        peaceful = summon(kind)
        ctl("wait", 65)
        sample["peaceful_attacks_player"] = player_health() < initial or poisoned()
        sample["peaceful_survives"] = read(peaceful, "Health") is not None
        reset()
        cmd("difficulty normal")
        outputs[kind] = sample

    cmd("time set midnight")
    initial = player_health()
    from_hand = summon("desert_scorpion", ",FromHand:1b")
    ctl("wait", 80)
    result = {"from_hand_hunts_player": player_health() < initial or poisoned()}
    cmd(f"tp {from_hand} 8.5 101 7.0")
    damage = cmd(f"damage {from_hand} 1 minecraft:player_attack by @s")
    assert any("Applied 1.0 damage" in line for line in damage), damage
    ctl("wait", 75)
    result["from_hand_retaliates"] = player_health() < initial or poisoned()

    reset()
    cmd("gamemode creative @s")
    zombie = "@e[type=minecraft:zombie,tag=targeting_zombie,limit=1]"
    for tag, from_hand in (("from_hand", ",FromHand:1b"), ("wild", "")):
        reset()
        cmd("summon minecraft:zombie 8.5 101 9.15 {NoAI:1b,Tags:[targeting_zombie],PersistenceRequired:1b}")
        before = float(read(zombie, "Health").rstrip("Ff"))
        summon("desert_scorpion", from_hand)
        ctl("wait", 115)
        after = read(zombie, "Health")
        result[f"{tag}_hunts_zombie"] = after is None or float(after.rstrip("Ff")) < before
    outputs["desert_behavior"] = result
    return outputs


def compare(output, reference):
    for kind, result in output.items():
        expected = reference[kind]
        for key, actual in result.items():
            target = expected[key]
            if key == "poison_duration":
                assert 111 <= actual <= target, (kind, key, actual, target)
            elif type(target) is float:
                assert actual is not None and abs(actual - target) < 1e-6, (kind, key, actual, target)
            else:
                assert actual == target, (kind, key, actual, target)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--targeting", action="store_true")
    options = parser.parse_args()
    result = capture(options.version, targeting=options.targeting)
    print(json.dumps(result, indent=2), flush=True)
    compare(result, json.loads(options.fixture.read_text()))
