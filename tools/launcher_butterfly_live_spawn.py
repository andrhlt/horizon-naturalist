"""Check natural nighttime Firefly spawning in an open swamp in the headless Horizon launcher build."""
import argparse
from collections import Counter
import json
import re
import time
import os
import subprocess
import tempfile
from pathlib import Path


def capture(version):
    os.environ["NATURALIST_FIXTURE_TRACE_SPAWNS"] = "1"
    with tempfile.TemporaryDirectory(prefix="nat-bird-flap-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "bird-flap.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def game_time():
            [line] = command("time query gametime")
            return int(re.findall(r"\d+", line)[-1])

        def sprint(ticks):
            starting_tick = game_time()
            command(f"tick sprint {ticks}")
            deadline = time.monotonic() + 45.0
            while game_time() - starting_tick < ticks:
                assert time.monotonic() < deadline, "natural spawn tick sprint did not finish"
                time.sleep(0.15)
            ctl("wait", 3)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Ambient Spawn Fixture", "--creative", "--test-terrain", "--render-distance", 8, "--wait")
            def report():
                return {name: ctl("entities", 90, f"naturalist:{name}")["entities"]
                        for name in ("butterfly", "dragonfly", "firefly")}

            command("gamerule minecraft:spawn_mobs false")
            command("difficulty peaceful")
            command("gamerule minecraft:advance_time false")
            command("gamerule minecraft:spawn_wandering_traders false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 80)
            for x in range(-64, 80, 16):
                for z in range(-64, 80, 16):
                    command(f"fillbiome {x} 48 {z} {x + 15} 110 {z + 15} minecraft:swamp")
                    command(f"fill {x} 99 {z} {x + 15} 99 {z + 15} minecraft:grass_block")
            command("time set noon")
            command("gamerule minecraft:spawn_mobs true")
            ctl("wait", 25)
            phases = {}
            for phase in ("noon", "midnight"):
                command(f"time set {phase}")
                command("kill @e[type=naturalist:butterfly]")
                command("kill @e[type=naturalist:dragonfly]")
                command("kill @e[type=naturalist:firefly]")
                sprint(800)
                counts = {}
                for name, rows in report().items():
                    counts[name] = len([row for row in rows if row["health"] > 0])
                phases[phase] = counts
            trace_path = socket.with_suffix(".log")
            if not trace_path.exists():
                trace_path = next(directory.glob("*.log"))
            prefix = "naturalist_butterfly_live "
            events = [json.loads(row.split(prefix, 1)[1]) for row in trace_path.read_text().splitlines()
                      if prefix in row]
            shared = []
            all_adult_and_in_pool = True
            for event in events:
                pairs = Counter(tuple(row["pair"]) for row in event["rows"])
                shared.extend({"tick": event["tick"], "pair": pair, "members": count}
                              for pair, count in pairs.items() if count >= 2)
                all_adult_and_in_pool &= all(row["adult"] and row["selected"] in row["pair"] for row in event["rows"])
            print(json.dumps({"living": phases, "natural_spawn_passes": len(events),
                              "shared_palette_samples": shared, "all_adult_and_in_pool": all_adult_and_in_pool},
                             indent=2), flush=True)
            assert shared and all_adult_and_in_pool, (events, phases)
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
