"""Replay Giant Isopod defense and bucket pickup/release from a connected guest."""
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


with tempfile.TemporaryDirectory(prefix="nat-lan-isopod-", ignore_cleanup_errors=True) as world:
    root = Path(world)
    h = root / "host.sock"
    g = root / "guest.sock"
    host = lambda *args: command(h, *args)
    guest = lambda *args: command(g, *args)
    try:
        home = root / "host"
        home.mkdir()
        host("launch", "--headless", "--game", game, "--resources", resources, "--home", home,
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist Giant Isopod LAN", "--creative",
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
        run('summon naturalist:giant_isopod 8.5 92.65 8.5 {NoAI:1b,NoGravity:1b,Variant:"naturalist:blue",Tags:[guest_pickup]}')
        run("tick step 3")
        guest("wait", 5)
        isopod = "@e[tag=guest_pickup,limit=1]"
        run(f"damage {isopod} 4 minecraft:generic")
        health = run(f"data get entity {isopod} Health")
        assert any("following entity data: 10.0f" in row for row in health), health

        run("item replace entity NatGuest weapon.mainhand with minecraft:bucket")
        guest("wait", 3)
        [mob] = guest("entities", 5, "naturalist:giant_isopod")["entities"]
        looked = guest("look-at-entity", mob["id"])
        assert looked["target"]["kind"] == "entity", looked
        guest("mouse", "right", "click")
        guest("wait", 3)
        assert guest("state")["held"]["item"] == "minecraft:bucket"

        run("item replace entity NatGuest weapon.mainhand with minecraft:water_bucket")
        guest("wait", 3)
        looked = guest("look-at-entity", mob["id"])
        assert looked["target"]["kind"] == "entity", looked
        guest("mouse", "right", "click")
        guest("wait", 6)
        inventory = "\n".join(run("data get entity NatGuest Inventory"))
        assert guest("state")["held"]["item"] == "naturalist:giant_isopod_bucket", inventory
        assert inventory.count('Variant: "naturalist:blue"') >= 2, inventory

        run("setblock 8 92 8 minecraft:water")
        guest("wait", 3)
        guest("look-at-block", 8, 91, 8)
        guest("mouse", "right", "click")
        guest("wait", 6)
        entity = '@e[type=naturalist:giant_isopod,nbt={FromBucket:1b},limit=1]'
        variant = run(f"data get entity {entity} Variant")
        assert any('following entity data: "naturalist:blue"' in row for row in variant), variant
        assert guest("state")["held"]["item"] == "minecraft:bucket"
        print("Guest-triggered shell defense, empty-bucket rejection, blue pickup and blue release passed.", flush=True)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
