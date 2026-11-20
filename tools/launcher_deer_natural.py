"""Observe naturally spawned Deer and their saved fields in the installed build."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time

def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-scoped-deer-", ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        socket = home / "deer.sock"

        def ctl(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  capture_output=True, check=True, text=True, timeout=65)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist Scoped Deer Spawn", "--creative",
                "--test-terrain", "--render-distance", 8, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:spawn_monsters false")
            command("gamerule minecraft:advance_time false")
            command("time set noon")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 80)
            for x in range(-48, 64, 16):
                for z in range(-48, 64, 16):
                    command(f"fillbiome {x} 48 {z} {x + 15} 109 {z + 15} minecraft:forest")
                    command(f"fill {x} 99 {z} {x + 15} 99 {z + 15} minecraft:grass_block")
            command("gamerule minecraft:mob_drops false")
            command("difficulty peaceful")
            command("gamemode survival @s")
            command("gamerule minecraft:spawn_mobs true")
            print("forest control", command("execute if biome 32 100 8 minecraft:forest run say forest-confirmed"), flush=True)

            def ticks():
                return next(int(row.split("The game time is ", 1)[1].split()[0])
                            for row in command("time query gametime") if "The game time is" in row)

            for trial in range(60):
                command("kill @e[type=!minecraft:player]")
                goal = ticks() + 800
                command("tick sprint 800")
                deadline = time.monotonic() + 50
                while ticks() < goal:
                    assert time.monotonic() < deadline, "Natural spawn sprint timed out"
                    time.sleep(0.1)
                ctl("wait", 3)
                deer = ctl("entities", 128, "naturalist:deer")["entities"]
                if deer:
                    break
            else:
                raise AssertionError("A naturally spawned Deer did not enter the fixture after 60 sprints")

            selector = "@e[type=naturalist:deer,limit=1]"
            field = " following entity data: "
            def read(key):
                lines = command(f"data get entity {selector} {key}")
                return next(line.partition(field)[2] for line in lines if field in line)
            assert read("Variant") == '"naturalist:deer"'
            assert read("LeftHanded") in ("0b", "1b")
            assert int(read("Age")) <= 0
            assert "minecraft:random_spawn_bonus" in read("attributes")
            print(json.dumps({"observed_deer": len(deer), "sprints": trial + 1,
                              "variant": read("Variant"), "age": read("Age"),
                              "left_handed": read("LeftHanded"), "attributes": read("attributes")}))
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
