"""Record the official Java bird's default attributes and variant IDs."""
import argparse
from pathlib import Path
import re
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    command("kill @e[tag=bird_reference]")
    command("tick freeze")
    for number, name in enumerate(("american_robin", "blue_jay", "northern_cardinal")):
        command(f'summon naturalist:bird {5 + 3 * number} 90 8 {{Variant:"naturalist:{name}",NoAI:1b,Tags:[bird_reference,{name}]}}')
    output = {"attributes": {}, "variants": {}}
    first = "@e[tag=american_robin,tag=bird_reference,limit=1]"
    for name in ("max_health", "movement_speed", "flying_speed"):
        rows = command(f"attribute {first} minecraft:{name} base get")
        output["attributes"][name] = next(float(m[1]) for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row)))
    for name in ("american_robin", "blue_jay", "northern_cardinal"):
        target = f"@e[tag={name},tag=bird_reference,limit=1]"
        rows = command(f"data get entity {target} Variant")
        output["variants"][name] = next(row.split("following entity data: ", 1)[1] for row in rows
                                        if "following entity data: " in row)
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "bird-fixture"])
