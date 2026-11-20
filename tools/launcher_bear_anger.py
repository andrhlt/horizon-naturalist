"""Replay bear anger identity and absolute expiry across a headless world reopen."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-bear-anger-", ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        socket = home / "anger.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75)
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        def read(who, key):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {who} {key}")
                         if "following entity data: " in row), None)

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Bear Anger Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")

        def close():
            if socket.exists():
                ctl("quit")
            deadline = time.monotonic() + 15
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.05)
            assert not socket.exists(), "The headless control socket did not close"

        def number(who, key):
            value = read(who, key)
            return int(value.rstrip("Ll")) if value else None

        def uuid(who, key):
            value = read(who, key)
            return tuple(map(int, re.findall(r"-?\d+", value))) if value else None

        species = ("bear", "black_bear")
        outputs = {animal: {} for animal in species}
        unrelated = (17, 773, 4625, -926)
        expires = 1_000_000
        try:
            launch()
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:advance_time false")
            cmd("time set 2000")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 107 15 minecraft:air")
            cmd("tp @s 8.5 101 6.5")
            ctl("wait", 8)
            cmd("gamemode survival @s")
            cmd("effect give @s minecraft:resistance infinite 3 true")
            owner = uuid("@s", "UUID")
            assert owner is not None and len(owner) == 4, owner
            for animal in species:
                tagged = f"@e[type=naturalist:{animal},tag=actual_anger,limit=1]"
                cmd(f"summon naturalist:{animal} 8.5 101 9.2 {{Tags:[actual_anger],PersistenceRequired:1b}}")
                cmd(f"damage {tagged} 1 minecraft:player_attack by @s")
                ctl("wait", 18)
                outputs[animal]["anger_saves_target"] = uuid(tagged, "angry_at") == owner
                end = number(tagged, "anger_end_time")
                outputs[animal]["anger_saves_end_time"] = end is not None and end > 0
                cmd(f"kill {tagged}")
                word = ",".join(map(str, unrelated))
                cmd(f"summon naturalist:{animal} 11.5 101 9.2 "
                    f"{{Tags:[saved_anger],NoAI:1b,PersistenceRequired:1b,"
                    f"anger_end_time:{expires}L,angry_at:[I;{word}]}}")
            cmd("save-all flush")
            close()
            launch()
            for animal in species:
                tagged = f"@e[type=naturalist:{animal},tag=saved_anger,limit=1]"
                outputs[animal]["anger_loads_same_end_time"] = number(tagged, "anger_end_time") == expires
                outputs[animal]["anger_loads_same_target"] = uuid(tagged, "angry_at") == unrelated
            cmd("gamemode survival @s")
            cmd("effect clear @s")
            for animal in species:
                tagged = f"@e[type=naturalist:{animal},tag=active_loaded_anger,limit=1]"
                for uuid_to_save, identity in ((unrelated, "other"), (owner, "matching")):
                    universal = str(identity == "other").lower()
                    cmd(f"gamerule minecraft:universal_anger {universal}")
                    cmd("effect give @s minecraft:instant_health 1 4 true")
                    ctl("wait", 3)
                    cmd("tp @s 8.5 101 6.5")
                    health_before = float(read("@s", "Health").rstrip("Ff"))
                    word = ",".join(map(str, uuid_to_save))
                    cmd(f"summon naturalist:{animal} 8.5 101 8.9 "
                        f"{{Tags:[active_loaded_anger],PersistenceRequired:1b,"
                        f"anger_end_time:{expires}L,angry_at:[I;{word}]}}")
                    ctl("wait", 65)
                    health_after = float(read("@s", "Health").rstrip("Ff"))
                    outputs[animal][f"loaded_anger_targets_{identity}_player"] = health_after < health_before
                    cmd(f"kill {tagged}")
            cmd("gamemode creative @s")
            cmd("gamerule minecraft:universal_anger false")
            for animal in species:
                actor = f"@e[type=naturalist:{animal},tag=named_actor,limit=1]"
                pig = "@e[type=minecraft:pig,tag=named_pig,limit=1]"
                cow = "@e[type=minecraft:cow,tag=near_cow,limit=1]"
                cmd("summon minecraft:pig 8.5 101 9.5 {Tags:[named_pig],NoAI:1b,PersistenceRequired:1b}")
                cmd("summon minecraft:cow 10.5 101 8.5 {Tags:[near_cow],NoAI:1b,PersistenceRequired:1b}")
                cmd(f"attribute {pig} minecraft:max_health base set 40")
                cmd(f"data merge entity {pig} {{Health:40.0f}}")
                assert float(read(pig, "Health").rstrip("Ff")) == 40.0
                word = ",".join(map(str, uuid(pig, "UUID")))
                cmd(f"summon naturalist:{animal} 8.5 101 7.5 "
                    f"{{Tags:[named_actor],PersistenceRequired:1b,anger_end_time:{expires}L,angry_at:[I;{word}]}}")
                ctl("wait", 75)
                outputs[animal]["loaded_anger_targets_matching_mob"] = float(read(pig, "Health").rstrip("Ff")) < 40.0
                outputs[animal]["loaded_anger_targets_other_mob"] = float(read(cow, "Health").rstrip("Ff")) < 10.0
                cmd(f"kill {actor}")
                cmd(f"kill {pig}")
                cmd(f"kill {cow}")
            return outputs
        finally:
            close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    result = capture(args.version)
    print(json.dumps(result, indent=2), flush=True)
    for species, observations in result.items():
        assert all(observed == expected[species][name] for name, observed in observations.items()), (species, observations)
