"""Compare brewed Antivenom through its original application routes."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


POTION = 'minecraft:potion_contents={potion:"naturalist:antivenom"}'
FORMS = ("potion", "splash_potion", "lingering_potion")


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-antivenom-", ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        socket = home / "antivenom.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75)
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        pig = "@e[type=minecraft:pig,tag=antivenom_fixture,limit=1]"

        def poison(who):
            cmd(f"effect give {who} minecraft:poison 30 0 true")
            cmd(f"effect give {who} minecraft:regeneration 30 0 true")
            assert has_effect(who, "poison"), who

        def has_effect(who, name):
            data = cmd(f"data get entity {who} active_effects")
            return any(f'"minecraft:{name}"' in row for row in data)

        def throw(form):
            cmd(f"item replace entity @s weapon.mainhand with minecraft:{form}[{POTION}]")
            ctl("look-at", 8.5, 101.2, 8.5)
            ctl("mouse", "right", "click")

        def brew():
            at = "3 101 3"
            cmd(f"setblock {at} minecraft:brewing_stand")
            items = [f'{{Slot:{slot}b,id:"minecraft:{item}",count:1,components:'
                     '{"minecraft:potion_contents":{potion:"minecraft:awkward"}}}'
                     for slot, item in enumerate(FORMS)]
            items += ['{Slot:3b,id:"naturalist:scorpion_poison_gland",count:1}',
                      '{Slot:4b,id:"minecraft:blaze_powder",count:1}']
            cmd(f'data merge block {at} {{Items:[{",".join(items)}],Fuel:1}}')
            ctl("wait", 5)
            cmd(f"data merge block {at} {{BrewTime:3}}")
            ctl("wait", 15)
            nbt = str(cmd(f"data get block {at} Items"))
            assert nbt.count('"naturalist:antivenom"') == len(FORMS), nbt
            assert all(f'"minecraft:{item}"' in nbt for item in FORMS), nbt

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Antivenom Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 106 15 minecraft:air")
            cmd("tp @s 8.5 101 6.0")
            ctl("wait", 8)
            brew()
            cmd("gamemode survival @s")
            cmd("summon minecraft:pig 8.5 101 8.5 {NoAI:1b,Tags:[antivenom_fixture]}")
            poison("@s")
            cmd(f"item replace entity @s weapon.mainhand with minecraft:potion[{POTION}]")
            ctl("mouse", "right", "down")
            ctl("wait", 37)
            ctl("mouse", "right", "up")
            result = {
                "drink_removes_poison": not has_effect("@s", "poison"),
                "drink_leaves_regeneration": has_effect("@s", "regeneration"),
            }

            poison(pig)
            throw("splash_potion")
            ctl("wait", 15)
            result.update(splash_removes_poison=not has_effect(pig, "poison"),
                          splash_leaves_regeneration=has_effect(pig, "regeneration"))

            poison(pig)
            throw("lingering_potion")
            ctl("wait", 4)
            result["lingering_waits_before_removing_poison"] = has_effect(pig, "poison")
            ctl("wait", 25)
            result.update(lingering_removes_poison=not has_effect(pig, "poison"),
                          lingering_leaves_regeneration=has_effect(pig, "regeneration"))
            cmd("kill @e[type=minecraft:area_effect_cloud]")

            poison(pig)
            cmd(f"effect give {pig} naturalist:antivenom 1 0 true")
            ctl("wait", 4)
            result["direct_effect_removes_poison"] = not has_effect(pig, "poison")

            cmd(f"effect give {pig} minecraft:resistance 30 3 true")
            cmd("item replace entity @s weapon.mainhand with minecraft:bow")
            cmd(f"item replace entity @s weapon.offhand with minecraft:tipped_arrow[{POTION}]")
            ctl("look-at", 8.5, 101.45, 8.5)
            ctl("mouse", "right", "down")
            ctl("wait", 22)
            ctl("mouse", "right", "up")
            ctl("wait", 4)
            nbt = str(cmd(f"data get entity {pig} HurtTime"))
            assert re.search(r"[1-9]\d*s?", nbt.split("entity data:")[-1]), nbt
            result["arrow_removes_poison"] = not has_effect(pig, "poison")
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    output = capture(args.version)
    print(json.dumps(output, indent=2), flush=True)
    mismatch = {key: (observed, expected[key]) for key, observed in output.items() if expected[key] != observed}
    assert not mismatch, mismatch
