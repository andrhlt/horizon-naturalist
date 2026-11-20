"""Drop an actual headless survival player onto Snails and compare Java outcomes."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time


def capture(version, enabled):
    with tempfile.TemporaryDirectory(prefix="nat-snail-crush-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "crush.sock"
        config = directory / "config" / "naturalist-server.toml"
        config.parent.mkdir()
        config.write_text(f"[behavior]\nsnail_crushing = {str(enabled).lower()}\n")

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", directory,
                "--game-dir", directory, "--world", "Snail Crush Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 103 10.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("gamemode survival @s")
            cmd("tick rate 5")
            results, impacts = {}, {}
            cases = [
                ("airborne", 2.5, "", False, False),
                ("feather", 4.5, "", True, False),
                ("grounded", 6.5, "", False, True),
                ("named", 8.5, ',CustomName:{text:"Protected"}', False, False),
                ("persistent", 10.5, ",PersistenceRequired:1b", False, False),
                ("from_hand", 12.5, ",FromHand:1b", False, False),
            ]
            for label, x, extra, boots, standing in cases:
                cmd("tp @s 8.5 100 12.5")
                cmd("item replace entity @s armor.feet with " +
                    ("minecraft:iron_boots[minecraft:enchantments={\"minecraft:feather_falling\":1}]" if boots else "minecraft:air"))
                response = cmd(f"summon naturalist:snail {x} 100 8.5 {{NoAI:1b,OnGround:1b,Tags:[crush_{label}]{extra}}}")
                assert any("Summoned" in row for row in response), response
                ctl("wait", 4)
                # Grounded control stands beside the Snail with overlapping boxes.
                cmd(f"tp @s {x + 0.4 if standing else x} {100 if standing else 101.4} 8.5")
                deadline = time.monotonic() + 10
                samples = []
                while time.monotonic() < deadline:
                    player = ctl("status")["player"]
                    point = [round(player["position"][1], 5), player["on_ground"]]
                    if not samples or samples[-1] != point:
                        samples.append(point)
                    if player["on_ground"]:
                        break
                else:
                    raise AssertionError(samples)
                if not standing:
                    assert any(100.0 < y < 101.2 and not grounded for y, grounded in samples), samples
                ctl("wait", 4)
                rows = cmd(f"data get entity @e[tag=crush_{label},limit=1] Health")
                matches = [m for row in rows if (m := re.search(r":\s*([-\d.]+)f\s*$", row))]
                results[f"{label}_health"] = float(matches[-1][1]) if matches else 0.0
                impacts[label] = samples
            return results, impacts
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--enabled", action="store_true")
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    actual, impacts = capture(args.version, args.enabled)
    print(json.dumps({"result": actual, "impact_samples": impacts}, indent=2), flush=True)
    assert actual == expected, (actual, expected)
