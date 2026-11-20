"""Compare seeded Deer summon variants and follow-range fields with Java."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, expected):
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
            command("fill 7 99 7 10 99 10 minecraft:grass_block")
            command("fill 7 100 7 10 104 10 minecraft:air")
            command("fillbiome 4 96 4 12 104 12 minecraft:forest")
            command("tick freeze")

            probe = "naturalist_fixture_deer_entry"
            prefix = "Naturalist Deer entry "
            actions = {"summon": "summon naturalist:deer 8.5 100 8.5",
                       "summon_nbt": "summon naturalist:deer 8.5 100 8.5 {NoAI:1b}",
                       "egg": probe + " egg"}
            result = {}
            for seed in expected:
                result[seed] = {}
                for mode, action in actions.items():
                    command("kill @e[type=naturalist:deer]")
                    assert any("Deer entry seeded" in line for line in command(f"{probe} seed {seed}"))
                    result[seed][mode] = None
                    action_output = command(action)
                    rows = command(probe + " observe")
                    actual = next((json.loads(line.partition(prefix)[2]) for line in rows if prefix in line), None)
                    assert actual is not None, (mode, action_output, rows)
                    result[seed][mode] = actual
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.reference.read_text())
    actual = capture(args.version, expected)
    print(json.dumps(actual, indent=2), flush=True)
    for seed, cases in expected.items():
        for mode, reference in cases.items():
            observed = actual[seed][mode]
            assert abs(reference["yaw"] - observed["yaw"]) <= 1e-4, (seed, mode, observed, reference)
            assert all(observed[key] == reference[key] for key in reference if key != "yaw"), (
                seed, mode, observed, reference)
