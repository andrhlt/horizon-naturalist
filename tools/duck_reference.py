"""Capture the duck's initial gameplay fixture from Naturalist Java 2.0.7."""
import argparse
import json
from pathlib import Path
import re
import time

import java_reference


def capture(command, y):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    command("kill @e[tag=duck_fixture]")
    command("kill @e[type=minecraft:item,x=8,y=91,z=8,distance=..10]")
    command("fill 1 90 1 15 90 15 minecraft:stone")
    command("fill 1 91 1 15 96 15 minecraft:air")
    command("tick freeze")

    adult = "@e[type=naturalist:duck,tag=duck_adult,limit=1]"
    baby = "@e[type=naturalist:duck,tag=duck_baby,limit=1]"
    command('summon naturalist:duck 5 91 8 {EggLayTime:2,NoAI:1b,Tags:[duck_fixture,duck_adult]}')
    command('summon naturalist:duck 9 91 8 {Age:-24000,EggLayTime:2,NoAI:1b,Tags:[duck_fixture,duck_baby]}')
    output = {"attributes": {}, "before": {}, "after": {}}
    for attribute in ("max_health", "movement_speed", "step_height"):
        reply = command(f"attribute {adult} minecraft:{attribute} base get")
        output["attributes"][attribute] = next(float(re.search(r" is ([0-9.eE+-]+)", row)[1])
                                                for row in reply if " is " in row)

    def query_data(entity, path):
        reply = command(f"data get entity {entity} {path}")
        return next(row.split("following entity data: ", 1)[1] for row in reply
                    if "following entity data: " in row)

    for who, selector in (("adult", adult), ("baby", baby)):
        output["before"][who] = {key: query_data(selector, key)
                                  for key in ("Age", "Health", "EggLayTime", "Variant")}
    command("tick step 2")
    time.sleep(0.3)
    for who, selector in (("adult", adult), ("baby", baby)):
        output["after"][who] = {key: query_data(selector, key) for key in ("Age", "EggLayTime")}
    selector = '@e[type=minecraft:item,nbt={Item:{id:"naturalist:duck_egg"}},distance=..4,limit=1]'
    reply = command(f"execute positioned 5 91 8 if entity {selector}")
    output["adult_laid_duck_egg"] = any("Test passed" in row for row in reply)
    assert output["adult_laid_duck_egg"] and output["after"]["baby"]["EggLayTime"] == "2", output
    assert 6000 <= int(output["after"]["adult"]["EggLayTime"]) <= 11999, output
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    parser.add_argument("--pets", action="store_true")
    args = parser.parse_args()
    args.variants = False
    args.grazing = False
    args.behavior = True
    server_args = ["sh", "run.sh", "nogui", "--world", "duck-fixture"]
    if args.pets:
        def pets(command, _):
            for row in command("duck_fixture_pets"):
                if "duck_fixture_pets {" in row:
                    return json.loads(row.split("duck_fixture_pets ", 1)[1])
            raise RuntimeError("No Java duck interaction result")

        java_reference.deer_behavior.capture = pets
        with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "DuckProbe.java"):
            java_reference.run_server(args, server_args)
    else:
        java_reference.deer_behavior.capture = capture
        java_reference.run_server(args, server_args)
