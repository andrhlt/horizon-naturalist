"""Compare real Horizon net pickup and ground release against the Java Hedgehog."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-hedgehog-net-", ignore_cleanup_errors=True) as temporary:
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

        def net_click():
            entities = ctl("entities", 6, "naturalist:hedgehog")["entities"]
            assert len(entities) == 1, entities
            looked = ctl("look-at-entity", entities[0]["id"])
            assert looked["target"]["kind"] == "entity", looked
            ctl("mouse", "right", "click")
            ctl("wait", 6)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Hedgehog Capture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 108 15 minecraft:air")
            cmd("tp @s 8.5 100 6.5")
            cmd("gamemode survival @s")
            cmd("clear @s")
            actor = field("@s", "UUID")
            targeted = "@e[tag=net_target,limit=1]"
            spawned = cmd("summon naturalist:hedgehog 8.5 100 8.5 {Tags:[net_target],NoAI:1b}")
            assert any("Summoned" in row for row in spawned), spawned
            cmd("item replace entity @s weapon.mainhand with naturalist:capture_net")
            ctl("wait", 6)
            net_click()
            durability = 'SelectedItem.components."minecraft:damage"'
            rejected_wild = field(targeted, "UUID") is not None and field("@s", durability) in (None, "0")
            changes = f"{{Owner:{actor},DyeColor:-1,FollowingOwner:0b,Sitting:0b,Health:7f}}"
            assert any("Modified" in row for row in cmd(f"data merge entity {targeted} {changes}"))
            net_click()
            removed = field(targeted, "UUID") is None and field("@s", durability) == "1"
            caught_owner = field("@s", 'Inventory[{id:"naturalist:hedgehog"}].components."minecraft:custom_data".Owner')
            ctl("select", "naturalist:hedgehog")
            ctl("look-at-block", 6, 99, 8)
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            released = "@e[type=naturalist:hedgehog,limit=1]"
            state = {
                key: field(released, key) for key in ("Owner", "DyeColor", "Health", "FollowingOwner", "FromHand", "NoGravity", "Pos")
            }
            position = [float(part) for part in re.findall(r"-?\d+(?:\.\d+)?", state["Pos"] or "")]
            print(json.dumps({"caught_owner": caught_owner, "recovered": state}), flush=True)
            return {
                "wild_net_rejected": rejected_wild,
                "tame_net_captured": removed,
                "net_owns_caught_item": caught_owner == actor,
                "ground_release_fields": state["Owner"] == actor and state["Health"] == "7.0f"
                    and state["DyeColor"] == "-1" and state["FollowingOwner"] == "0b"
                    and state["FromHand"] == "1b" and state["NoGravity"] in (None, "0b"),
                "release_center_on_ground": position == [6.5, 100.0, 8.5],
            }
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    actual = capture(args.version)
    print(json.dumps(actual, indent=2), flush=True)
    expected = json.loads(args.fixture.read_text())
    assert all(value == expected[key] for key, value in actual.items()), (actual, expected)
