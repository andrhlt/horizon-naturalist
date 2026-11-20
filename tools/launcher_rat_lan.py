"""Exercise Rat ownership, whistle, and nonowner net capture over headless LAN."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])

with tempfile.TemporaryDirectory(prefix="nat-rat-lan-", ignore_cleanup_errors=True) as temp:
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

    def hold(item, hand="mainhand"):
        command(f"item replace entity RatGuest weapon.{hand} with {item}")
        control("guest", "wait", 4)

    def click(wait=6):
        entries = control("guest", "entities", 6, "naturalist:rat")["entities"]
        assert len(entries) == 1, entries
        target = control("guest", "look-at-entity", entries[0]["id"])["target"]
        assert target["kind"] == "entity", target
        control("guest", "mouse", "right", "click")
        control("host", "wait", wait)

    target = "@e[type=naturalist:rat,tag=fixture_rat_lan,limit=1]"
    try:
        launch("host", "--world", "Naturalist Rat LAN", "--creative", "--test-terrain", "--wait")
        command("gamerule minecraft:spawn_mobs false")
        command("forceload add 0 0 15 15")
        command("tp @s 4.5 101 8.5")
        control("host", "wait", 22)
        command("fill 0 100 0 15 100 15 minecraft:stone")
        command("fill 0 101 0 15 106 15 minecraft:air")
        command("tp @s 4.5 101 8.5")
        port = control("host", "lan", "open", 0, "commands")["port"]
        launch("guest")
        control("guest", "connect", f"127.0.0.1:{port}", "RatGuest")
        control("guest", "wait-for", "loaded", "--timeout", 55)
        command("tp RatGuest 8.5 101 6.5")
        command("gamemode survival RatGuest")
        command("gamemode spectator @s")
        owner = data("RatGuest", "UUID")
        host_owner = data("@s", "UUID")
        assert owner and host_owner and owner != host_owner
        cases = {}
        expected = {}
        for trial in range(8):
            command(f"summon naturalist:rat 8.5 101 8.5 {{Owner:{owner},Age:6000,NoAI:1b,Tags:[fixture_rat_lan]}}")
            control("guest", "wait", 8)
            command("clear RatGuest")
            offhand = trial % 2 != 0
            if offhand:
                hold("naturalist:whistle", "offhand")
            click()
            sitting, following = data(target, "Sitting"), data(target, "FollowingOwner")
            hold("minecraft:bread" if offhand else "naturalist:whistle")
            click(wait=3)
            cases[str(trial)] = [sitting, following, data(target, "FollowingOwner"), data(target, "Sitting")]
            # Java Animal.mobInteract returns CONSUME for a tame adult's food
            # on the client, including while the server's breeding age is positive.
            expected[str(trial)] = ["1b", "1b", "1b", "1b"] if offhand else ["1b", "1b", "0b", "0b"]
            if trial == 0:
                click(wait=3)
                cooldown = [data(target, "FollowingOwner"), data(target, "Sitting")]
                print("Rat click during whistle cooldown", cooldown, flush=True)
            print(f"Guest-owned Rat trial {trial}", cases[str(trial)], flush=True)
            command(f"kill {target}")
            control("host", "wait", 25)
        # Java's Rat mobInteract permits any player to capture a tamed Rat. Its
        # sit and whistle choices require the interacting player to be the owner.
        command(f"summon naturalist:rat 8.5 101 8.5 {{Owner:{host_owner},NoAI:1b,Tags:[fixture_rat_lan]}}")
        control("guest", "wait", 8)
        command("clear RatGuest")
        click()
        hold("naturalist:whistle")
        click()
        nonowner = [data(target, "FollowingOwner"), data(target, "Sitting")]
        print("Other-owned Rat sit/whistle", nonowner, flush=True)
        hold("naturalist:capture_net")
        click()
        inventory = data("RatGuest", "Inventory") or ""
        captured = 'id: "naturalist:rat"' in inventory and host_owner in inventory and data(target, "UUID") is None
        print("Other-owned Rat captured", bool(captured), inventory, flush=True)
        mismatches = {case: values for case, values in cases.items() if values != expected[case]}
        assert not mismatches and nonowner == ["1b", "0b"] and captured and cooldown == ["0b", "1b"], (mismatches, nonowner, captured, cooldown)
    finally:
        for side in ("guest", "host"):
            if (root / f"{side}.sock").exists():
                control(side, "quit")
