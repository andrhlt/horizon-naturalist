"""Compare original Java and installed Horizon Firefly grass replays."""
import argparse
import json
import math
from pathlib import Path


def compare(actual, reference):
    assert actual["scene"] == reference["scene"]
    scene = actual["scene"]
    labels = ("short", "fern", "tall")
    assert all(row["positions"]["named"] is not None for row in actual["observations"])
    assert all(row["positions"]["named"] is not None for row in reference["observations"])
    actual_bursts = sorted(actual["hiding_packets"], key=lambda row: row["at"][2])
    reference_bursts = sorted(reference["hiding_packets"], key=lambda row: row["at"][2])
    actual_sounds = sorted(actual["sound_packets"], key=lambda row: row["at"][2])
    reference_sounds = sorted(reference["sound_events"], key=lambda row: row["at"][2])

    def delta(p, q):
        assert len(p) == len(q), (p, q)
        return max(abs(a - b) for a, b in zip(p, q))

    if scene == "canopy":
        assert reference["reference_light"] == {"raw": 10, "bright": True, "darken": 3}
        for game in (actual, reference):
            assert game["observations"][-1]["tick"] == 44
            assert all(game["observations"][-1]["positions"][label] is not None for label in labels)
        assert not (actual_bursts or reference_bursts or actual_sounds or reference_sounds)
        return {"scene": scene, "remaining": len(labels), "bursts": 0, "hide_sounds": 0}

    assert reference["reference_light"] == {"raw": 15, "bright": True, "darken": 0}
    assert len(actual_bursts) == len(reference_bursts) == len(labels)
    assert len(actual_sounds) == len(reference_sounds) == len(labels)
    assert len(actual["observations"]) == len(reference["observations"])
    last_positions = {}
    max_position_delta = 0.0
    for a, j in zip(actual["observations"], reference["observations"]):
        assert a["tick"] == j["tick"]
        for label in labels:
            p, q = a["positions"][label], j["positions"][label]
            if q is None:
                assert p is None, (label, a["tick"], p)
                last_positions.setdefault(label, a["tick"])
            else:
                assert p is not None, (label, a["tick"], q)
                max_position_delta = max(max_position_delta, delta(p, q))
    assert max_position_delta < 1e-6, max_position_delta

    max_origin_delta = 0.0
    for a, j in zip(actual_bursts, reference_bursts):
        assert a["count"] == j["count"] == 50
        assert a["randomization"] == 0
        assert delta(a["spread"], j["spread"]) < 1e-8
        assert delta(a["speed"], j["speed"]) < 1e-8
        max_origin_delta = max(max_origin_delta, delta(a["at"], j["at"]))
    assert max_origin_delta < 1e-6, max_origin_delta

    for a, j in zip(actual_sounds, reference_sounds):
        assert a["id"] == j["id"] == "naturalist:entity.firefly.hide"
        assert a["source"] == j["source"] == 6
        assert a["at"] == j["at"]
        assert math.isclose(a["volume"], j["volume"], abs_tol=1e-6)
        assert all(0.9 - 1e-6 <= row["pitch"] <= 1.1 + 1e-6 for row in (a, j))
    assert actual["observations"][-1]["block_particles"]["guest"] - actual["initial_block_particles"]["guest"] == 150
    return {"scene": scene, "departure_tick": last_positions, "position_delta": max_position_delta,
            "particle_origin_delta": max_origin_delta, "bursts": len(actual_bursts),
            "hide_sounds": len(actual_sounds)}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("horizon", type=Path)
    parser.add_argument("java", type=Path)
    args = parser.parse_args()
    print(json.dumps(compare(json.loads(args.horizon.read_text()), json.loads(args.java.read_text())), indent=2))
