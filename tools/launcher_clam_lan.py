"""Replay the Clam’s shared launch and treasure slot from two players."""
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
reference_path = Path(sys.argv[2]) if len(sys.argv) > 2 else Path(__file__).parent.parent / "fixtures/clam-java.json"
expected_increment = json.loads(reference_path.read_text())["launch_vertical"]
game = version / "horizon-client"
resources = version / ".local/horizon-client/26.3"


def command(socket, *args):
    call = subprocess.run([ctl, "--socket", socket, *map(str, args)], capture_output=True,
                          check=True, text=True, timeout=60)
    return json.loads(call.stdout.splitlines()[-1])

def quantize_vertical(value):
    scale = math.ceil(abs(value))
    packed = math.floor((value / scale * 0.5 + 0.5) * 32766.0 + 0.5)
    return (packed * 2.0 / 32766.0 - 1.0) * scale


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

with tempfile.TemporaryDirectory(prefix="nat-lan-clam-", ignore_cleanup_errors=True) as world:
    root = Path(world)
    h = root / "host.sock"
    g = root / "guest.sock"
    host = lambda *args: command(h, *args)
    guest = lambda *args: command(g, *args)
    try:
        home = root / "host"
        home.mkdir()
        host("launch", "--headless", "--game", game, "--resources", resources, "--home", home,
             "--game-dir", home, "--name", "NatHost", "--world", "Naturalist Clam LAN", "--creative",
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
            if g.exists() and guest("state")["screen"] == "playing" and any(
                player["name"] == "NatGuest" and player["loaded"]
                for player in host("lan", "status")["guests"]
            ):
                break
            time.sleep(0.5)
        else:
            raise RuntimeError("Guest did not join")
        print("Players", host("lan", "status"), flush=True)
        run("tick freeze")
        run("fill 4 91 3 16 91 16 minecraft:stone")
        run("fill 4 92 3 16 96 16 minecraft:air")
        run("gamemode survival NatGuest")
        run("gamemode survival NatHost")
        run("tp NatHost 8.0 92 7.4")
        run("tp NatGuest 9.0 92 7.4")
        run('summon naturalist:clam 8.5 92 8.5 {NoGravity:1b,PersistenceRequired:1b,Tags:[shared_shell]}')
        shell = "@e[type=naturalist:clam,tag=shared_shell,limit=1]"

        def step(ticks):
            run(f"tick step {ticks}")
            host("wait", ticks + 3)
            guest("wait", 3)

        def read(field):
            rows = run(f"data get entity {shell} {field}")
            return next((row.split("following entity data: ", 1)[1] for row in rows if "following entity data: " in row), None)

        def click(client):
            [entity] = client("entities", 5, "naturalist:clam")["entities"]
            looked = client("look-at-entity", entity["id"])
            assert looked["target"]["kind"] == "entity", looked
            client("mouse", "right", "click")
            client("wait", 4)

        step(1)
        assert read("Open") == "1b"
        guest_id = guest("status")["server"]["entity_id"]
        studio(g, "start clam_before_launch")
        step(3)
        before = [client("state")["player"] for client in (host, guest)]
        assert [actor["position"][1] for actor in before] == [92.0, 92.0], before
        quiet = player_motion(guest_home / studio(g, "stop")["path"], guest_id)
        assert quiet == [], quiet
        studio(g, "start clam_launch_tick")
        step(1)
        launched = player_motion(guest_home / studio(g, "stop")["path"], guest_id)
        expected_motion = quantize_vertical(before[1]["velocity"][1] + expected_increment)
        assert launched == [[0.0, expected_motion, 0.0]], (launched, expected_motion)
        after_impulse = [client("state")["player"] for client in (host, guest)]
        print(json.dumps({"quiet_packets": quiet, "launch_packet": launched,
                          "before_velocity": [actor["velocity"] for actor in before],
                          "after_velocity": [actor["velocity"] for actor in after_impulse]}), flush=True)
        step(2)
        after = [client("state")["player"]["position"][1] for client in (host, guest)]
        assert all(y > 94 for y in after), after

        run("tp NatHost 8.0 92 7.4")
        run("tp NatGuest 9.0 92 7.4")
        run("item replace entity NatGuest weapon.mainhand with minecraft:ender_pearl 3")
        run("item replace entity NatHost weapon.mainhand with minecraft:air")
        guest("wait", 5)
        click(guest)
        assert read("HasTreasure") == "1b"
        assert 'id: "minecraft:ender_pearl"' in read("equipment.mainhand")
        assert guest("inventory")["counts"].get("minecraft:ender_pearl") == 2
        click(host)
        assert read("HasTreasure") == "0b"
        assert read("Open") == "0b"
        assert host("inventory")["counts"].get("minecraft:ender_pearl") == 1
        assert guest("inventory")["counts"].get("minecraft:ender_pearl") == 2
        print(json.dumps({"host_y_after_launch": after[0], "guest_y_after_launch": after[1],
            "guest_remaining_pearls": 2, "host_withdrawn_pearls": 1, "shell_closed": True}), flush=True)
    finally:
        if g.exists():
            guest("quit")
        if h.exists():
            host("quit")
