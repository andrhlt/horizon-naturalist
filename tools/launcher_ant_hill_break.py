"""Replay survival Ant Hill mining, worker release and dropped storage in Horizon."""
import argparse
import hashlib
import shutil
from prepare import JAR_SHA
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, jar):
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
        raise ValueError("expected the pinned Naturalist 2.0.7 NeoForge 26.3 jar")
    with tempfile.TemporaryDirectory(prefix="nat-ant-hill-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "control.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def path(log):
            return [row.split("following entity data: ", 1)[1] for row in log
                    if "following entity data: " in row]

        try:
            (root / "jars").mkdir()
            shutil.copy2(jar, root / "jars" / f"{JAR_SHA}.jar")
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ant Hill Mining", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 8.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tp @s 8.5 100 6.5")
            cmd("gamemode survival @s")
            owner = path(cmd("data get entity @s UUID"))[0]

            results = []
            for same_owner, block, other in [(True, 8, owner), (False, 12, "[I;0,17,0,31]")]:
                cmd("kill @e[type=naturalist:ant]")
                cmd("kill @e[type=minecraft:item]")
                cmd("clear @s")
                cmd("item replace entity @s weapon.mainhand with minecraft:diamond_shovel")
                cmd(f"tp @s {block + 0.5} 100 6.5")
                cmd(f"setblock {block} 100 8 naturalist:ant_hill[open=true,workers=2,has_queen=false]")
                cmd(f'data merge block {block} 100 8 {{Owner:{other},Storage:[{{id:"minecraft:apple",count:2}}]}}')
                mined = ctl("break", block, 100, 8, "--timeout", 7)
                ctl("wait", 4)
                ants = [entity for entity in ctl("entities", 8, "naturalist:ant")["entities"]
                        if entity.get("health", 0) > 0]
                prefix = f"execute positioned {block + 0.5} 100 8.5 as @e[type=naturalist:ant,distance=..4] run data get entity @s"
                owner_refs = path(cmd(prefix + " Owner"))
                anger_refs = path(cmd(prefix + " angry_at"))
                items = ctl("entities", 8, "minecraft:item")["entities"]
                stacks = path(cmd("execute as @e[type=minecraft:item] run data get entity @s Item"))
                print(json.dumps({"mined": mined, "ants": ants, "items": items, "stacks": stacks,
                                  "owner_refs": owner_refs, "anger_refs": anger_refs}), flush=True)
                apples = mined.get("drops", {}).get("minecraft:apple", 0)
                result = {
                    "same_owner": same_owner, "released": len(ants),
                    "tamed": sum(row.replace(" ", "") == other.replace(" ", "") for row in owner_refs),
                    "angry": sum(row.replace(" ", "") == owner.replace(" ", "") for row in anger_refs),
                    "apples": apples,
                }
                results.append(result)
            return results
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("jar", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())["hill_breaks"]
    observed = capture(args.version, args.jar)
    print(json.dumps(observed, indent=2))
    assert observed == expected, (observed, expected)
