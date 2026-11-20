"""Assert the exact 601-tick meal state against the pinned Java Vulture."""
import argparse
import json
from pathlib import Path
import re


def effects(value):
    found = {}
    if value:
        for entry in re.findall(r"\{([^{}]*)\}", value):
            identifier = re.search(r'id: "([^"]+)"', entry)[1]
            duration = int(re.search(r"duration: (\d+)", entry)[1])
            amplifier = re.search(r"amplifier: (\d+)", entry)
            found[identifier] = [duration, int(amplifier[1]) if amplifier else 0]
    return found


def normalized(result):
    return {
        moment: {
            case: {**record, "effects": effects(record["effects"])}
            for case, record in snapshot.items()
        }
        for moment, snapshot in result.items()
    }


def compare(original, installed):
    java, horizon = normalized(original), normalized(installed)
    assert java.keys() == horizon.keys()
    for moment, creatures in java.items():
        assert creatures.keys() == horizon[moment].keys()
        for key, expected in creatures.items():
            assert horizon[moment][key] == expected, (moment, key, expected, horizon[moment][key])
    return java["after"]


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("java", type=Path)
    parser.add_argument("horizon", type=Path)
    args = parser.parse_args()
    print(json.dumps(compare(json.loads(args.java.read_text()), json.loads(args.horizon.read_text())), indent=2))
