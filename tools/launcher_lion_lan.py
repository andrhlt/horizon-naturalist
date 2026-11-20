"""Exercise Lion ownership and edible items from a headless LAN guest."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
with tempfile.TemporaryDirectory(prefix="nat-lion-lan-", ignore_cleanup_errors=True) as temp:
    root = Path(temp)

    def control(side, *args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def launch(side, *args):
        home = root / side
        home.mkdir()
        control(side, "launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--render-distance", 4, *args)

    def command(line):
        return control("host", "command", line)["chat"]

    def data(target, field):
        marker = " following entity data: "
        return next((line.split(marker, 1)[1] for line in command(f"data get entity {target} {field}")
                     if marker in line), None)

    def give(item, n=1):
        command(f"item replace entity LionGuest weapon.mainhand with {item} {n}")
        control("guest", "wait", 4)

    def click(x, y):
        control("guest", "look-at", x, y, 8.5)
        target = control("guest", "looking-at")
        assert target["kind"] == "entity", target
        control("guest", "mouse", "right", "click")
        control("host", "wait", 6)

    try:
        launch("host", "--world", "Naturalist Lion LAN", "--creative", "--test-terrain", "--wait")
        command("gamerule minecraft:spawn_mobs false")
        command("forceload add 0 0 15 15")
        command("tp @s 4.5 101 8.5")
        control("host", "wait", 22)
        command("fill 0 100 0 15 100 15 minecraft:stone")
        command("fill 0 101 0 15 106 15 minecraft:air")
        command("tp @s 4.5 101 8.5")
        port = control("host", "lan", "open", 0, "commands")["port"]
        launch("guest")
        control("guest", "connect", f"127.0.0.1:{port}", "LionGuest")
        control("guest", "wait-for", "loaded", "--timeout", 55)
        command("tp LionGuest 8.5 101 5.5")
        command("gamemode survival LionGuest")
        command("gamemode spectator @s")
        owner = data("LionGuest", "UUID")
        assert owner, "Host must see the actual guest UUID"
        adult = "@e[type=naturalist:lion,tag=lan_adult,limit=1]"
        cub = "@e[type=naturalist:lion,tag=lan_cub,limit=1]"
        command(f"summon naturalist:lion 8.5 101 8.5 {{Owner:{owner},Health:12.0f,NoAI:1b,Tags:[lan_adult]}}")
        command(f"summon naturalist:lion 11.5 101 8.5 {{Owner:{owner},Age:-24000,NoAI:1b,Tags:[lan_cub]}}")
        control("guest", "wait", 12)
        give("minecraft:cooked_mutton", 2)
        click(8.5, 102.0)
        got = data(adult, "Health")
        inventory = data("LionGuest", "Inventory")
        result = {"healed_six": got == "18.0f", "healing_consumed_one": "count: 1" in inventory}
        command("clear LionGuest")
        click(8.5, 102.0)
        result["sat"] = data(adult, "Sitting") == "1b"
        give("naturalist:whistle")
        click(8.5, 102.0)
        print("After whistle", data(adult, "FollowingOwner"), data(adult, "Sitting"), data(adult, "Owner"), flush=True)
        result["whistle_wander"] = data(adult, "FollowingOwner") == "0b"
        result["whistle_released_sit"] = data(adult, "Sitting") == "0b"
        command("tp LionGuest 11.5 101 5.5")
        give("minecraft:beef", 2)
        age = int(data(cub, "Age"))
        click(11.5, 101.5)
        result["cub_grew"] = 2370 <= int(data(cub, "Age")) - age <= 2425
        result["cub_consumed_one"] = "count: 1" in data("LionGuest", "Inventory")
        print(json.dumps(result, indent=2), flush=True)
        assert all(result.values()), result
    finally:
        for side in ("guest", "host"):
            if (root / f"{side}.sock").exists():
                control(side, "quit")
