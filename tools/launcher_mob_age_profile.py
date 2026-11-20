"""Compare the installed Naturalist adult and juvenile body dimensions and baby state."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

from launcher_mob_profile import differences


def capture(version, reference):
    registry = version / "native-content/naturalist/data/naturalist/horizon/entity_type"
    kinds = sorted(f"naturalist:{definition.stem}" for definition in registry.glob("*.json")
                   if definition.stem not in ("duck_egg", "dirt_trail"))
    assert kinds and set(kinds) <= reference.keys(), (kinds, reference.keys())
    with tempfile.TemporaryDirectory(prefix="nat-mob-age-profiles-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "profile.sock"

        def call(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  capture_output=True, text=True, check=True, timeout=65)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return call("command", line)["chat"]

        try:
            call("launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--world", "Naturalist Mob Age Profiles",
                 "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("forceload add 0 0")
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("tp @s 8.5 101 6.5")
            call("wait", 15)
            for kind in kinds:
                command(f"kill @e[type={kind}]")
            call("wait", 25)
            command("tick freeze")
            output = {}
            marker = "Naturalist mob age profile "
            for kind in kinds:
                output[kind] = {}
                for stage, age in (("adult", 0), ("juvenile", -24000)):
                    rows = command(f"summon {kind} 8.5 101 8.5 "
                                   f"{{Age:{age},NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[age_profile]}}")
                    assert any("Summoned" in row for row in rows), (kind, stage, rows)
                    rows = command(f"naturalist_fixture_mob_age_profile {kind}")
                    profile = next(json.loads(row.split(marker, 1)[1])
                                   for row in rows if marker + "{" in row)
                    alive = [row for row in call("entities", 40, kind)["entities"] if row["health"] > 0]
                    assert len(alive) == 1, (kind, stage, alive)
                    output[kind][stage] = {**profile, "client_baby": alive[0]["baby"]}
                    command("kill @e[tag=age_profile]")
            return output
        finally:
            if socket.exists():
                call("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    reference = json.loads(args.fixture.read_text())
    actual = capture(args.version, reference)
    expected = {
        kind: {stage: {**profile, "client_baby": profile["baby"]}
               for stage, profile in reference[kind].items()}
        for kind in actual
    }
    changes = list(differences(expected, actual))
    print(json.dumps({"entities": len(actual), "discrepancies": changes, "profiles": actual}, indent=2), flush=True)
    assert not changes, f"{len(changes)} deltas across {len(actual)} entities"
