"""Record the official Java bass's default attributes and variant IDs."""
import argparse
import json
from pathlib import Path
import re
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    command("kill @e[tag=bass_reference]")
    command("tick freeze")
    for number, name in enumerate(("bass", "bass_medium", "bass_large")):
        command(f'summon naturalist:bass {5 + 3 * number} 90 8 {{Variant:"naturalist:{name}",NoAI:1b,Tags:[bass_reference,{name}]}}')
    output = {"attributes": {}, "variants": {}, "size": {}}
    first = "@e[tag=bass,tag=bass_reference,limit=1]"
    for name in ("max_health", "movement_speed", "attack_damage"):
        rows = command(f"attribute {first} minecraft:{name} base get")
        output["attributes"][name] = next(float(m[1]) for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row)))
    for name in ("bass", "bass_medium", "bass_large"):
        target = f"@e[tag={name},tag=bass_reference,limit=1]"
        rows = command(f"data get entity {target} Variant")
        output["variants"][name] = next(row.split("following entity data: ", 1)[1] for row in rows
                                        if "following entity data: " in row)
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    parser.add_argument("--hunt", action="store_true")
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    server_args = ["sh", "run.sh", "nogui", "--world", "bass-fixture"]
    if args.hunt:
        def hunt(command, _):
            for row in command("bass_fixture_hunt"):
                if "bass_fixture_hunt {" in row:
                    return json.loads(row.split("bass_fixture_hunt ", 1)[1])
            raise RuntimeError("No bass hunting result from Java")

        java_reference.deer_behavior.capture = hunt
        with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "BassProbe.java"):
            java_reference.run_server(args, server_args)
    else:
        java_reference.deer_behavior.capture = capture
        java_reference.run_server(args, server_args)
