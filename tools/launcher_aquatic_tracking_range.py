import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])


def call(socket, *args):
    response = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=60)
    return json.loads(response.stdout.splitlines()[-1])


with tempfile.TemporaryDirectory(prefix="nat-aquatic-tracking-", ignore_cleanup_errors=True) as tmp:
    root = Path(tmp)
    host_socket, guest_socket = root / "host.sock", root / "guest.sock"
    host = lambda *args: call(host_socket, *args)
    guest = lambda *args: call(guest_socket, *args)
    command = lambda message: host("command", message)["chat"]
    try:
        def launch(socket, directory, player, *options):
            folder = root / directory
            folder.mkdir()
            return call(socket, "launch", "--headless", "--game", version / "horizon-client",
                        "--resources", version / ".local/horizon-client/26.3", "--home", folder,
                        "--game-dir", folder, "--name", player, "--render-distance", 12, *options)

        launch(host_socket, "host", "NatHost", "--world", "Naturalist aquatic tracking", "--creative", "--test-terrain", "--wait")
        command("gamerule minecraft:spawn_mobs false")
        command("tp @s 20.5 101 14.5")
        host("wait-ready")
        port = host("lan", "open", 0, "commands")["port"]
        launch(guest_socket, "guest", "NatGuest", "--server", f"127.0.0.1:{port}")
        for _ in range(60):
            if guest_socket.exists() and guest("state")["screen"] == "playing" and any(
                row["loaded"] for row in host("lan", "status")["guests"] if row["name"] == "NatGuest"):
                break
            time.sleep(0.5)
        else:
            raise AssertionError("Guest never joined")
        command("gamemode creative NatGuest")
        command("tick freeze")
        command("setblock 20 100 14 minecraft:gold_block")
        species = {"bass": "naturalist:bass", "great_white_shark": "naturalist:great_white_shark",
                   "whale": "naturalist:whale", "jellyfish": "naturalist:jellyfish", "cod": "minecraft:cod"}
        for index, mob in enumerate(species.values()):
            rows = command(f"summon {mob} 20.5 101 {10.5 + index * 4} {{NoAI:1b,NoGravity:1b}}")
            assert any("Summoned" in row for row in rows), rows
        output = {}
        for distance in (32, 72, 100, 136, 32):
            command(f"tp NatGuest {20.5 + distance} 101 14.5")
            for _ in range(20):
                guest("wait", 4)
                at = guest("state")["player"]["position"]
                block = guest("block", 20, 100, 14)
                if abs(at[0] - (20.5 + distance)) < .001 and "minecraft:gold_block" in str(block):
                    break
            else:
                raise AssertionError(("origin chunk not observed", distance, at, block))
            guest("wait", 8)
            output.setdefault(str(distance), []).append({
                name: len(guest("entities", 150, mob)["entities"]) == 1 for name, mob in species.items()
            })
        print(json.dumps(output, indent=2), flush=True)
    finally:
        if guest_socket.exists():
            guest("quit")
        if host_socket.exists():
            host("quit")
