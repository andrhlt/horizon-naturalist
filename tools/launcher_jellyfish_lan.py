"""Measure one Jellyfish sting across both real headless LAN clients."""
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


with tempfile.TemporaryDirectory(prefix="nat-lan-jellyfish-", ignore_cleanup_errors=True) as world:
    root = Path(world)
    h = root / "host.sock"
    g = root / "guest.sock"
    host = lambda *args: command(h, *args)
    guest = lambda *args: command(g, *args)
    try:
        home = root / "host"
        home.mkdir()
        host("launch", "--headless", "--game", game, "--resources", resources, "--home", home,
             "--game-dir", home, "--name", "NatHost", "--world", "Jellyfish LAN", "--creative",
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
        run("fill 4 91 3 14 91 14 minecraft:stone")
        run("fill 4 92 3 14 96 14 minecraft:air")
        run("fill 5 92 8 13 95 13 minecraft:water")
        run("gamemode survival @a")
        run("tp @s 8 92 8.5")
        run("tp NatGuest 9 92 8.5")
        host("wait", 5)
        print("Pre", host("state")["player"], guest("state")["player"], flush=True)
        assert guest("state")["player"]["position"][0] == 9.5, guest("state")
        run('summon naturalist:jellyfish 8.9 92 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b}')
        run("tick step 1")
        host("wait", 6)
        before = {"host_health": host("state")["player"]["health"], "guest_health": guest("state")["player"]["health"]}
        print(json.dumps(before), flush=True)
        if "--assert" in sys.argv[2:]:
            assert before == {"host_health": 18.0, "guest_health": 18.0}, before
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
