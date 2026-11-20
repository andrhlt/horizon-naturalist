"""Record pinned Java Whale traits, multipart geometry, and movement goals."""
import argparse
import json
import time
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 16 31")
    command("difficulty normal")
    command("tick freeze")
    command("fill 2 91 1 16 91 31 minecraft:stone")
    command("fill 2 92 1 16 100 31 minecraft:air")
    command("fill 2 92 13 16 99 13 minecraft:glass")
    command("fill 2 92 18 15 97 30 minecraft:water")
    command("fill 2 107 1 16 124 31 minecraft:air")
    command("fill 2 107 1 16 107 31 minecraft:stone")
    command("fill 2 108 1 16 123 31 minecraft:water")
    command("tick step 40")
    time.sleep(3.0)
    marker = "whale_fixture "
    for row in command("whale_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("Whale fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "WhaleProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "whale-overworld"])
