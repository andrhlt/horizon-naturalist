"""Exercise actual player-fed Rat and Cow families using physical clicks."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, lan=False):
    with tempfile.TemporaryDirectory(prefix="nat-rat-family-", ignore_cleanup_errors=True) as folder:
        home = Path(folder)
        host_socket = home / "host.sock"
        guest_socket = home / "guest.sock"
        socket = guest_socket if lan else host_socket

        def ctl(*args, via=None):
            proc = subprocess.run([version / "horizon-ctl", "--socket", via or socket, *map(str, args)],
                                  capture_output=True, text=True, timeout=75)
            assert proc.returncode == 0, (args, proc.stdout, proc.stderr)
            return json.loads(proc.stdout.splitlines()[-1])

        def cmd(line):
            line = line.replace("@s", "RatFamilyGuest") if lan else line
            return ctl("command", line, via=host_socket)["chat"]

        def data(target, path):
            prefix = "following entity data: "
            rows = cmd(f"data get entity {target} {path}")
            return next((row.split(prefix, 1)[1] for row in rows if prefix in row), None)

        def target(tag):
            return f"@e[tag={tag},limit=1]"

        def exists(tag):
            return data(target(tag), "UUID") is not None

        def teleport(x, y, z):
            cmd(f"tp @s {x} {y} {z}")
            ctl("wait", 5)

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2, via=host_socket)

        def launch(profile, profile_socket, *args):
            profile.mkdir(exist_ok=True)
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", profile, "--game-dir", profile,
                "--render-distance", 4, *args, via=profile_socket)

        def click_mob(x, y, z, kind):
            ctl("look-at", x, y, z)
            hit = ctl("looking-at")
            assert hit["kind"] == "entity" and hit["entity"]["type"] == kind, (x, y, z, kind, hit, ctl("status").get("player"))
            ctl("mouse", "right", "click")
            ctl("wait", 3)

        try:
            launch(home / "host", host_socket, "--world", "Naturalist Actual Rat Families",
                   "--creative", "--test-terrain", "--wait")
            if lan:
                port = ctl("lan", "open", 0, "commands", via=host_socket)["port"]
                launch(home / "guest", guest_socket)
                ctl("connect", f"127.0.0.1:{port}", "RatFamilyGuest")
                ctl("wait-for", "loaded", "--timeout", 55)
            teleport(8.5, 102, 6.5)
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:advance_time false")
            cmd("gamerule minecraft:mob_drops false")
            cmd("time set 1000")
            cmd("fill 2 101 2 15 101 15 minecraft:stone")
            cmd("fill 2 102 2 15 106 15 minecraft:air")
            cmd("gamemode survival @s")
            teleport(8.5, 102, 6.5)
            cmd("tick freeze")
            owner = data("@s", "UUID")
            assert owner
            for right_kind in ("minecraft:cow", "naturalist:rat"):
                cmd("kill @e[type=naturalist:rat]")
                cmd("kill @e[type=minecraft:cow]")
                step(25)
                teleport(8.5, 102, 6.5)
                cmd("clear @s")
                positions = (
                    ("left", "naturalist:rat", 8.5, 0.25, "minecraft:bread"),
                    ("right", right_kind, 9.5, 0.25 if right_kind == "naturalist:rat" else 0.65,
                     "minecraft:bread" if right_kind == "naturalist:rat" else "minecraft:wheat"),
                )
                for name, kind, x, look_y, food in positions:
                    ttl = f",Owner:{owner}" if kind == "naturalist:rat" else ""
                    rows = cmd(f"summon {kind} {x} 102 8.5 "
                               f"{{OnGround:1b,Tags:[{name},parents]{ttl}}}")
                    assert exists(name), rows
                step(1)
                ctl("wait", 5)
                for name, kind, x, look_y, food in positions:
                    cmd(f"item replace entity @s weapon.mainhand with {food}")
                    click_mob(x, 102 + look_y, 8.5, kind)
                    assert data(target(name), "InLove") == "600", (rows, data(target(name), "InLove"))
                    assert data("@s", "SelectedItem.id") is None
                teleport(12.5, 102, 12.5)
                step(90)
                love = [data(target(name), "InLove") for name, *_ in positions]
                new_rat = data("@e[type=naturalist:rat,tag=!parents,limit=1]", "Age")
                new_cow = data("@e[type=minecraft:cow,tag=!parents,limit=1]", "Age")
                if right_kind == "minecraft:cow":
                    assert love == ["510", "510"] and new_rat is None and new_cow is None, (love, new_rat, new_cow)
                    print("Player-fed Rat and Cow stayed in love without selecting each other", flush=True)
                else:
                    assert love == ["0", "0"] and new_rat and -24000 <= int(new_rat) < 0 and new_cow is None, (
                        love, new_rat, new_cow)
                    assert data("@e[type=naturalist:rat,tag=!parents,limit=1]", "Owner") == owner
                    print("Two player-fed Rats produced an actual owner-inheriting baby", flush=True)
        finally:
            for profile in (guest_socket, host_socket):
                if profile.exists():
                    ctl("quit", via=profile)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--lan", action="store_true")
    args = parser.parse_args()
    capture(args.version, args.lan)
