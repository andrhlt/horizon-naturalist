"""Replay the dragonfly hover and chorus fruit behaviors using the installed build."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, fixture):
    with tempfile.TemporaryDirectory(prefix="nat-wings-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "wings.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def data(target, key):
            rows = cmd(f"data get entity {target} {key}")
            return next((row.split("following entity data: ", 1)[1] for row in rows
                         if "following entity data: " in row), None)

        def position(target):
            return tuple(float(v) for v in re.findall(r"(-?\d+(?:\.\d+)?(?:e[+-]?\d+)?)d", data(target, "Pos")))

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Insect Wings Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 110 8.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 109 15 minecraft:air")
            cmd("tick freeze")
            cmd("summon naturalist:dragonfly 11.5 106 11.5 {Tags:[command_gravity]}")
            command_gravity = data("@e[tag=command_gravity,limit=1]", "NoGravity") in ("true", "1b", "1")
            cmd("kill @e[tag=command_gravity]")
            cmd("summon naturalist:dragonfly 11.5 105 11.5 {Tags:[flight_test],NoGravity:1b,PersistenceRequired:1b}")
            dragonfly = "@e[tag=flight_test,limit=1]"
            before = position(dragonfly)
            no_gravity = data(dragonfly, "NoGravity") in ("true", "1b", "1")
            step(30)
            at_30 = position(dragonfly)
            step(16)
            at_46 = position(dragonfly)
            cmd("kill @e[tag=flight_test]")

            cmd("tp @s 7.5 100 6.5")
            cmd("summon naturalist:dragonfly 7.5 100 8.5 {NoAI:1b,Health:1.0f,Tags:[feeding_test],PersistenceRequired:1b}")
            cmd("summon minecraft:chicken 7.7 100 8.5 {NoAI:1b,NoGravity:1b,Tags:[cloud_target],PersistenceRequired:1b}")
            cmd("gamemode survival @s")
            cmd("item replace entity @s weapon.mainhand with minecraft:chorus_fruit 2")
            cmd("tick unfreeze")
            ctl("look", 0, 32)
            ctl("mouse", "right", "click")
            ctl("wait", 3)
            cloud = "@e[type=minecraft:area_effect_cloud,limit=1]"
            details = {
                "radius": data(cloud, "Radius"),
                "duration": data(cloud, "Duration"),
                "effects": data(cloud, "potion_contents.custom_effects"),
                "particle": data(cloud, "custom_particle"),
            }
            ctl("wait", 23)
            chicken_health = data("@e[tag=cloud_target,limit=1]", "Health")
            after_health = data("@e[tag=feeding_test,limit=1]", "Health")
            output = {
                "summon_dragonfly_no_gravity": command_gravity,
                "dragonfly_still_after_30": before == at_30,
                "dragonfly_moves_at_46": at_30 != at_46,
                "dragonfly_no_gravity": no_gravity,
                "chorus_cloud": details,
                "chorus_hurts_chicken": chicken_health is None or float(chicken_health.rstrip("f")) < 4.0,
                "chorus_heals_dragonfly": after_health is not None and float(after_health.rstrip("f")) > 1.0,
                "positions": [before, at_30, at_46],
            }
            print(json.dumps(output, indent=2), flush=True)
            assert all(output[key] == fixture[key]
                       for key in ("dragonfly_still_after_30", "dragonfly_moves_at_46", "dragonfly_no_gravity")), output
            assert details["radius"] == "0.5f", details
            assert details["duration"] == "200", details
            assert output["chorus_hurts_chicken"] and output["chorus_heals_dragonfly"], output
            if "summon_dragonfly_no_gravity" in fixture:
                assert output["summon_dragonfly_no_gravity"] == fixture["summon_dragonfly_no_gravity"], output
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
