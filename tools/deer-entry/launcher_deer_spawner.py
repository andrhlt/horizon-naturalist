"""Compare real seeded Deer spawner ticks against pinned Java."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
import tempfile


def capture(version, expected, empty_potentials=False):
    with tempfile.TemporaryDirectory(prefix="nat-deer-entry-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "entry.sock"

        def ctl(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=65)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Naturalist Deer Spawns", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 100 5.5")
            ctl("wait", 25)
            command("tick freeze")

            command("gamerule minecraft:spawner_blocks_work true")
            command("fill 0 97 0 15 97 15 minecraft:grass_block")
            command("fill 0 98 0 15 104 15 minecraft:air")
            command("fillbiome 0 96 0 15 104 15 minecraft:forest")
            command("tp @s 8.5 100 15.5")
            probe = "naturalist_fixture_deer_entry"
            prefix = "Naturalist Deer entry "
            output = {}
            for seed in expected:
                output[seed] = {}
                for mode, tag in (("bare", ""), ("explicit", ",NoAI:1b")):
                    command("setblock 8 100 8 air")
                    command("kill @e[type=naturalist:deer]")
                    spec = ("minecraft:spawner{Delay:0s,MinSpawnDelay:200s,MaxSpawnDelay:200s,SpawnCount:1s,"
                            "MaxNearbyEntities:6s,RequiredPlayerRange:16s,SpawnRange:4s,"
                            'SpawnData:{entity:{id:"naturalist:deer"' + tag + "},custom_spawn_rules:{}}}")
                    if empty_potentials:
                        spec = spec.replace("MaxNearbyEntities:", "SpawnPotentials:[],MaxNearbyEntities:")
                    assert any("Changed the block" in line for line in command(f"setblock 8 100 8 {spec}"))
                    assert any("Deer entry seeded" in line for line in command(f"{probe} seed {seed}"))
                    command(probe + " spawner")
                    rows = command(probe + " observe")
                    output[seed][mode] = next((json.loads(line.partition(prefix)[2]) for line in rows if prefix in line), None)
            return output
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--empty-potentials", action="store_true")
    args = parser.parse_args()
    expected = json.loads(args.reference.read_text())
    actual = capture(args.version, expected, args.empty_potentials)
    print(json.dumps(actual, indent=2), flush=True)
    failures = []
    for seed, cases in expected.items():
        for mode, reference in cases.items():
            observed = actual[seed][mode]
            if reference is None or observed is None:
                if reference != observed:
                    failures.append((seed, mode, "spawn", observed, reference))
                continue
            for key, value in reference.items():
                other = observed[key]
                if key == "yaw":
                    match = abs(other - value) <= 1e-4
                else:
                    match = other == value
                if not match:
                    failures.append((seed, mode, key, other, value))
    for failure in failures:
        print(failure, file=sys.stderr)
    assert not failures
