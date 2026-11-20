"""Exercise the original rare boar record drop with actual headless combat."""
import argparse
import json
import re
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-boar-disc-", ignore_cleanup_errors=True) as path:
        home = Path(path)
        socket = home / "boar.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75)
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        def kill_by_boar():
            cmd("tp @s 8.5 101 7.5")
            cmd("attribute @s minecraft:max_health base set 1")
            ctl("wait", 3)
            cmd("summon naturalist:boar 8.5 101 9.25 {Tags:[disc_fixture]}")
            cmd("damage @e[tag=disc_fixture,limit=1] 1 minecraft:player_attack by @s")
            for _ in range(20):
                ctl("wait", 5)
                state = ctl("state")
                if state.get("screen") in ("death", "death_screen") or state.get("dead") is True:
                    break
                if any(re.search(r"following entity data:\s*0(?:\.0+)?f?\s*$", row)
                       for row in cmd("data get entity @s Health")):
                    break
            else:
                raise AssertionError("Boar did not finish the fixture player")
            discs = cmd('execute if entity @e[type=minecraft:item,nbt={Item:{id:"naturalist:music_disc_death_by_hogs"}}] run say boar-record-found')
            ctl("respawn")
            ctl("wait", 5)
            cmd("kill @e[tag=disc_fixture]")
            cmd("kill @e[type=minecraft:item]")
            return any("boar-record-found" in row for row in discs)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Boar Disc Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:keep_inventory true")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 105 15 minecraft:air")
            cmd("spawnpoint @s 8 101 7")
            cmd("tp @s 8.5 101 7.5")
            ctl("wait", 10)
            cmd("difficulty normal")
            cmd("gamemode survival @s")
            cmd("gamerule minecraft:mob_drops true")
            for attempt in range(1, 81):
                if kill_by_boar():
                    break
            else:
                raise AssertionError("No music disc after 80 verified fatal boar attacks")
            cmd("gamerule minecraft:mob_drops false")
            disabled = sum(kill_by_boar() for _ in range(12))
            return {"first_disc_attempt": attempt, "discs_when_mob_drops_disabled": disabled}
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    reference = json.loads(options.fixture.read_text())["boar_disc"]
    assert 0 < reference["discs"] < reference["fatal_hits"], reference
    result = capture(options.version)
    print(json.dumps(result, indent=2), flush=True)
    assert result["discs_when_mob_drops_disabled"] == reference["discs_when_mob_drops_disabled"], result
