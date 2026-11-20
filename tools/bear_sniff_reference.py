"""Capture real flower sniff timing in the original bear AI, without a Java client."""
import argparse
from pathlib import Path
import re
import time

import deer_behavior
from deer_behavior import reference_probe
from java_reference import run_server


SPECIES = ("bear", "black_bear")


def sample_flower_sniffs(command, counts):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:advance_time false")
    command("time set 0")
    command("forceload add 0 0")
    time.sleep(2)
    command("fill 1 100 1 15 100 15 minecraft:grass_block")
    command("fill 1 101 1 15 105 15 minecraft:air")
    command("setblock 8 101 8 minecraft:dandelion")
    command("tick freeze")
    try:
        result = {}
        for species in SPECIES:
            command("kill @e[tag=flower_bear]")
            command("tick step 25")
            time.sleep(1.5)
            before = counts()[species]
            command(f"summon naturalist:{species} 8.5 101 9.5 "
                    "{Tags:[flower_bear],Age:-24000,PersistenceRequired:1b}")
            result[species] = []
            for ticks in (6, 18, 20, 20, 20):
                command(f"tick step {ticks}")
                time.sleep(ticks / 20 + 0.2)
                result[species].append(counts()[species] - before)
        return result
    finally:
        command("tick unfreeze")


def capture(command, _):
    def counts():
        for row in command("naturalist_fixture_bear_sniff"):
            if match := re.search(r"naturalist_bear_sniff (\d+) (\d+)", row):
                return dict(zip(SPECIES, map(int, match.groups())))
        raise AssertionError("Reference server did not report flower sniff audio")

    return sample_flower_sniffs(command, counts)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("server", type=Path)
    parser.add_argument("java_home", type=Path)
    parser.add_argument("output", type=Path)
    options = parser.parse_args()
    options.variants = False
    options.grazing = False
    options.behavior = True
    deer_behavior.capture = capture
    with reference_probe(options.server, options.java_home, "BearSniffProbe.java"):
        run_server(options, ["sh", "run.sh", "nogui", "--world", "bear-sniff-fixture"])
