import argparse
import json
from pathlib import Path
import struct
import subprocess
import tempfile
from zipfile import ZipFile

from firefly_fixed_light_arena import arena
from firefly_grass_arena import stepper
from launcher_firefly_grass import varint

parser = argparse.ArgumentParser()
parser.add_argument("version", type=Path)
parser.add_argument("--reference", type=Path)
args = parser.parse_args()
version = args.version


def packets(path, targets):
    glowing = set()
    with ZipFile(path) as recording, recording.open("recording.tmcpr") as stream:
        while header := stream.read(8):
            _, length = struct.unpack(">II", header)
            packet = stream.read(length)
            packet_id, offset = varint(packet)
            if packet_id != 101:  # 26.3 SetEntityData
                continue
            entity, _ = varint(packet, offset)
            if entity in targets and packet[-4:-2] == b"\xfa\x00" and packet[-2] & 32:
                glowing.add(entity)
    return {group: sum(targets[entity] == group for entity in glowing) for group in ("bright", "dark")}


with tempfile.TemporaryDirectory(prefix="nat-firefly-fixed-", ignore_cleanup_errors=True) as work:
    root = Path(work)

    def ctl(side, *args):
        process = subprocess.run(
            [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
            capture_output=True, text=True, timeout=100,
        )
        assert process.returncode == 0, (side, args, process.stderr, process.stdout)
        return json.loads(process.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("host", "command", line)["chat"]

    def launch(side, *args):
        home = root / side
        home.mkdir()
        ctl(side, "launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--render-distance", 4, *args)

    try:
        launch("host", "--world", "Firefly fixed light", "--creative", "--test-terrain", "--wait")
        port = ctl("host", "lan", "open", 0, "commands")["port"]
        launch("guest")
        ctl("guest", "connect", f"127.0.0.1:{port}", "FixedWitness")
        ctl("guest", "wait-for", "loaded", "--timeout", 55)
        cmd("execute in minecraft:the_nether run tp @s 15.5 106 7.5")
        ctl("host", "wait", 60)
        cmd("gamemode spectator FixedWitness")
        cmd("execute in minecraft:the_nether run tp FixedWitness 15.5 106 7.5")
        ctl("guest", "wait", 45)
        step = stepper(cmd)
        arena(cmd, step)
        start = ctl("guest", "studio", "record", "start", "firefly_fixed")
        assert start["ok"], start
        for _ in range(3):
            step(50)
        ctl("guest", "wait", 8)
        files = ctl("guest", "studio", "record", "stop")
        assert files["ok"], files
        targets = {
            entity["id"]: "bright" if entity["pos"][0] < 14 else "dark"
            for entity in ctl("guest", "entities", 42)["entities"]
            if entity["type"] == "naturalist:firefly"
        }
        assert sorted(targets.values()) == ["bright"] * 16 + ["dark"] * 16, targets
        result = packets(root / "guest" / files["path"], targets)
        print(json.dumps(result, indent=2))
        if args.reference:
            java = json.loads(args.reference.read_text())
            assert java and all(row["fixed"] and not row["dark_outside"] for row in java), java
            assert all(row["bright_raw"] == 14 and row["dark_raw"] == 0
                       and row["total"] == {"bright": 16, "dark": 16} for row in java), java
            assert all(not row["glowing"].get("bright", 0) for row in java), java
            assert any(row["glowing"].get("dark", 0) for row in java), java
            assert result["bright"] == 0 and result["dark"] > 0, result
    finally:
        for side in ("guest", "host"):
            if (root / f"{side}.sock").exists():
                ctl(side, "quit")
