"""Check actual AI-driven Hedgehog breeding and offspring ownership headlessly."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-hedgehog-breed-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "hedgehog.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def field(entity, key):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in cmd(f"data get entity {entity} {key}")
                         if marker in row), None)

        def babies():
            return [entity for entity in ctl("entities", 15, "naturalist:hedgehog")["entities"]
                    if entity["health"] > 0 and entity["baby"]]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Hedgehog Breeding", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 108 15 minecraft:air")
            cmd("tp @s 8.5 100 6.5")
            cmd("forceload add 0 0")
            owner = field("@s", "UUID")
            cmd("tick freeze")

            for x, name, variant in ((7.5, "left_parent", "brown"), (9.5, "right_parent", "dark")):
                parent = f"@e[tag={name},limit=1]"
                reply = cmd(f'summon naturalist:hedgehog {x} 100 8.5 '
                            f'{{Tags:[{name}],Owner:{owner},Age:0,InLove:600,DyeColor:10,Variant:"naturalist:{variant}"}}')
                assert any("Summoned" in row for row in reply), reply
                assert field(parent, "Owner") == owner
                assert field(parent, "DyeColor") == "10"
                assert field(parent, "InLove") == "600"
            cmd("tick step 110")
            ctl("wait", 112)
            born = babies()
            print(json.dumps({"babies": born}), flush=True)
            assert len(born) == 1, born

            child = "@e[type=naturalist:hedgehog,tag=!left_parent,tag=!right_parent,limit=1]"
            got = {key: field(child, key) for key in ("Age", "Variant", "DyeColor", "Owner")}
            print(json.dumps({"offspring": got}), flush=True)
            return {
                "can_mate": len(born) == 1,
                "offspring_owner_and_dye": got["Owner"] == owner and got["DyeColor"] == "10",
                "offspring_variant_from_parent": got["Variant"] in ('"naturalist:brown"', '"naturalist:dark"'),
            }
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    actual = capture(args.version)
    print(json.dumps(actual, indent=2), flush=True)
    expect = json.loads(args.fixture.read_text())
    assert all(value == expect.get(key, True) for key, value in actual.items()), (actual, expect)
