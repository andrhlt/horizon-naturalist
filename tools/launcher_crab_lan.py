"""Replay Crab ownership, capture, and release from a connected guest."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
ctl = version / "horizon-ctl"
game = version / "horizon-client"
resources = version / ".local/horizon-client/26.3"


def command(socket, *args):
    call = subprocess.run([ctl, "--socket", socket, *map(str, args)], capture_output=True,
                          check=True, text=True, timeout=60)
    return json.loads(call.stdout.splitlines()[-1])


with tempfile.TemporaryDirectory(prefix="nat-lan-crab-", ignore_cleanup_errors=True) as world:
    root = Path(world)
    h = root / "host.sock"
    g = root / "guest.sock"
    host = lambda *args: command(h, *args)
    guest = lambda *args: command(g, *args)
    try:
        home = root / "host"
        home.mkdir()
        host("launch", "--headless", "--game", game, "--resources", resources, "--home", home,
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist Crab LAN", "--creative",
             "--test-terrain", "--render-distance", 4, "--wait")
        def run(line):
            return host("command", line)["chat"]
        run("gamerule minecraft:spawn_mobs false")
        run("gamerule minecraft:natural_regeneration false")
        run("difficulty normal")
        run("tp @s 8.5 92 6.5")
        host("wait", 20)
        lan = host("lan", "open", 0, "commands")
        print("LAN", lan, flush=True)
        address = f"127.0.0.1:{lan['port']}"
        guest_home = root / "guest"
        guest_home.mkdir()
        guest("launch", "--headless", "--game", game, "--resources", resources, "--home", guest_home,
              "--game-dir", guest_home, "--name", "NatGuest", "--server", address, "--render-distance", 4)
        for _ in range(60):
            if g.exists() and guest("state")["screen"] == "playing":
                break
            time.sleep(0.5)
        else:
            raise RuntimeError("Guest did not join")
        print("Players", host("lan", "status"), flush=True)
        run("tick freeze")
        run("fill 4 91 3 16 91 16 minecraft:stone")
        run("fill 4 92 3 16 96 16 minecraft:air")
        run("gamemode survival NatGuest")
        run("tp @s 14.5 92 7.5")
        run("tp NatGuest 8.5 92 7.4")
        run('summon naturalist:crab 8.5 92 8.5 {NoAI:1b,NoGravity:1b,Variant:"naturalist:yellow",Tags:[guest_pet]}')
        original = "@e[type=naturalist:crab,tag=guest_pet,limit=1]"
        run("tick step 3")
        guest("wait", 6)

        def field(entity, key):
            replies = run(f"data get entity {entity} {key}")
            return next((row.split("following entity data: ", 1)[1] for row in replies if "following entity data: " in row), None)

        def click():
            [mob] = guest("entities", 5, "naturalist:crab")["entities"]
            looked = guest("look-at-entity", mob["id"])
            assert looked["target"]["kind"] == "entity", looked
            guest("mouse", "right", "click")
            guest("wait", 4)

        run("item replace entity NatGuest weapon.mainhand with minecraft:tropical_fish 32")
        guest("wait", 3)
        for _ in range(32):
            click()
            owner = field(original, "Owner")
            if owner:
                break
        assert owner and owner == field("NatGuest", "UUID"), owner
        run(f"item replace entity {original} weapon.mainhand with minecraft:iron_axe[minecraft:damage=7]")
        run("item replace entity NatGuest weapon.mainhand with naturalist:capture_net")
        guest("wait", 3)
        click()
        assert field(original, "Owner") is None
        inventory = "\n".join(run("data get entity NatGuest Inventory"))
        assert 'id: "naturalist:crab"' in inventory and '"minecraft:damage": 1' in inventory, inventory

        guest("select", "naturalist:crab")
        guest("look-at-block", 8, 91, 8)
        guest("mouse", "right", "click")
        guest("wait", 6)
        released = "@e[type=naturalist:crab,nbt={FromHand:1b},limit=1]"
        assert field(released, "Owner") == owner
        assert field(released, "Variant") == '"naturalist:yellow"'
        gear = field(released, "equipment.mainhand")
        assert 'id: "minecraft:iron_axe"' in gear and '"minecraft:damage": 7' in gear, gear
        run("item replace entity NatGuest weapon.mainhand with naturalist:whistle")
        guest("wait", 3)
        click()
        assert field(released, "FollowingOwner") == "0b"
        print("Guest tamed, captured, and released their yellow Crab with its damaged weapon, then used the whistle.", flush=True)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
