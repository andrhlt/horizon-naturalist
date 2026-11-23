"""Compare authoritative Firefly age and client collision size at a growth boundary."""
import argparse
import json
from pathlib import Path
from launcher_setup import prepare_home
import subprocess
import tempfile


def capture(version, jar):
    with tempfile.TemporaryDirectory(prefix="nat-firefly-age-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "firefly-age.sock"
        prepare_home(directory, jar)

        def call(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        def command(line):
            return call("command", line)["chat"]

        def state(age):
            mobs = call("entities", 8, "naturalist:firefly")["entities"]
            assert len(mobs) == 1, (age, mobs, call("status").get("player"))
            return {"age": age, "baby": mobs[0]["baby"]}

        try:
            call("launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", directory,
                 "--game-dir", directory, "--world", "Firefly Age Fixture", "--creative",
                 "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("forceload add 0 0")
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("tp @s 8.5 101 8.5")
            command("time set midnight")
            call("wait", 20)
            command("tick freeze")
            rows = command("summon naturalist:firefly 8.5 101 11.5 "
                           "{Age:-2,NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[firefly_age]}")
            assert any("Summoned" in row for row in rows), rows
            call("wait", 6)
            juvenile = state(-2)
            command("tick step 1")
            call("wait", 4)
            midpoint = state(-1)
            command("tick step 1")
            call("wait", 4)
            adult = state(0)
            return {"juvenile": juvenile, "near_adult": midpoint, "adult": adult}
        finally:
            if socket.exists():
                call("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--jar", type=Path, required=True)
    args = parser.parse_args()
    fixture = json.loads(args.fixture.read_text())
    observed = capture(args.version, args.jar)
    print(json.dumps(observed, indent=2), flush=True)
    for stage, age in (("juvenile", -2), ("near_adult", -1), ("adult", 0)):
        actual = observed[stage]
        expected = fixture["juvenile" if age < 0 else "adult"]
        assert actual["age"] == age, (stage, actual, age)
        assert actual["baby"] == expected["baby"], (stage, actual, expected)
