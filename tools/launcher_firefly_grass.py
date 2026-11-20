"""Replay daylight Firefly grass hiding through installed stock Horizon."""
import argparse
import json
from pathlib import Path
import struct
import subprocess
import tempfile
from zipfile import ZipFile

from firefly_grass_arena import capture, stepper


def varint(data, offset=0):
    result = 0
    for shift in range(0, 35, 7):
        byte = data[offset]
        offset += 1
        result |= (byte & 0x7f) << shift
        if byte < 0x80:
            return result, offset
    raise ValueError("Invalid packet VarInt")


def grass_packets(path):
    packets = []
    sounds = []
    with ZipFile(path) as recording, recording.open("recording.tmcpr") as stream:
        while header := stream.read(8):
            _, length = struct.unpack(">II", header)
            packet = stream.read(length)
            packet_id, pos = varint(packet)
            if packet_id == 119:  # 26.3 clientbound Sound
                holder, pos = varint(packet, pos)
                if holder == 0:
                    length, pos = varint(packet, pos)
                    name = packet[pos:pos + length].decode()
                    pos += length
                    if "firefly.hide" in name:
                        pos += 5 if packet[pos] else 1  # inline optional fixed range
                        source, pos = varint(packet, pos)
                        x, y, z, volume, pitch = struct.unpack_from(">3i2f", packet, pos)
                        sounds.append({"id": name, "source": source, "at": (x / 8, y / 8, z / 8),
                                       "volume": volume, "pitch": pitch})
                continue
            if packet_id != 48:  # 26.3 clientbound LevelParticles
                continue
            particle, pos = varint(packet, pos)
            if particle != 1:  # minecraft:block
                continue
            state, pos = varint(packet, pos)
            at_and_velocity = struct.unpack_from(">3d6f", packet, pos + 2)
            count, pos = varint(packet, pos + 2 + 48)
            randomization, _ = varint(packet, pos)
            packets.append({"state": state, "at": at_and_velocity[:3], "spread": at_and_velocity[3:6],
                            "speed": at_and_velocity[6:], "count": count, "randomization": randomization})
    states = {packet["state"] for packet in packets
              if packet["count"] == 3 and packet["at"] == (2.5, 101, 2.5)}
    assert len(states) == 1, ("short grass calibration unavailable", packets)
    return {
        "hiding_packets": [packet for packet in packets if packet["state"] in states and packet["count"] == 50],
        "sound_packets": sounds,
    }


def run(version, lan, *, scene, interval):
    with tempfile.TemporaryDirectory(prefix="nat-firefly-grass-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def ctl(side, *arguments):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, arguments)],
                check=True, capture_output=True, text=True, timeout=70,
            )
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("host", "command", line)["chat"]

        def launch(side, *args):
            home = root / side
            home.mkdir()
            ctl(side, "launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--render-distance", 4, *args)

        def block_particles(side):
            return ctl(side, "status")["particles"].get("block", 0)

        def particles():
            result = {"host": block_particles("host")}
            if lan:
                ctl("guest", "wait", 1)
                result["guest"] = block_particles("guest")
            return result

        try:
            launch("host", "--world", "Firefly grass", "--creative", "--test-terrain", "--wait")
            cmd("tp @s 8.5 104 7.5")
            ctl("host", "wait", 15)
            if lan:
                port = ctl("host", "lan", "open", 0, "commands")["port"]
                launch("guest")
                ctl("guest", "connect", f"127.0.0.1:{port}", "GrassWitness")
                ctl("guest", "wait-for", "loaded", "--timeout", 55)
                cmd("gamemode spectator GrassWitness")
                cmd("tp GrassWitness 8.5 104 7.5")
                ctl("guest", "wait", 10)
                recording = ctl("guest", "studio", "record", "start", "firefly_grass")
                assert recording["ok"], recording
            result = capture(cmd, stepper(cmd), particles, scene=scene, interval=interval)
            if lan:
                rows = cmd('particle minecraft:block{block_state:"minecraft:short_grass"} '
                           '2.5 101 2.5 0 0 0 0.05 3 force GrassWitness')
                assert any("particle" in row for row in rows), rows
                ctl("guest", "wait", 7)
                recording = ctl("guest", "studio", "record", "stop")
                assert recording["ok"], recording
                result.update(grass_packets(root / "guest" / recording["path"]))
            result["hide_sounds"] = {
                side: {key: count for key, count in ctl(side, "status")["heard"].items()
                       if "firefly.hide" in key}
                for side in (("host", "guest") if lan else ("host",))
            }
            return result
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    ctl(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--lan", action="store_true")
    parser.add_argument("--scene", choices=["noon", "canopy"], default="noon")
    parser.add_argument("--interval", type=int, choices=[1, 4], default=4)
    args = parser.parse_args()
    print(json.dumps(run(args.version, args.lan, scene=args.scene, interval=args.interval), indent=2), flush=True)
