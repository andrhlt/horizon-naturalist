"""Measure physical player contact with Shark and Whale detached hitboxes in LAN."""
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import struct
import zipfile

from horizon_replay_particles import studio, varint

version = Path(sys.argv[1])
ctl = version / "horizon-ctl"
game = version / "horizon-client"
resources = version / ".local/horizon-client/26.3"
reference = json.loads(Path(sys.argv[2]).read_text())
selected = sys.argv[sys.argv.index("--species") + 1] if "--species" in sys.argv[3:] else None


def call(socket, *args):
    result = subprocess.run(
        [ctl, "--socket", socket, *map(str, args)], capture_output=True, check=True, text=True, timeout=60,
    )
    return json.loads(result.stdout.splitlines()[-1])


def quantize(velocity):
    scale = math.ceil(max(map(abs, velocity)))
    if scale == 0:
        return [0.0] * 3
    return [
        (math.floor((value / scale * 0.5 + 0.5) * 32766.0 + 0.5) * 2.0 / 32766.0 - 1.0) * scale
        for value in velocity
    ]


def player_motion(replay, entity_id):
    with zipfile.ZipFile(replay) as archive:
        packets = archive.read("recording.tmcpr")
    start = 0
    values = []
    while start < len(packets):
        _, length = struct.unpack_from(">II", packets, start)
        start += 8
        end = start + length
        packet_id, body = varint(packets, start)
        start = end
        if packet_id != 103:
            continue
        named, body = varint(packets, body)
        if named != entity_id:
            continue
        lowest = packets[body]
        if lowest == 0:
            values.append([0.0] * 3)
            continue
        middle = packets[body + 1]
        highest = struct.unpack_from(">I", packets, body + 2)[0]
        buffer = highest << 16 | middle << 8 | lowest
        scale = lowest & 3
        body += 6
        if lowest & 4:
            more, body = varint(packets, body)
            scale |= more << 2
        assert body == end, (body, end)
        values.append([(min((buffer >> shift) & 32767, 32766) * 2.0 / 32766.0 - 1.0) * scale
                       for shift in (3, 18, 33)])
    return values

with tempfile.TemporaryDirectory(prefix="nat-lan-body-push-", ignore_cleanup_errors=True) as tmp:
    root = Path(tmp)
    h, g = root / "host.sock", root / "guest.sock"
    host = lambda *args: call(h, *args)
    guest = lambda *args: call(g, *args)
    run = lambda line: host("command", line)["chat"]
    try:
        home, guest_home = root / "host", root / "guest"
        home.mkdir()
        guest_home.mkdir()
        host("launch", "--headless", "--game", game, "--resources", resources, "--home", home,
             "--game-dir", home, "--name", "NatHost", "--world", "Multipart body LAN", "--creative",
             "--test-terrain", "--render-distance", 4, "--wait")
        run("gamerule minecraft:spawn_mobs false")
        run("tp @s 8.5 92 6.5")
        host("wait-ready")
        status = host("lan", "open", 0, "commands")
        guest("launch", "--headless", "--game", game, "--resources", resources, "--home", guest_home,
             "--game-dir", guest_home, "--name", "NatGuest", "--server", f"127.0.0.1:{status['port']}",
             "--render-distance", 4)
        for _ in range(60):
            if g.exists() and guest("state")["screen"] == "playing" and any(
                actor["name"] == "NatGuest" and actor["loaded"] for actor in host("lan", "status")["guests"]
            ):
                break
            time.sleep(0.5)
        else:
            raise RuntimeError("LAN guest did not finish joining")
        run("tick freeze")
        run("fill 2 91 2 22 91 22 minecraft:stone")
        run("fill 2 92 2 22 96 22 minecraft:air")
        run("fill 4 92 6 20 96 19 minecraft:water")

        run("gamemode survival NatHost")
        run("gamemode survival NatGuest")
        results = {}
        for species, z in (("great_white_shark", 12.6), ("whale", 14.7)):
            if selected is not None and selected != species:
                continue
            full_id = f"naturalist:{species}"
            for actor in ("NatHost", "NatGuest"):
                run(f"tp {actor} 21.5 92 21.5")
            host("wait", 36)
            guest("wait", 36)
            run(f"tp NatHost 8.15 92 {z}")
            run(f"tp NatGuest 8.85 92 {z}")
            run(f"summon {full_id} 8.5 92 10.5 {{NoAI:1b,NoGravity:1b,Rotation:[0.0f,0.0f],Tags:[multipart_probe]}}")
            host("wait", 4)
            guest("wait", 4)
            before = [client("state")["player"] for client in (host, guest)]
            assert before[0]["position"] == [8.15, 92.0, z], before
            assert before[1]["position"] == [8.85, 92.0, z], before
            studio(g, "start simultaneous_contact")
            run("tick step 1")
            host("wait", 4)
            guest("wait", 6)
            after = [client("state")["player"] for client in (host, guest)]
            guest_id = guest("status")["server"]["entity_id"]
            path = guest_home / studio(g, "stop")["path"]
            packets = player_motion(path, guest_id)
            expected = quantize([a + b for a,b in zip(before[1]["velocity"], reference[species]["delta"], strict=True)])
            assert packets == [expected], (packets, expected)
            assert after[0]["position"][0] < 8.10 and after[1]["position"][0] > 8.90, (before, after)
            results[species] = {"host": after[0]["position"], "guest": after[1]["position"], "guest_packet": packets[0]}
            run(f"kill @e[type={full_id},tag=multipart_probe]")
            run("tick step 25")
            host("wait", 28)
        print(json.dumps(results, sort_keys=True), flush=True)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
