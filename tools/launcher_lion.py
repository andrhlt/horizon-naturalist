"""Check real Lion owner feeding, growth, sitting and whistle interactions."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

from launcher_managed import launch_managed, wait_managed

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-lion-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=False, capture_output=True, text=True, timeout=75)
        if result.returncode:
            raise RuntimeError(
                f"horizon-ctl {args!r} failed ({result.returncode}): "
                f"stdout={result.stdout!r}; stderr={result.stderr!r}"
            )
        return json.loads(result.stdout.splitlines()[-1])

    def command(line):
        return ctl("command", line)["chat"]

    def data(target, key):
        marker = " following entity data: "
        return next((row.split(marker, 1)[1] for row in command(f"data get entity {target} {key}")
                     if marker in row), None)

    def click(kind, x):
        command(f"tp @s {x} 92 5.5")
        ctl("wait", 2)
        entities = ctl("entities", 8, kind)["entities"]
        target = min(entities, key=lambda row: abs(row["pos"][0] - x))
        ctl("look-at-entity", target["id"])
        ctl("mouse", "right", "click")
        ctl("wait", 3)

    play = None
    try:
        play = launch_managed(version, socket, home, "Naturalist Lion Interactions",
                              creative=True, test_terrain=True)
        command("gamerule minecraft:spawn_mobs false")
        command("fill 2 91 2 18 91 12 minecraft:stone")
        command("fill 2 92 2 18 96 12 minecraft:air")
        command("tp @s 7.5 92 3.5")
        command("gamemode survival @s")
        ctl("wait", 20)
        owner = data("@s", "UUID")
        assert owner and owner.startswith("[I;"), owner
        command(f"summon naturalist:lion 8.5 92 7.5 {{NoAI:1b,Tame:1b,Owner:{owner},Mane:1b,Health:12.0f,Sitting:0b,FollowingOwner:1b,Tags:[lion_owner]}}")
        command(f"summon naturalist:lion 11.5 92 7.5 {{NoAI:1b,Tame:1b,Owner:{owner},Age:-24000,Sitting:0b,FollowingOwner:1b,Tags:[lion_cub]}}")
        ctl("wait", 8)
        adult = "@e[type=naturalist:lion,tag=lion_owner,limit=1]"
        cub = "@e[type=naturalist:lion,tag=lion_cub,limit=1]"
        command("item replace entity @s weapon.mainhand with minecraft:cooked_mutton 2")
        click("naturalist:lion", 8.5)
        inventory = " ".join(command("data get entity @s Inventory"))
        output = {
            "owner_heal": [float(data(adult, "Health").rstrip("fF")), 1 if "count: 1" in inventory else -1],
        }
        command("item replace entity @s weapon.mainhand with minecraft:beef 2")
        age_before = int(data(cub, "Age"))
        click("naturalist:lion", 11.5)
        age_after = int(data(cub, "Age"))
        inventory = " ".join(command("data get entity @s Inventory"))
        output["cub_age_before"] = age_before
        output["cub_age_after"] = age_after
        output["cub_age_gain"] = age_after - age_before
        output["cub_food_remaining"] = 1 if "count: 1" in inventory else -1
        output["tamed_cub_grows"] = 2300 <= age_after - age_before <= 2400 and "count: 1" in inventory
        command("clear @s")
        click("naturalist:lion", 8.5)
        output["owner_sits"] = data(adult, "Sitting") == "1b"
        command("item replace entity @s weapon.mainhand with naturalist:whistle")
        click("naturalist:lion", 8.5)
        output["owner_whistle_stops_follow"] = data(adult, "FollowingOwner") == "0b"
        print(json.dumps(output, indent=2), flush=True)
        assert output["owner_heal"] == reference["owner_heal"], output
        assert output["tamed_cub_grows"] == reference["tamed_cub_grows"], output
        assert output["owner_sits"] == reference["owner_sits"], output
        assert output["owner_whistle_stops_follow"] == reference["owner_whistle_stops_follow"], output
    finally:
        if socket.exists():
            ctl("quit")
        wait_managed(play)
