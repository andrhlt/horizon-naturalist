"""Replay the pinned Catfish's delayed strike through Horizon's live game tick."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-catfish-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def data(target, key):
        return next(row.split("following entity data: ", 1)[1]
                    for row in cmd(f"data get entity {target} {key}") if "following entity data: " in row)

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(sys.argv[2]).parent.parent / "catfish/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Catfish Predation {home.name}", "--creative", "--arg=--control-socket",
                    f"--arg={socket}", "--arg=--headless", "--arg=--test-terrain", "--arg=--render-distance", "--arg=4"],
                    stdout=handle, stderr=subprocess.STDOUT)
            for _ in range(50):
                if child.poll() is not None:
                    raise RuntimeError(f"Horizon Play exited with code {child.returncode}")
                if socket.exists() and ctl("state")["screen"] == "playing":
                    break
                time.sleep(1)
            else:
                raise RuntimeError("Timed out waiting for Horizon Play to load the fixture world")
        else:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Catfish Predation", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("tp @s 7.5 101 6.5")
        ctl("wait", 20)
        cmd("fill 2 100 2 15 100 15 minecraft:stone")
        cmd("fill 2 101 2 15 107 15 minecraft:air")
        cmd("tick freeze")
        cmd("fill 4 101 8 12 104 14 minecraft:water")
        cmd('summon naturalist:catfish 7.5 102.25 8.5 {NoAI:1b,PersistenceRequired:1b}')
        cmd('summon naturalist:bass 7.8 102.25 8.5 {NoAI:1b,PersistenceRequired:1b}')
        catfish = "@e[type=naturalist:catfish,limit=1]"
        def has_bass():
            return any("following entity data:" in row for row in
                       cmd("data get entity @e[type=naturalist:bass,limit=1] Health"))
        output = {}
        for attribute in ("max_health", "movement_speed", "attack_damage"):
            rows = cmd(f"attribute {catfish} minecraft:{attribute} base get")
            output[attribute] = float(next(m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))))
        output["variant"] = data(catfish, "Variant").strip('"')
        output["bite_landed"] = any("Catfish strike true" in row for row in cmd("naturalist_fixture_catfish_strike"))
        output["prey_alive_after_bite"] = has_bass()
        output["cooldown_after_bite"] = int(data(catfish, "HuntingCooldown"))
        step(7)
        output["prey_alive_after_seven"] = has_bass()
        step(1)
        output["prey_gone_after_eight"] = not has_bass()
        output["bone_meal"] = any("following entity data:" in row for row in cmd(
            'data get entity @e[type=minecraft:item,nbt={Item:{id:"minecraft:bone_meal"}},limit=1] Item'))
        output["cooldown_after_eight"] = int(data(catfish, "HuntingCooldown"))
        cmd(f"tp {catfish} 7.5 102.25 8.5")
        cmd("tp @s 7.5 101 6.5")
        cmd("gamemode survival @s")
        cmd("item replace entity @s weapon.mainhand with minecraft:water_bucket")
        ctl("look", 0, 6)
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        inventory = data("@s", "Inventory")
        assert 'id: "naturalist:catfish_bucket"' in inventory, (inventory, ctl("state"))
        output["bucket_has_variant"] = "Variant:" in inventory
        output["bucket_has_cooldown"] = "HuntingCooldown:" in inventory
        print(json.dumps(output, indent=2), flush=True)
        assert output == reference, (output, reference)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
