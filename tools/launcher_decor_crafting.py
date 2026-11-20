"""Replay Java's three Shellstone crafting recipes via the real LAN inventory."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
recipes = json.loads(Path(sys.argv[2]).read_text())

with tempfile.TemporaryDirectory(prefix="nat-decor-craft-", ignore_cleanup_errors=True) as folder:
    root = Path(folder)
    script = root / "clicks.json"
    guest_env = {**os.environ, "HORIZON_TICK_SCRIPT": str(script)}

    def ctl(side, *args, env=None):
        proc = subprocess.run([version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75, env=env)
        return json.loads(proc.stdout.splitlines()[-1])

    def command(line):
        return ctl("host", "command", line)["chat"]

    def launch(side, *args, env=None):
        home = root / side
        home.mkdir()
        ctl(side, "launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--render-distance", 4, *args, env=env)

    def craft(case, recipe):
        command("clear DecorGuest")
        pattern = "".join(row.ljust(2) for row in recipe["pattern"])
        assert 0 < len(pattern) <= 4, (case, recipe)
        clicks = []
        for slot, key in enumerate(pattern):
            if key == " ":
                continue
            command(f'item replace entity DecorGuest hotbar.{slot} with {recipe["key"][key]}')
            clicks.extend((slot, 100 + slot))
        clicks.extend((200, 0))
        close = 6 + 3 * len(clicks)
        ticks = close + 3
        steps = {3: ["key e tap"], close: ["key e tap"]}
        for tick, slot in zip(range(6, close, 3), clicks, strict=True):
            steps[tick] = [f"click-slot {slot}"]
        script.write_text(json.dumps({"name": case, "ticks": ticks, "steps": steps}))
        done = script.with_suffix(".done")
        done.unlink(missing_ok=True)
        script.with_suffix(".go").touch()
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline and not done.exists():
            time.sleep(.05)
        assert done.exists() and done.read_text() == case, case
        actual = "\n".join(command("data get entity DecorGuest Inventory"))
        count, name = recipe["result"].get("count", 1), recipe["result"]["id"]
        assert f'id: "{name}"' in actual and f"count: {count}" in actual, actual
        print(case, actual, flush=True)

    try:
        launch("host", "--world", "Naturalist Decoration Crafting", "--creative", "--test-terrain", "--wait")
        command("gamerule minecraft:spawn_mobs false")
        port = ctl("host", "lan", "open", 0, "commands")["port"]
        launch("guest", env=guest_env)
        ctl("guest", "connect", f"127.0.0.1:{port}", "DecorGuest")
        ctl("guest", "wait-for", "loaded", "--timeout", 55)
        command("gamemode survival DecorGuest")
        for name, payload in recipes.items():
            craft(name, payload)
    finally:
        for side in ("guest", "host"):
            if (root / f"{side}.sock").exists():
                ctl(side, "quit")
