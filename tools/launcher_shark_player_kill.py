"""Measure an Shark hunting cooldown after its real player kill."""
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
             "--game-dir", home, "--name", "NatHost", "--world", "Shark LAN", "--creative",
             "--test-terrain", "--render-distance", 4, "--wait")
        def run(line):
            return host("command", line)["chat"]
        run("gamerule minecraft:spawn_mobs false")
        run("gamerule minecraft:natural_health_regeneration false")
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
        run("time set 18000")
        run("tick freeze")
        run("fill 4 91 3 14 91 14 minecraft:stone")
        run("fill 4 92 3 14 96 14 minecraft:air")
        run("fill 5 92 8 13 95 13 minecraft:water")
        run("gamemode survival NatGuest")
        run("tp @s 13.5 92 8.5")
        run("tp NatGuest 11.5 93 8.5")
        host("wait", 10)
        run("damage NatGuest 19 minecraft:generic")
        host("wait", 25)
        assert guest("state")["player"]["health"] == 1.0, guest("state")
        run('summon naturalist:great_white_shark 9.5 93 8.5 {NoGravity:1b,Rotation:[-90.0f,0.0f],PersistenceRequired:1b}')
        fish = "@e[type=naturalist:great_white_shark,limit=1]"
        run(f"damage {fish} 1 minecraft:player_attack by NatGuest")
        for _ in range(90):
            run("tick step 1")
            host("wait", 4)
            if guest("state")["player"]["health"] == 0:
                break
        rows = run(f"data get entity {fish} HuntingCooldown")
        cooldown = next(int(row.split("following entity data: ", 1)[1])
            for row in rows if "following entity data: " in row)
        health = guest("state")["player"]["health"]
        run("tick step 1")
        host("wait", 4)
        next_rows = run(f"data get entity {fish} HuntingCooldown")
        next_cooldown = next(int(row.split("following entity data: ", 1)[1])
            for row in next_rows if "following entity data: " in row)
        result = {"hit": health == 0.0, "player_dead": health == 0.0,
                  "cooldown": cooldown, "cooldown_next_tick": next_cooldown}
        print(json.dumps(result), flush=True)
        if len(sys.argv) >= 3:
            reference = json.loads(Path(sys.argv[2]).read_text())
            assert result == reference, (result, reference)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
