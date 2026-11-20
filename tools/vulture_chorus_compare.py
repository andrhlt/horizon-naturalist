"""Confirm that actual Vultures eat chorus fruit and reach the prepared floor."""
import argparse
import json
import math
from pathlib import Path


def compare(java, horizon):
    assert java["before"] == horizon["before"]
    assert java["after"].keys() == horizon["after"].keys()
    result = {}
    for key, before in java["before"].items():
        assert "minecraft:chorus_fruit" in before["hand"]
        result[key] = {}
        for game, snapshot in (("java", java), ("horizon", horizon)):
            after = snapshot["after"][key]
            assert after["hand"] is None, (game, key, after)
            assert 100 <= after["position"][1] < 101, (game, key, after)
            movement = math.dist(before["position"], after["position"])
            assert movement > 4.0, (game, key, before, after)
            result[key][game] = {"position": after["position"], "movement": movement}
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("java", type=Path)
    parser.add_argument("horizon", type=Path)
    args = parser.parse_args()
    print(json.dumps(compare(json.loads(args.java.read_text()), json.loads(args.horizon.read_text())), indent=2))
