"""Delay a Duck's baby metadata packet while a matching LAN client uses a bucket."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import socket
import subprocess
import tempfile
import threading
import time
import zlib

from prepare import JAR_SHA


def varint(data, offset=0):
    value = shift = 0
    while True:
        byte = data[offset]
        offset += 1
        value |= (byte & 0x7f) << shift
        if byte < 0x80:
            return value, offset
        shift += 7


class MetadataDelay:
    def __init__(self, upstream, delay_seconds=1.5):
        self.upstream = upstream
        self.delay_seconds = delay_seconds
        self.listener = socket.socket()
        self.listener.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.listener.bind(("127.0.0.1", 0))
        self.listener.listen(1)
        self.port = self.listener.getsockname()[1]
        self.armed = threading.Event()
        self.delayed = threading.Event()
        self.forwarded = threading.Event()
        self.errors = []
        self.observed = []
        self.delayed_entity_id = None
        self.thread = threading.Thread(target=self._accept, daemon=True)
        self.thread.start()

    def _accept(self):
        try:
            client, _ = self.listener.accept()
            server = socket.create_connection(self.upstream)
            threading.Thread(target=self._pump, args=(client, server, False), daemon=True).start()
            self._pump(server, client, True)
        except Exception as exc:
            self.errors.append(repr(exc))

    def _pump(self, source, target, server_to_client):
        pending = bytearray()
        try:
            while True:
                chunk = source.recv(65536)
                if not chunk:
                    break
                if not server_to_client:
                    target.sendall(chunk)
                    continue
                pending.extend(chunk)
                while pending:
                    length = offset = 0
                    shift = 0
                    while True:
                        if offset >= len(pending):
                            break
                        byte = pending[offset]
                        offset += 1
                        length |= (byte & 0x7f) << shift
                        if byte < 0x80:
                            break
                        shift += 7
                    else:
                        pass
                    if offset == 0 or offset >= len(pending) and pending[offset - 1] >= 0x80:
                        break
                    if len(pending) < offset + length:
                        break
                    frame = bytes(pending[:offset + length])
                    first, payload_offset = varint(frame, offset)
                    if first == 0:
                        payload = frame[payload_offset:]
                    else:
                        try:
                            payload = zlib.decompress(frame[payload_offset:])
                        except zlib.error:
                            # Login/configuration frames before compression has
                            # been negotiated have no uncompressed-length prefix.
                            payload = frame[offset:]
                    packet_id, packet_offset = varint(payload)
                    del pending[:offset + length]
                    if self.armed.is_set() and len(self.observed) < 100:
                        self.observed.append(packet_id)
                    if packet_id == 101 and self.armed.is_set() and not self.delayed.is_set():
                        self.delayed_entity_id, _ = varint(payload, packet_offset)
                        self.delayed.set()
                        time.sleep(self.delay_seconds)
                        target.sendall(frame)
                        self.forwarded.set()
                    else:
                        target.sendall(frame)
        except Exception as exc:
            self.errors.append(repr(exc))
        finally:
            for stream in (source, target):
                try:
                    stream.shutdown(socket.SHUT_RDWR)
                except OSError:
                    pass
                stream.close()

    def close(self):
        self.listener.close()


def capture(version: Path, jar: Path):
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
        raise ValueError("expected the pinned Naturalist 2.0.7 NeoForge 26.3 jar")
    with tempfile.TemporaryDirectory(prefix="nat-duck-age-delay-", ignore_cleanup_errors=True) as temp:
        root = Path(temp)
        print(f"evidence_root={root}", flush=True)

        def ctl(side, *args):
            home = root / side
            proc = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                cwd=root,
                env={**os.environ, "HORIZON_HOME": str(home), "HORIZON_NO_AUDIO": "1"},
                capture_output=True,
                text=True,
                timeout=85,
            )
            assert proc.returncode == 0, (args, proc.stdout, proc.stderr)
            return json.loads(proc.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("host", "command", line)
            assert result["ok"], (line, result)
            return result["chat"]

        def data(target, key):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {target} {key}")
                         if "following entity data: " in row), None)

        def launch(side, *args):
            home = root / side
            (home / "jars").mkdir(parents=True, exist_ok=True)
            shutil.copy2(jar, home / "jars" / f"{JAR_SHA}.jar")
            (home / "horizon-mods.txt").write_text("mod.naturalist:true\n")
            ctl(side, "launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--render-distance", 4, *args)

        proxy = None
        try:
            launch("host", "--world", "Duck delayed age sync", "--test-terrain", "--creative", "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("kill @e[type=naturalist:duck]")
            cmd("tp @s 7.5 100 6.5")
            ctl("host", "wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 108 15 minecraft:air")
            assert not ctl("host", "entities", 32, "naturalist:duck")["entities"]
            port = ctl("host", "lan", "open", 0, "commands")["port"]
            proxy = MetadataDelay(("127.0.0.1", port))
            launch("guest")
            ctl("guest", "connect", f"127.0.0.1:{proxy.port}", "DuckDelayGuest")
            ctl("guest", "wait-for", "loaded", "--timeout", 55)
            cmd("tp DuckDelayGuest 7.5 100 6.5")
            cmd("tp @s 12.5 100 6.5")
            cmd("gamemode survival DuckDelayGuest")
            # Let the matching client's capability announcement reach the host;
            # the intended race is age metadata, not registration timing.
            ctl("host", "wait", 25)
            cmd("item replace entity DuckDelayGuest weapon.mainhand with minecraft:bucket")
            owner = data("DuckDelayGuest", "UUID")
            assert owner
            proxy.armed.set()
            cmd(f'summon naturalist:duck 7.5 100 8.5 {{Tags:[delayed_duck],Owner:{owner},NoAI:1b,NoGravity:1b,Age:-12000,Health:1.5f}}')
            target = "@e[tag=delayed_duck,limit=1]"
            before = data(target, "UUID")
            assert before and int(data(target, "Age")) < 0
            deadline = time.monotonic() + 8
            ducks = []
            while not ducks and time.monotonic() < deadline:
                ducks = ctl("guest", "entities", 8, "naturalist:duck")["entities"]
            assert len(ducks) == 1, ducks
            assert proxy.delayed.wait(1), ("SetEntityData packet was not delayed", proxy.observed, proxy.errors)
            # The entity spawn arrived, but its baby flag is still buffered by the proxy.
            assert not ducks[0]["baby"], ducks[0]
            assert proxy.delayed_entity_id == ducks[0]["id"], (proxy.delayed_entity_id, ducks[0])
            ctl("guest", "look-at", "entity", ducks[0]["id"])
            looking = ctl("guest", "looking-at")
            ctl("guest", "mouse", "right", "click")
            ctl("host", "wait", 2)
            item = 'Inventory[{id:"naturalist:duck_bucket"}].components."minecraft:bucket_entity_data"'
            age = data("DuckDelayGuest", item + ".Age")
            assert age is not None and int(age) < 0, {
                "looking_at": looking,
                "duck_list": ctl("host", "entities", 8, "naturalist:duck"),
                "server_age": data(target, "Age"),
                "age": age,
                "player_inventory": data("DuckDelayGuest", "Inventory"),
                "hand": ctl("guest", "inventory"),
                "duck_uuid": data(target, "UUID"),
                "metadata_forwarded": proxy.forwarded.is_set(),
            }
            assert data(target, "UUID") is None
            assert not proxy.forwarded.is_set(), "delayed metadata arrived before the server handled the click"
            print(json.dumps({
                "matching_artifact_lan": True,
                "server_age": -12000,
                "client_saw_baby_before_click": False,
                "server_to_client_set_entity_data_delayed_ms": 1500,
                "captured_age": int(age),
                "captured_once_before_metadata_sync": True,
            }), flush=True)
        finally:
            if proxy:
                proxy.close()
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    ctl(side, "quit")
            evidence = Path("/private/tmp") / f"{root.name}-logs"
            evidence.mkdir(exist_ok=True)
            for log in root.rglob("*.log"):
                shutil.copy2(log, evidence / f"{log.parent.name}-{log.name}")
            print(f"saved_logs={evidence}", flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("jar", type=Path)
    args = parser.parse_args()
    capture(args.version, args.jar)
