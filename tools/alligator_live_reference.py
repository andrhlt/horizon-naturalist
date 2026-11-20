"""Record the actual Java Alligator target goals across nine controlled cases."""
import argparse
import json
import time
from uuid import uuid4
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:random_tick_speed 0")
    command("gamerule minecraft:natural_health_regeneration false")
    command("forceload add 0 0 16 0")
    command("difficulty normal")
    command("weather clear")
    command("tick freeze")
    command("fill 3 115 2 13 115 13 minecraft:stone")
    command("fill 3 116 2 13 121 13 minecraft:air")
    command("fill 7 114 7 12 114 12 minecraft:stone")
    command("fill 7 115 7 12 115 12 minecraft:water")
    marker = "alligator_live "
    output = {}
    for phase in ("prey", "sated", "nest", "nest_three_away", "day_player", "dark_player",
                  "pregnant_player", "water_player", "baby"):
        command("time set " + ("18000" if phase == "dark_player" else "6000"))
        command("alligator_live " + phase)
        found = {"targeted": False, "injured": False, "wet": False}
        for _ in range(10):
            command("tick step 16")
            time.sleep(1.0)
            rows = command("alligator_live observe")
            record = next(json.loads(row.split(marker, 1)[1]) for row in rows if marker + "{" in row)
            for key, value in record.items():
                found[key] |= value
            if found["targeted"] and found["injured"]:
                break
        assert phase != "water_player" or found["wet"], found
        print(phase, found, flush=True)
        output[phase] = found
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "AlligatorLiveProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "alligator-live-" + uuid4().hex[:8]])
