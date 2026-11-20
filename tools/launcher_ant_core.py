"""Replay actual Queen placement, owned hill entry and worker food in headless Horizon."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-ant-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "ant.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def data(path):
            return next((row.split(" data: ", 1)[1] for row in cmd("data get " + path)
                         if " data: " in row), None)

        def block(x, y, z, properties=""):
            return any("Test passed" in row for row in
                       cmd(f"execute if block {x} {y} {z} naturalist:ant_hill{properties}"))

        def give(item):
            cmd("item replace entity @s weapon.mainhand with " + item)
            ctl("wait", 4)

        def click_block(x, y, z):
            ctl("look-at-block", x, y, z, "up")
            ctl("mouse", "right", "click")
            ctl("wait", 6)

        def wait_until(check, ticks=600):
            for _ in range((ticks + 19) // 20):
                if result := check():
                    return result
                ctl("wait", 20)
            return None

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ant Hill Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 10.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 10.5 100 6.5")
            cmd("gamemode survival @s")
            owner = data("entity @s UUID")
            assert owner and owner.startswith("[I;"), owner
            cmd("setblock 8 101 8 minecraft:stone")
            give("naturalist:queen_ant")
            click_block(8, 99, 8)
            blocked = data("entity @s SelectedItem.id") == '"naturalist:queen_ant"'
            cmd("setblock 8 101 8 minecraft:air")
            click_block(8, 99, 8)
            consumed = data("entity @s SelectedItem.id") is None
            positions = [(x, y, z) for x in range(7, 10) for y in range(100, 103)
                         for z in range(7, 10) if block(x, y, z)]
            entrances = [pos for pos in positions if block(*pos, "[open=true]")]
            queen = [pos for pos in positions if block(*pos, "[has_queen=true,open=true,workers=0]")]
            links = [data("block " + " ".join(map(str, position)) + " Owner") for position in positions]
            linked = links.count(owner)
            if linked != len(positions):
                print(json.dumps({"owner": owner, "links": links, "block": cmd("data get block 8 100 8")}), flush=True)
            main = {"blocked": blocked, "consumed": consumed, "blocks": len(positions),
                    "open_in_range": 4 <= len(entrances) <= 6, "queens": len(queen), "owner_links": linked}
            print(json.dumps({"queen": main}, indent=2), flush=True)

            # A separate hill isolates correct owner checks and autonomous food storage.
            cmd("setblock 3 100 3 naturalist:ant_hill[open=true,workers=0,has_queen=false]")
            cmd(f"data merge block 3 100 3 {{Owner:{owner}}}")
            ant = "@e[tag=keeper,limit=1]"
            args = f"{{Owner:{owner},Tags:[keeper],Health:3f,HillCooldown:1000}}"
            assert any("Summoned" in row for row in cmd(f"summon naturalist:ant 4.5 100 3.5 {args}"))
            # Headless command inspection is used only for observations; sugar is a real item use.
            give("minecraft:sugar 2")
            cmd("tp @s 5.1 100 4.6")
            ctl("wait", 4)
            nearby = ctl("entities", 6)["entities"]
            worker = min((entity for entity in nearby if entity["type"] == "naturalist:ant"),
                         key=lambda entity: (entity["pos"][0] - 4.5) ** 2)
            print(json.dumps({"sugar_before": data(f"entity {ant} Health"), "nearby_worker": worker}), flush=True)
            ctl("look-at-entity", worker["id"])
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            after_health, after_sugar = data(f"entity {ant} Health"), data("entity @s SelectedItem.count")
            print(json.dumps({"sugar_after": after_sugar, "health_after": after_health}), flush=True)
            heals = after_health in {"4.0f", "4f"} and after_sugar == "1"
            cmd("tp @s 12.5 100 12.5")
            # The pinned Java goal only searches on multiples of ten, and
            # vanilla's full goal-selection tick alternates by entity ID.
            if worker["id"] & 1:
                cmd(f"kill {ant}")
                ant = "@e[tag=gatherer,limit=1]"
                gatherer = args.replace("Tags:[keeper]", "Tags:[gatherer]")
                assert any("Summoned" in row for row in cmd(f"summon naturalist:ant 4.5 100 3.5 {gatherer}"))
            position = data(f"entity {ant} Pos")
            numbers = [float(token) for token in re.findall(r"[-+]?\d+(?:\.\d+)?", position or "")]
            assert len(numbers) == 3, position
            x, y, z = numbers
            print(json.dumps({"drop": cmd(f'summon minecraft:item {x + 0.2} {y} {z} {{Item:{{id:"minecraft:apple",count:2}},PickupDelay:0s,NoGravity:1b,Tags:[ant-food]}}')}), flush=True)
            print(json.dumps({"worker_food_source": data("entity @e[tag=ant-food,limit=1]")}), flush=True)
            carried = wait_until(lambda: data(f"entity {ant} CarriedFood"), 320)
            stored = wait_until(lambda: data("block 3 100 3 Storage[0].id") == '"minecraft:apple"', 800)
            data_result = {"sugar_heals": heals, "carried_food_uuid": carried, "stored": bool(stored)}
            print(json.dumps({"worker": data_result}, indent=2), flush=True)
            return main, data_result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    queen, worker = capture(args.version)
    assert all(value == expected[key] for key, value in queen.items()), queen
    assert worker["sugar_heals"] == expected["sugar_heals"] and worker["stored"] == expected["stored"], worker
