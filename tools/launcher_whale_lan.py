"""Verify a connected guest attacks and feeds separate Whale parts."""
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


with tempfile.TemporaryDirectory(prefix="nat-lan-whale-", ignore_cleanup_errors=True) as world:
    root = Path(world)
    h = root / "host.sock"
    g = root / "guest.sock"
    host = lambda *args: command(h, *args)
    guest = lambda *args: command(g, *args)
    try:
        home = root / "host"
        home.mkdir()
        host("launch", "--headless", "--game", game, "--resources", resources, "--home", home,
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist Whale LAN", "--creative",
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
        run("fill 4 91 0 16 91 17 minecraft:stone")
        run("fill 4 92 0 16 96 17 minecraft:air")
        run("tp @s 14.5 92 7.5")
        run("tp NatGuest 8.5 92 6.5")
        run("gamemode survival NatGuest")
        kind = "naturalist:whale"
        run(f"summon {kind} 8.5 92 6.5 {{NoAI:1b,NoGravity:1b,Health:70.0f,Rotation:[0.0f,0.0f],Tags:[lan_whale]}}")
        run("tp NatGuest 11.0 92 10.5")
        whale = "@e[tag=lan_whale,limit=1]"

        def step(n):
            run(f"tick step {n}")
            guest("wait", n + 3)

        def health():
            return next(float(line.split("following entity data: ", 1)[1].rstrip("fF"))
                for line in run(f"data get entity {whale} Health") if "following entity data: " in line)

        step(4)
        [tracked] = guest("entities", 8, kind)["entities"]
        guest("look-at", 8.5, 93.2, 10.5)
        looks = guest("looking-at")
        target = looks.get("target", looks)
        assert target.get("entity") and target["entity"]["id"] == tracked["id"], looks
        start = health()
        guest("mouse", "left", "click")
        step(4)
        after = health()
        assert after < start, ("head damaged by real guest", start, after)

        run("tp NatGuest 10.75 92 1.2")
        run("item replace entity NatGuest weapon.mainhand with minecraft:cod 2")
        step(2)
        guest("look-at", 8.5, 93.2, 1.2)
        looks = guest("looking-at")
        target = looks.get("target", looks)
        assert target.get("entity") and target["entity"]["id"] == tracked["id"], looks
        guest("mouse", "right", "click")
        step(5)
        assert health() == after + 4, (after, health())
        assert guest("state")["held"]["count"] == 1
        print("The connected guest attacked the head and used the tail to heal the same authoritative Whale.", flush=True)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
