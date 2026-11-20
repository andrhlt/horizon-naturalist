"""Record actual nearby LAN packet batches during a Whale or Shark beach flop."""
import json
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import time
from zipfile import ZipFile
from horizon_replay_particles import studio, varint

version = Path(sys.argv[1])


def motion_packets(path, entity_id):
    with ZipFile(path) as archive:
        data = archive.read("recording.tmcpr")
    at = 0
    while at < len(data):
        _, length = struct.unpack_from(">II", data, at)
        packet, body = varint(data, at + 8)
        at += 8 + length
        if packet != 103:
            continue
        ident, body = varint(data, body)
        if ident != entity_id:
            continue
        low = data[body]
        if low == 0:
            yield [0.0] * 3
            continue
        word = low | (data[body + 1] << 8) | (struct.unpack_from(">I", data, body + 2)[0] << 16)
        scale = low & 3
        if low & 4:
            high, _ = varint(data, body + 6)
            scale |= high << 2
        yield [(min((word >> shift) & 32767, 32766) * 2 / 32766 - 1) * scale for shift in (3, 18, 33)]


def call(socket, *args):
    response = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=60)
    return json.loads(response.stdout.splitlines()[-1])


with tempfile.TemporaryDirectory(prefix="nat-flop-lan-", ignore_cleanup_errors=True) as tmp:
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
                        "--game-dir", folder, "--name", player, "--render-distance", 4, *options)

        launch(host_socket, "host", "NatHost", "--world", "Naturalist shoreline LAN", "--creative", "--test-terrain", "--wait")
        command("gamerule minecraft:spawn_mobs false")
        command("tp @s 16.5 98 14.5")
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
        command("tp NatGuest 16.5 98 14.5")
        command("tick freeze")
        command("fill 3 91 3 30 91 26 minecraft:stone")
        command("fill 3 92 3 30 102 26 minecraft:air")
        command("fill 23 92 3 23 92 26 minecraft:glass")
        command("fill 24 92 3 30 92 26 minecraft:water")
        for species, z in (("whale", 9.5), ("great_white_shark", 20.5)):
            kind = "naturalist:" + species
            result = command(f"summon {kind} 16.5 92 {z} {{Tags:[flop_probe],Rotation:[0f,0f]}}")
            assert any("summoned" in row.lower() for row in result), result
            host("wait", 8)
            guest("wait", 4)
            tracked = guest("entities", 24, kind)["entities"]
            assert len(tracked) == 1, tracked
            entity_id = tracked[0]["id"]
            rows = []
            for tick in range(1, 7):
                started = studio(guest_socket, f"start {species}_{tick}")
                command("tick step 1")
                host("wait", 3)
                guest("wait", 3)
                stopped = studio(guest_socket, "stop")
                assert started["path"] == stopped["path"]
                path = root / "guest" / stopped["path"]
                updates = list(motion_packets(path, entity_id))
                motion = command("data get entity @e[tag=flop_probe,limit=1] Motion")
                position = command("data get entity @e[tag=flop_probe,limit=1] Pos")
                rows.append({"tick": tick, "motion": motion, "position": position, "received": updates})
            print(species, json.dumps(rows), flush=True)
            command("tag @e[tag=flop_probe] remove flop_probe")
    finally:
        if guest_socket.exists():
            guest("quit")
        if host_socket.exists():
            host("quit")
