import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-bass-hunt-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=45)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Bass Hunt Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("tp @s 8.5 106 5.5")
        ctl("wait", 20)
        cmd("fill 3 99 3 13 106 14 minecraft:stone")
        cmd("fill 4 100 4 12 105 13 minecraft:water")
        cmd("fill 3 106 3 13 108 14 minecraft:air")
        cmd("tp @s 8.5 106 5.5")
        cmd("tick freeze")
        for line in [
            'summon naturalist:bass 8.5 102.5 8.5 {Variant:"naturalist:bass_large",PersistenceRequired:1b,Tags:[predator]}',
            'summon naturalist:bass 8.8 102.5 8.5 {Variant:"naturalist:bass",NoAI:1b,PersistenceRequired:1b,Tags:[prey]}',
        ]:
            rows = cmd(line)
            assert any("Summoned" in r for r in rows), rows
        cmd("tick step 160")
        ctl("wait", 165)
        after = cmd("data get entity @e[tag=predator,limit=1] EatCooldown")
        prey = cmd("data get entity @e[tag=prey,limit=1] Variant")
        drops = ctl("entities", 40, "minecraft:item")["entities"]
        output = {"cooldown": next((int(text.split("following entity data: ")[1]) for text in after
                                    if "following entity data: " in text), None),
                  "discards_prey": not any("following entity data" in row for row in prey),
                  "bone_meal": any(drop["item"]["item"] == "minecraft:bone_meal" for drop in drops)}
        print(json.dumps(output), flush=True)
        assert output["discards_prey"] == reference["discards_prey"], output
        assert 0 < output["cooldown"] <= 5000 and output["bone_meal"], output
        ctl("quit")
        deadline = time.monotonic() + 6
        while socket.exists() and time.monotonic() < deadline:
            time.sleep(0.1)
        assert not socket.exists()
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Bass Hunt Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        rows = cmd("data get entity @e[tag=predator,limit=1] EatCooldown")
        saved = next((int(row.split("following entity data: ", 1)[1])
                      for row in rows if "following entity data: " in row), None)
        print(json.dumps({"reopened_cooldown": saved}), flush=True)
        assert saved is not None and 0 < saved <= output["cooldown"], (saved, output)
    finally:
        if socket.exists():
            ctl("quit")
