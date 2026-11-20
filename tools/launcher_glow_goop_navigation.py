"""Check that a tempted cow can path through a full dry Goop cell."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, content):
    with tempfile.TemporaryDirectory(prefix="nat-goop-path-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "path.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(command):
            return ctl("command", command)["chat"]

        try:
            launch = ["launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Glow Goop pathing", "--creative", "--test-terrain", "--render-distance", 4, "--wait"]
            if content is not None:
                launch.extend(("--dev-mod", content))
            ctl(*launch)
            loaded = {}
            for path in (root / ".horizon-cache").rglob("mob-data.json"):
                facts = path.with_name("facts.json")
                if not facts.exists():
                    continue
                exported = json.loads(path.read_text())["states"]
                cooked = json.loads(facts.read_text())["states"]
                for index, row in enumerate(cooked):
                    if row[0] == "naturalist:glow_goop":
                        properties = row[4]["properties"]
                        loaded[f"{properties['goop']}:{properties['waterlogged']}"] = exported[index]
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 100 7.5")
            ctl("wait", 20)
            cmd("fill 6 99 5 10 99 14 minecraft:stone")
            cmd("fill 6 100 5 10 105 14 minecraft:air")
            cmd("fill 7 100 5 7 103 14 minecraft:barrier")
            cmd("fill 9 100 5 9 103 14 minecraft:barrier")
            cmd("fill 7 100 5 9 103 5 minecraft:barrier")
            cmd("fill 7 100 14 9 103 14 minecraft:barrier")
            cmd("fill 7 102 5 9 102 14 minecraft:barrier")

            cmd("tp @s 8.5 100 7.5")
            cmd("gamemode survival @s")
            cmd("item replace entity @s weapon.mainhand with minecraft:wheat")
            cmd("fill 8 100 10 8 101 10 naturalist:glow_goop[goop=3]")
            spawn = cmd("summon minecraft:cow 8.5 100 12.5 {Tags:[goop_path_control]}")
            assert any("Summoned" in row for row in spawn), spawn
            positions = []
            for _ in range(16):
                ctl("wait", 10)
                cows = ctl("entities", 12, "minecraft:cow")["entities"]
                if cows:
                    positions.append(cows[0]["pos"])
                    if cows[0]["pos"][2] < 10.0:
                        break
            return {"crossed": any(pos[2] < 10.0 for pos in positions), "positions": positions,
                    "loaded_rows": loaded}
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--content", type=Path)
    parser.add_argument("--blocked", action="store_true")
    args = parser.parse_args()
    result = capture(args.version, args.content)
    print(json.dumps(result, indent=2), flush=True)
    assert result["crossed"] != args.blocked, result
    if not args.blocked:
        expected = {f"{n}:{wet}": value for n in range(1, 4)
                    for wet, value in (("false", ["OPEN", 135, 0.0]), ("true", ["WATER", 7, 0.0]))}
        assert result["loaded_rows"] == expected, result["loaded_rows"]
