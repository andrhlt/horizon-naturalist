"""Capture the original Java turkey and boar behavior without a client."""
import argparse
import json
import time
from pathlib import Path

from deer_behavior import reference_probe
from java_reference import run_server


def capture(command, _):
    command("forceload add 0 0")
    command("time set 7000")
    time.sleep(2.0)
    def observations(grief):
        command(f"gamerule minecraft:mob_drops {str(grief).lower()}")
        for row in command("naturalist_fixture_land"):
            if "naturalist_land {" in row:
                return json.loads(row.split("naturalist_land ", 1)[1])
        raise RuntimeError("Java server did not report the land fixture")

    result = observations(True)
    off = observations(False)["boar_disc"]
    assert off["fatal_hits"] > 0, off
    result["boar_disc"]["discs_when_mob_drops_disabled"] = off["discs"]
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("server", type=Path)
    parser.add_argument("java_home", type=Path)
    parser.add_argument("output", type=Path)
    options = parser.parse_args()
    options.variants = False
    options.grazing = False
    options.behavior = True
    import deer_behavior

    deer_behavior.capture = capture
    with reference_probe(options.server, options.java_home, "LandProbe.java"):
        run_server(options, ["sh", "run.sh", "nogui", "--world", "land-fixture"])
