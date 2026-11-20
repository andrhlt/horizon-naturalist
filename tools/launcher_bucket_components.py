"""Exercise the fish buckets and Duck Egg stack limit on the real headless client."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


BUCKETS = (
    "anglerfish", "bass", "blobfish", "catfish", "giant_isopod",
    "jellyfish", "piranha", "ray", "starfish",
)


def replay(version):
    with tempfile.TemporaryDirectory(prefix="nat-item-defaults-", ignore_cleanup_errors=True) as folder:
        home = Path(folder)
        socket = home / "test.sock"

        def ctl(*args):
            call = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  capture_output=True, text=True, check=True, timeout=65)
            return json.loads(call.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def food():
            player = ctl("state")["player"]
            return player["food"], player["saturation"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist Item Defaults", "--creative", "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:natural_regeneration false")
            command("gamemode survival @s")
            command("effect give @s minecraft:hunger 30 100 true")
            for _ in range(16):
                ctl("wait", 10)
                if food()[0] <= 5:
                    break
            command("effect clear @s minecraft:hunger")
            assert food()[0] <= 8, food()
            ctl("look", 0, -75)
            result = {}
            for kind in BUCKETS:
                name = f"naturalist:{kind}_bucket"
                command(f"item replace entity @s weapon.mainhand with {name}")
                before = food()
                ctl("mouse", "right", "down")
                ctl("wait", 35)
                ctl("mouse", "right", "up")
                state = ctl("state")
                result[name] = {"nutrition_delta": state["player"]["food"] - before[0],
                                "retained": (state["held"] or {}).get("item") == name}
            command("clear @s")
            command("give @s naturalist:duck_egg 64")
            inventory = " ".join(command("data get entity @s Inventory"))
            stacks = [stack for stack in re.findall(r"\{[^{}]*\}", inventory)
                      if 'id: "naturalist:duck_egg"' in stack]
            result["duck_egg_stacks"] = sorted(int(re.search(r"count: (\d+)", stack)[1])
                                               for stack in stacks)
            assert sum(result["duck_egg_stacks"]) == 64, inventory
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--baseline", action="store_true")
    args = parser.parse_args()
    result = replay(args.version)
    print(json.dumps(result, indent=2), flush=True)
    if args.baseline:
        assert result["duck_egg_stacks"] == [16] * 4, result
    else:
        assert result["duck_egg_stacks"] == [64], result
        assert all(result[f"naturalist:{kind}_bucket"] == {"nutrition_delta": 0, "retained": True}
                   for kind in BUCKETS), result
