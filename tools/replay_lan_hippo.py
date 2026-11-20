"""Replay Hippo taming and commands from a connected guest."""
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


with tempfile.TemporaryDirectory(prefix="nat-lan-hippo-", ignore_cleanup_errors=True) as world:
    root = Path(world)
    h = root / "host.sock"
    g = root / "guest.sock"
    host = lambda *args: command(h, *args)
    guest = lambda *args: command(g, *args)
    try:
        home = root / "host"
        home.mkdir()
        host("launch", "--headless", "--game", game, "--resources", resources, "--home", home,
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist Hippo LAN", "--creative",
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
        run("tp @s 14.5 92 7.5")
        run("tp NatGuest 8.5 92 6.5")
        run("gamemode survival NatGuest")
        run("summon naturalist:hippo 8.5 92 8.5 {Age:-24000,NoAI:1b,NoGravity:1b,Tags:[lan_hippo]}")
        actor = "@e[tag=lan_hippo,limit=1]"

        def step(n):
            run(f"tick step {n}")
            guest("wait", n + 3)

        def field(entity, key):
            return next((line.split("following entity data: ", 1)[1] for line in
                 run(f"data get entity {entity} {key}") if "following entity data: " in line), None)

        def click():
            [mob] = guest("entities", 6, "naturalist:hippo")["entities"]
            look = guest("look-at-entity", mob["id"])
            assert look["target"]["kind"] == "entity" and look["target"]["entity"]["id"] == mob["id"], look
            guest("mouse", "right", "click")
            guest("wait", 5)

        run("item replace entity NatGuest weapon.mainhand with minecraft:melon 8")
        step(4)
        click()
        assert field(actor, "Owner") == field("NatGuest", "UUID")
        assert field(actor, "Sitting") == "1b"
        run(f"damage {actor} 7 minecraft:generic")
        hurt = float(field(actor, "Health").rstrip("fF"))
        click()
        assert float(field(actor, "Health").rstrip("fF")) == hurt + 4
        run("item replace entity NatGuest weapon.mainhand with naturalist:whistle")
        guest("wait", 5)
        click()
        assert field(actor, "FollowingOwner") == "0b" and field(actor, "Sitting") == "0b"
        run("item replace entity NatGuest weapon.mainhand with minecraft:air")
        guest("wait", 5)
        click()
        assert field(actor, "Sitting") == "0b"
        guest("key", "shift", "down")
        try:
            step(2)
            click()
        finally:
            guest("key", "shift", "up")
        assert field(actor, "Sitting") == "1b"

        expected = "Test passed"
        check = run("execute if entity @a[name=NatGuest,advancements={naturalist:husbandry/feed_hippo_melon=true}]")
        assert any(expected in row for row in check), check
        print("A real guest tamed and healed the baby Hippo, used the whistle, and crouched to sit it; the melon advancement was awarded.", flush=True)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
