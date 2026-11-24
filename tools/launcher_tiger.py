"""Check real Tiger owner interactions and night hunting in headless Play."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

from launcher_managed import launch_managed, wait_managed

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-tiger-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def command(line):
        return ctl("command", line)["chat"]

    def step(ticks):
        command(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    def data(target, key):
        marker = " following entity data: "
        return next((row.split(marker, 1)[1] for row in command(f"data get entity {target} {key}")
                     if marker in row), None)

    def click(x):
        command(f"tp @s {x} 92 5.5")
        ctl("wait", 2)
        entities = ctl("entities", 8, "naturalist:tiger")["entities"]
        target = min(entities, key=lambda row: abs(row["pos"][0] - x))
        ctl("look-at-entity", target["id"])
        ctl("mouse", "right", "click")
        ctl("wait", 3)

    play = None
    try:
        play = launch_managed(version, socket, home, "Naturalist Tiger Interactions",
                              creative=True, test_terrain=True)
        command("gamerule minecraft:spawn_mobs false")
        command("fill 2 91 2 30 91 12 minecraft:stone")
        command("fill 2 92 2 30 96 12 minecraft:air")
        command("tp @s 7.5 92 3.5")
        command("gamemode survival @s")
        ctl("wait", 20)
        owner = data("@s", "UUID")
        assert owner and owner.startswith("[I;"), owner
        command(f"summon naturalist:tiger 6.5 92 7.5 {{NoAI:1b,Tame:1b,Owner:{owner},Health:12.0f,Sitting:0b,FollowingOwner:1b,Tags:[tiger_owner]}}")
        command(f"summon naturalist:tiger 9.5 92 7.5 {{NoAI:1b,Tame:1b,Owner:{owner},Age:-24000,Sitting:0b,FollowingOwner:1b,Tags:[tiger_cub]}}")
        ctl("wait", 8)
        adult = "@e[type=naturalist:tiger,tag=tiger_owner,limit=1]"
        cub = "@e[type=naturalist:tiger,tag=tiger_cub,limit=1]"
        command("item replace entity @s weapon.mainhand with minecraft:cooked_mutton 2")
        click(6.5)
        inventory = " ".join(command("data get entity @s Inventory"))
        output = {
            "owner_heal": [float(data(adult, "Health").rstrip("fF")), 1 if "count: 1" in inventory else -1],
        }
        command("item replace entity @s weapon.mainhand with minecraft:beef 2")
        age_before = int(data(cub, "Age"))
        click(9.5)
        age_after = int(data(cub, "Age"))
        inventory = " ".join(command("data get entity @s Inventory"))
        output["tamed_cub_grows"] = 2300 <= age_after - age_before <= 2400 and "count: 1" in inventory
        command("clear @s")
        click(6.5)
        output["owner_sits"] = data(adult, "Sitting") == "1b"
        command("item replace entity @s weapon.mainhand with naturalist:whistle")
        click(6.5)
        output["owner_whistle_stops_follow"] = data(adult, "FollowingOwner") == "0b"
        print("Owner interactions", json.dumps(output), flush=True)
        assert output["owner_heal"] == reference["owner_heal"], output
        assert output["tamed_cub_grows"] == reference["tamed_cub_grows"], output
        assert output["owner_sits"] == reference["owner_sits"], output
        assert output["owner_whistle_stops_follow"] == reference["owner_whistle_stops_follow"], output

        command("kill @e[type=naturalist:tiger]")
        step(25)
        command("gamemode spectator @s")
        command("tp @s 7.5 92 3.5")
        command("time set 18000")
        command("summon naturalist:tiger 6.5 92 7.5 {PersistenceRequired:1b,Tags:[tiger_hunter]}")
        command("summon minecraft:pig 15.5 92 7.5 {NoAI:1b,Health:20.0f,Tags:[tiger_prey]}")
        prey = "@e[type=minecraft:pig,tag=tiger_prey,limit=1]"
        start = float(data(prey, "Health").rstrip("fF"))
        for _ in range(60):
            step(5)
            if float(data(prey, "Health").rstrip("fF")) < start:
                break
        output["night_hunt_health"] = float(data(prey, "Health").rstrip("fF"))
        print("Night hunt", output["night_hunt_health"], flush=True)
        assert output["night_hunt_health"] < start, output
    finally:
        if socket.exists():
            ctl("quit")
        wait_managed(play)
