"""Compare the pinned Java and Horizon passenger outcome when allied birds overlap their owner."""
import argparse
import json
import subprocess
import tempfile
from pathlib import Path


def capture(version, fixture):
    with tempfile.TemporaryDirectory(prefix="nat-bird-perch-", ignore_cleanup_errors=True) as temp:
        directory = Path(temp)
        socket = directory / "bird.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def value(target, field):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {target} {field}")
                         if "following entity data: " in row), None)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Bird Perch Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 109 15 minecraft:air")
            cmd("tp @s 8.5 100 8.5")
            cmd("tick freeze")
            owner = value("@s", "UUID")
            assert owner and owner.startswith("[I;"), owner
            birds = []
            for name, x in (("first", 8.6), ("second", 8.4)):
                cmd(f'summon naturalist:bird {x} 100 8.5 {{Tags:[{name}],Owner:{owner},'
                    'Sitting:0b,NoAI:0b,PersistenceRequired:1b}')
                target = f"@e[tag={name},limit=1]"
                assert value(target, "Owner") == owner, (target, value(target, "Owner"))
                birds.append(target)
            cmd("gamemode survival @s")
            cmd("tick step 10")
            ctl("wait", 12)
            player_passengers = value("@s", "Passengers")
            output = {"ordinary_mount_blocked": not player_passengers,
                      "both_birds_remain_in_world": all(value(bird, "Pos") for bird in birds)}
            print(json.dumps(output, indent=2), flush=True)
            assert output["ordinary_mount_blocked"] == fixture["ordinary_mount_blocked"], output
            assert output["both_birds_remain_in_world"], output
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
