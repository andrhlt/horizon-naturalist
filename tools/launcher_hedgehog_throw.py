"""Compare a real Horizon Hedgehog throw, enchanted hit and loyalty return with Java."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-hedgehog-throw-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "hedgehog.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def field(entity, key):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in cmd(f"data get entity {entity} {key}")
                         if marker in row), None)

        def nums(text):
            return [float(number) for number in re.findall(r"-?\d+(?:\.\d+)?", text or "")]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Hedgehog Air Throw", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 108 15 minecraft:air")
            cmd("tp @s 8.5 100 6.5 0 0")
            cmd("gamemode survival @s")
            cmd("clear @s")
            actor = field("@s", "UUID")
            cmd("tick freeze")
            target = "@e[tag=throw-target,limit=1]"
            cmd("summon minecraft:pig 8.5 100.9 8.15 {Tags:[throw-target],NoAI:1b,NoGravity:1b}")
            data = f'{{Owner:{actor},Health:12f,Variant:"naturalist:brown",DyeColor:14,Age:0,FollowingOwner:1b,Tame:1b}}'
            cmd(f'item replace entity @s weapon.mainhand with naturalist:hedgehog[minecraft:custom_data={data}]')
            for name, level in (("thorns", 2), ("loyalty", 3), ("flame", 1)):
                result = cmd(f"enchant @s minecraft:{name} {level}")
                assert any("Applied" in row or "enchanted" in row.lower() for row in result), (name, result)
            invalid = cmd("enchant @s minecraft:sharpness 1")
            assert all("Applied" not in row for row in invalid), invalid
            ctl("wait", 5)
            start = field("@s", "SelectedItem")
            ctl("mouse", "right", "click")
            outcome = "@e[type=naturalist:hedgehog,limit=1]"
            ctl("wait", 5)
            launch = nums(field(outcome, "Pos"))
            motion = nums(field(outcome, "Motion"))
            consumed = field("@s", "SelectedItem.id") is None
            from_hand = field(outcome, "FromHand") == "1b"
            print(json.dumps({"before": start, "launch": launch, "motion": motion,
                              "consumed": consumed, "from_hand": from_hand}), flush=True)

            for _ in range(8):
                cmd("tick step 1")
                ctl("wait", 3)
                pig_health = nums(field(target, "Health"))
                if pig_health and pig_health[0] < 10.0:
                    break
            pig_fire = nums(field(target, "Fire"))
            cmd("tick step 11")
            ctl("wait", 13)
            returned = field("@s", 'Inventory[{id:"naturalist:hedgehog"}]')
            print(json.dumps({"pig_health": pig_health, "pig_fire": pig_fire,
                              "inventory": returned, "pet": field(outcome, "Health")}), flush=True)
            return {
                "launch": launch, "motion": motion, "consumed": consumed, "roll": from_hand,
                "pig_health": pig_health[0] if pig_health else None,
                "pig_fire": pig_fire[0] if pig_fire else None, "return": bool(returned),
                "returned_health": nums(field("@s", 'Inventory[{id:"naturalist:hedgehog"}].components."minecraft:custom_data".Health')),
                "returned_owner": field("@s", 'Inventory[{id:"naturalist:hedgehog"}].components."minecraft:custom_data".Owner') == actor,
                "enchants": bool(returned and all(f"minecraft:{key}" in returned for key in ("thorns", "loyalty", "flame"))),
            }
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    observed = capture(args.version)
    print(json.dumps(observed, indent=2))
    assert len(observed["launch"]) == len(observed["motion"]) == 3, observed
    expected_launch = expected["launch"].copy()
    expected_launch[1] += 7
    assert all(abs(world - java) < 0.001 for world, java in zip(observed.pop("launch"), expected_launch))
    assert all(abs(world - java) < 0.001 for world, java in zip(observed.pop("motion"), expected["motion"]))
    # The Java probe pauses the victim on impact. Horizon observes the live
    # victim at the command boundary, which can include its next base tick.
    assert expected["pig_fire"] - 1 <= observed.pop("pig_fire") <= expected["pig_fire"]
    assert observed.pop("returned_health") == [expected["returned_health"]]
    assert all(value == expected[key] for key, value in observed.items()), observed
