"""Assert Bird contact and cramming parity between original Java and installed Horizon."""
import argparse
import json
from pathlib import Path


def compare(java, horizon):
    assert len(java["motion"]) == len(horizon["motion"])
    assert java["cramming"] == horizon["cramming"], (java["cramming"], horizon["cramming"])
    assert java["cramming"] == {
        "before": {"vulnerable": 6.0, "protected": 6.0},
        "after": {"vulnerable": None, "protected": 6.0},
    }
    largest_delta = 0.0
    for original, installed in zip(java["motion"], horizon["motion"]):
        assert original["tick"] == installed["tick"]
        assert original["cases"].keys() == installed["cases"].keys()
        for case, expected in original["cases"].items():
            observed = installed["cases"][case]
            assert expected.keys() == observed.keys()
            for key in expected:
                assert len(expected[key]) == len(observed[key]) == 3
                largest_delta = max(
                    largest_delta,
                    *(abs(a - b) for a, b in zip(expected[key], observed[key])),
                )
    assert largest_delta < 1e-7, largest_delta
    first_tick = horizon["motion"][1]["cases"]
    return {
        "maximum_physics_delta": largest_delta,
        "bird_pair_impulses": [first_tick[key]["motion"][0] for key in ("a", "b")],
        "mixed_pair_impulses": [first_tick[key]["motion"][0] for key in ("mixed_bird", "sheep")],
        "cramming": horizon["cramming"],
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("java", type=Path)
    parser.add_argument("horizon", type=Path)
    args = parser.parse_args()
    print(json.dumps(compare(json.loads(args.java.read_text()), json.loads(args.horizon.read_text())), indent=2))
