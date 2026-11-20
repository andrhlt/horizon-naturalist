"""Assert Java Dragonfly water entry and vanilla positive controls in a Horizon replay."""
import argparse
import json
from pathlib import Path


def differences(before, after):
    return {key: after[key] - before[key] for key in before}


def compare(java, horizon):
    assert java["dragonfly"] == {"events": [], "sounds": []}, java["dragonfly"]
    assert java["vanilla"]["before"] == {"events": [], "sounds": []}
    assert java["vanilla"]["after"] == {
        "events": ["entity.minecraft.pig"], "sounds": ["minecraft:entity.generic.splash"]
    }
    assert len(java["motion"]) == len(horizon["motion"])
    largest_delta = 0.0
    for original, installed in zip(java["motion"], horizon["motion"]):
        assert original["tick"] == installed["tick"]
        assert original["cases"].keys() == installed["cases"].keys()
        for case, data in original["cases"].items():
            candidate = installed["cases"][case]
            assert data.keys() == candidate.keys()
            for field in data:
                assert len(candidate[field]) == len(data[field])
                largest_delta = max(largest_delta, *(abs(a - b) for a, b in zip(data[field], candidate[field])))
    assert largest_delta < 1e-7, largest_delta

    before = horizon["motion"][0]["effects"]
    after = horizon["motion"][-1]["effects"]
    dragonfly = {view: differences(before[view], after[view]) for view in ("host", "guest")}
    vanilla = {view: differences(horizon["vanilla"]["before"][view], horizon["vanilla"]["after"][view])
               for view in ("host", "guest")}
    assert dragonfly == {view: {"bubble": 0, "splash": 0, "splash_sounds": 0} for view in ("host", "guest")}, dragonfly
    assert all(vanilla[view]["bubble"] > 0 and vanilla[view]["splash"] > 0 for view in ("host", "guest")), vanilla
    assert vanilla["guest"]["splash_sounds"] == 1, vanilla
    return {"maximum_physics_delta": largest_delta, "dragonfly": dragonfly, "pig": vanilla}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("java", type=Path)
    parser.add_argument("horizon", type=Path)
    args = parser.parse_args()
    print(json.dumps(compare(json.loads(args.java.read_text()), json.loads(args.horizon.read_text())), indent=2))
