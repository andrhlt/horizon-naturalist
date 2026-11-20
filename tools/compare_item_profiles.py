"""Compare the stock Horizon materialized default item table with pinned Java Naturalist."""
import argparse
import json
import math
from pathlib import Path


def collect(reference, materialized):
    present = {key for key in materialized if key.startswith("naturalist:")}
    fields = ("max_stack", "max_damage", "rarity", "food")
    differences = {}
    for key in sorted(present & reference.keys()):
        original = reference[key]
        item = materialized[key]
        food = item.get("minecraft:food")
        observed = dict(zip(fields, (
            item.get("minecraft:max_stack_size", 64),
            item.get("minecraft:max_damage", 0),
            item.get("minecraft:rarity", "common"),
            None if food is None else {
                "nutrition": food["nutrition"], "saturation": food["saturation"],
                "always_eat": food.get("can_always_eat", False),
            },
        )))
        want = original.get("food")
        if want is not None and observed["food"] is not None and math.isclose(
            want["saturation"], observed["food"]["saturation"], abs_tol=1e-6,
        ):
            observed["food"]["saturation"] = want["saturation"]
        mismatch = {field: [original.get(field), observed[field]]
                    for field in fields if original.get(field) != observed[field]}
        if mismatch:
            differences[key] = mismatch
    return {
        "reference_items": len(reference),
        "materialized_items": len(present),
        "matched": len(present & reference.keys()) - len(differences),
        "missing": sorted(reference.keys() - present),
        "extra": sorted(present - reference.keys()),
        "differences": differences,
    }


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("reference", type=Path)
    p.add_argument("materialized", type=Path)
    p.add_argument("--exact", action="store_true")
    a = p.parse_args()
    result = collect(json.loads(a.reference.read_text()), json.loads(a.materialized.read_text()))
    print(json.dumps(result, indent=2))
    if a.exact:
        assert not (result["missing"] or result["extra"] or result["differences"]), result
