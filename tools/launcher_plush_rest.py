"""Check whether holding the Plush Bear in either hand allows bed use near a monster."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, lan):
    with tempfile.TemporaryDirectory(prefix="nat-plush-rest-", ignore_cleanup_errors=True) as folder:
        home = Path(folder)
        host_socket = home / "host.sock"
        guest_socket = home / "guest.sock"
        socket = guest_socket if lan else host_socket

        def ctl(*args, via=None):
            result = subprocess.run([version / "horizon-ctl", "--socket", via or socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            if lan:
                line = line.replace("@s", "PlushGuest")
            return ctl("command", line, via=host_socket)["chat"]

        def bed_occupied():
            rows = cmd("execute if block 8 102 9 minecraft:red_bed[facing=south,part=head,occupied=true] run say occupied-rest")
            return any("occupied-rest" in line for line in rows)

        def reset_bed():
            cmd("setblock 8 102 9 minecraft:air")
            cmd("setblock 8 102 8 minecraft:air")
            ctl("wait", 6)
            cmd("setblock 8 102 8 minecraft:red_bed[facing=south,part=foot,occupied=false]")
            cmd("setblock 8 102 9 minecraft:red_bed[facing=south,part=head,occupied=false]")
            ctl("wait", 3)
            cmd("tp @s 8.5 102 6.5")

        try:
            def launch(profile, profile_socket, *args):
                profile.mkdir()
                ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", profile, "--game-dir", profile,
                    "--render-distance", 4, *args, via=profile_socket)

            launch(home / "host", host_socket,
                   "--world", "Naturalist Plush Rest", "--creative", "--test-terrain", "--wait")
            if lan:
                port = ctl("lan", "open", 0, "commands", via=host_socket)["port"]
                launch(home / "guest", guest_socket)
                ctl("connect", f"127.0.0.1:{port}", "PlushGuest")
                ctl("wait-for", "loaded", "--timeout", 55)
            cmd("gamemode creative @s")
            cmd("gamerule minecraft:fall_damage false")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:advance_time false")
            cmd("difficulty normal")
            cmd("tp @s 8.5 102 6.5")
            ctl("wait", 25)
            cmd("fill 2 101 2 15 101 15 minecraft:stone")
            cmd("fill 2 102 2 15 109 15 minecraft:air")
            cmd("gamemode survival @s")
            result = {}
            for name, held, offhand, dark, monster in (
                ("empty_nearby", "air", "air", True, True),
                ("main_nearby", "naturalist:plush_bear", "air", True, True),
                ("offhand_nearby", "air", "naturalist:plush_bear", True, True),
                ("ordinary_item_nearby", "stick", "air", True, True),
                ("inventory_nearby", "air", "air", True, True),
                ("day_plush", "naturalist:plush_bear", "air", False, True),
                ("empty_no_monsters", "air", "air", True, False),
                ("angry_piglin_plush", "naturalist:plush_bear", "air", True, True),
                ("calm_piglin_plush", "naturalist:plush_bear", "air", True, True),
                ("blocked_plush", "naturalist:plush_bear", "air", True, True),
            ):
                cmd("kill @e[tag=plush_rest_monster]")
                reset_bed()
                cmd("clear @s")
                cmd(f"time set {'midnight' if dark else 'noon'}")
                if monster:
                    kind = "zombified_piglin" if "piglin" in name else "zombie"
                    if name == "angry_piglin_plush":
                        cmd("fill 8 102 10 11 103 10 minecraft:glass")
                    no_ai = "0b" if name == "angry_piglin_plush" else "1b"
                    cmd(f"summon minecraft:{kind} 9.5 102 11.5 {{NoAI:{no_ai},Silent:1b,Tags:[plush_rest_monster]}}")
                    if name == "angry_piglin_plush":
                        cmd("damage @e[tag=plush_rest_monster,limit=1] 1 minecraft:player_attack by @s")
                        ctl("wait", 15)
                cmd(f"item replace entity @s weapon.mainhand with {held}")
                cmd(f"item replace entity @s weapon.offhand with {offhand}")
                if name == "inventory_nearby":
                    cmd("item replace entity @s hotbar.8 with naturalist:plush_bear")
                if name == "blocked_plush":
                    cmd("setblock 8 103 9 minecraft:stone")
                ctl("look-at", "block", 8, 102, 8, "up")
                hit = ctl("looking-at")
                assert hit["kind"] == "block" and hit["block"]["block"] == "minecraft:red_bed", (name, hit)
                ctl("mouse", "right", "click")
                ctl("wait", 5)
                result[name] = bed_occupied()
                print(name, result[name], flush=True)
            return result
        finally:
            for profile in guest_socket, host_socket:
                if profile.exists():
                    ctl("quit", via=profile)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path, nargs="?")
    parser.add_argument("--baseline", action="store_true")
    parser.add_argument("--lan", action="store_true")
    args = parser.parse_args()
    result = capture(args.version, args.lan)
    print(json.dumps(result, indent=2), flush=True)
    expected = (json.loads(args.reference.read_text()) if args.reference else
                {"empty_nearby": False, "main_nearby": True, "offhand_nearby": True,
                 "ordinary_item_nearby": False, "inventory_nearby": False, "day_plush": False,
                 "empty_no_monsters": True, "angry_piglin_plush": False, "calm_piglin_plush": True, "blocked_plush": False})
    if args.baseline:
        expected["main_nearby"] = expected["offhand_nearby"] = False
    assert result == expected, (result, expected)
