"""Replay the pinned Capybara owner interactions headlessly."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
resources = Path.home() / "Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3"
shared_jars = Path.home() / "Library/Application Support/horizon/jars"

with tempfile.TemporaryDirectory(prefix="nat-capybara-", ignore_cleanup_errors=True) as temp:
    home = Path(temp)
    shutil.copytree(shared_jars, home / "jars")
    socket = home / "capybara.sock"
    env = os.environ.copy()
    env["HORIZON_HOME"] = str(home)

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=100, env=env)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def data(target, key=None):
        path = f" {key}" if key else ""
        rows = cmd(f"data get entity {target}{path}")
        value = next((row.split("following entity data: ", 1)[1] for row in rows
                      if "following entity data: " in row), None)
        return value if value is not None else "\n".join(rows)

    def click(target_id):
        cmd("tp @s 8.5 92 5.5")
        ctl("look", 0, 0)
        ctl("look-at-entity", target_id)
        target = ctl("looking-at")
        assert target and target.get("kind") == "entity", {"target": target, "state": ctl("state")}
        ctl("mouse", "right", "click")
        ctl("wait", 3)

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources", resources,
            "--home", home, "--game-dir", home, "--world", "Capybara Replay", "--creative",
            "--test-terrain", "--render-distance", 4, "--wait")
        for command in (
            "gamerule minecraft:spawn_mobs false",
            "tp @s 8.5 92 5.5",
            "tick freeze",
            "fill 4 91 3 18 91 16 minecraft:stone",
            "fill 4 92 3 18 98 16 minecraft:air",
            "summon naturalist:capybara 8.5 92 8.5 {NoAI:1b,PersistenceRequired:1b,Tags:[capybara_fixture]}",
            "gamemode survival @s",
        ):
            cmd(command)
        mob = ctl("entities", 5, "naturalist:capybara")["entities"][0]
        selector = "@e[type=naturalist:capybara,tag=capybara_fixture,limit=1]"
        player = "@s"
        cmd("item replace entity @s weapon.mainhand with minecraft:melon_slice 64")
        ctl("wait", 3)
        tame_attempts = 0
        while tame_attempts < 12:
            click(mob["id"])
            tame_attempts += 1
            if "Owner:" in data(selector):
                break
        assert "Owner:" in data(selector), {"attempts": tame_attempts, "entity": data(selector),
                                             "inventory": ctl("inventory")}
        owner = data(selector, "Owner")
        tame = data(selector, "Tame")
        dye = data(selector, "DyeColor")
        owner_saved = owner.startswith("[I;")
        assert owner_saved and tame.endswith("1b") and dye.endswith("14"), (owner, tame, dye)
        remaining = ctl("inventory")["counts"].get("minecraft:melon_slice", 0)

        cmd("item replace entity @s weapon.mainhand with minecraft:air")
        click(mob["id"])
        sitting = data(selector, "Sitting")
        cmd("item replace entity @s weapon.mainhand with naturalist:whistle")
        click(mob["id"])
        following = data(selector, "FollowingOwner")

        cmd(f"damage {selector} 10 minecraft:generic")
        cmd("item replace entity @s weapon.mainhand with minecraft:melon_slice 1")
        heal_count_before = ctl("inventory")["held"]["count"]
        click(mob["id"])
        healed = float(data(selector, "Health").rstrip("fF"))
        held_after = ctl("inventory")["held"]
        output = {
            "tame_attempts": tame_attempts,
            "tame_saved": tame.endswith("1b"),
            "owner_saved": owner_saved,
            "default_dye_id": 14 if dye.endswith("14") else dye,
            "melon_remaining_after_tame": remaining,
            "ordered_sit_saved": sitting.endswith("1b"),
            "whistle_following_disabled": following.endswith("0b"),
            "whistle_stood_pet_up": data(selector, "Sitting").endswith("0b"),
            "healed_health": healed,
            "heal_consumed": held_after is None or held_after["count"] < heal_count_before,
        }
        print(json.dumps(output, indent=2), flush=True)
    finally:
        try:
            ctl("quit")
        except Exception:
            pass
