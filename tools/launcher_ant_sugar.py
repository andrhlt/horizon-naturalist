"""Verify ant sugar healing and empty-hand routing in headless Horizon."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time


def capture(version, jar):
    with tempfile.TemporaryDirectory(prefix="nat-ant-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        import hashlib
        import shutil
        from prepare import JAR_SHA
        if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
            raise ValueError("expected the pinned Naturalist jar")
        (root / "jars").mkdir()
        shutil.copyfile(jar, root / "jars" / f"{JAR_SHA}.jar")
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
            ctl("wait", 4)
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            after_health, after_sugar = data(f"entity {ant} Health"), data("entity @s SelectedItem.count")
            print(json.dumps({"sugar_after": after_sugar, "health_after": after_health}), flush=True)
            heals = after_health in {"4.0f", "4f"} and after_sugar == "1"
            assert heals, (after_health, after_sugar)
            cmd(f"data merge entity {ant} {{Health:6f,NoAI:1b}}")
            give("minecraft:sugar 2")
            ctl("look-at-entity", worker["id"])
            ctl("wait", 4)
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            full = (data(f"entity {ant} Health"), data("entity @s SelectedItem.count"))
            assert full == ("6.0f", "2"), full
            cmd(f"data merge entity {ant} {{Health:3f}}")
            cmd("gamemode creative @s")
            ctl("look-at-entity", worker["id"])
            ctl("wait", 4)
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            creative = (data(f"entity {ant} Health"), data("entity @s SelectedItem.count"))
            assert creative == ("4.0f", "2"), creative
            cmd("gamemode survival @s")
            cmd(f"kill {ant}")
            ctl("wait", 20)
            ant = "@e[tag=offtest,limit=1]"
            cmd(f"summon naturalist:ant 4.5 100 3.5 {{Health:3f,NoAI:1b,Owner:{owner},Tags:[offtest]}}")
            ctl("wait", 5)
            worker = next(entity for entity in ctl("entities", 6)["entities"] if entity["type"] == "naturalist:ant")
            cmd("item replace entity @s weapon.mainhand with minecraft:air")
            cmd("item replace entity @s weapon.offhand with minecraft:sugar 2")
            ctl("wait", 10)
            assert ctl("status")["offhand"]["count"] == 2
            ctl("look-at-entity", worker["id"])
            ctl("wait", 4)
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            print("offhand-after", ctl("status")["offhand"], flush=True)
            offhand = (data(f"entity {ant} Health"), str((ctl("status")["offhand"] or {}).get("count")))
            assert offhand == ("4.0f", "1"), offhand
            print("maximum-health probe", cmd(f"attribute {ant} minecraft:max_health base get"), flush=True)
            cmd("item replace entity @s weapon.offhand with minecraft:air")
            ctl("wait", 10)
            before_sit = data(f"entity {ant} Sitting")
            ctl("look-at-entity", worker["id"])
            ctl("wait", 4)
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            after_sit = data(f"entity {ant} Sitting")
            assert before_sit == after_sit == "0b", (before_sit, after_sit)
            print("full health, creative, offhand and no-sit passed", full, creative, offhand, before_sit, after_sit)
            cmd(f"kill {ant}")
            ctl("wait", 20)
            cmd(f"summon minecraft:wolf 4.5 100 3.5 {{Health:8f,NoAI:1b,Owner:{owner},Tags:[wolfcontrol]}}")
            ctl("wait", 10)
            wolf = next(entity for entity in ctl("entities", 6)["entities"] if entity["type"] == "minecraft:wolf")
            assert data("entity @e[tag=wolfcontrol,limit=1] Sitting") == "0b"
            ctl("look-at-entity", wolf["id"])
            ctl("wait", 4)
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            assert data("entity @e[tag=wolfcontrol,limit=1] Sitting") == "1b"
            print("vanilla owned wolf sit passed", flush=True)
            return {"sugar_heals": heals}
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("jar", type=Path)
    args = parser.parse_args()
    capture(args.version, args.jar)
