"""Observe two in-love adult whales in the pinned Java server."""
import argparse
from pathlib import Path
import re
import time
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 31 31")
    command("tick freeze")
    command("fill 0 91 0 20 91 22 minecraft:stone")
    command("fill 0 92 0 20 108 22 minecraft:water")
    target = "@e[type=naturalist:whale]"
    command(f"kill {target}")
    for x in (7.5, 13.5):
        result = command(f"summon naturalist:whale {x} 98 10.5 {{InLove:600,NoGravity:1b}}")
        assert any("Summoned" in row for row in result), result
    command("tick step 150")
    time.sleep(9.0)
    rows = command(f"execute if entity {target}")
    adults = next((int(match[1]) for row in rows if (match := re.search(r"count: (\d+)", row, re.IGNORECASE))), None)
    assert adults is not None, rows
    return {"adult_count": adults, "actual_breed_goal_spawned_offspring": adults > 2}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "whale-mate"])
