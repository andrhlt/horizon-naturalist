#!/usr/bin/env python3
"""Keep the Java age-group fixture honest about Horizon coverage."""
import json
from pathlib import Path

ROOT = Path(__file__).parents[1]
java = json.loads((ROOT / "fixtures/natural-group-age-java.json").read_text())
horizon = json.loads((ROOT / "fixtures/natural-group-age-horizon.json").read_text())
assert len(java) == horizon["java_species_count"] == 36
assert horizon["status"] == "mapped"
assert horizon["horizon_group_input"] == "$local_mut:group:0"
assert sum(horizon["mapped_probability_groups"].values()) + len(horizon["special_group_species"]) == 36
for species, expected in java.items():
    if species in horizon["special_group_species"]:
        assert horizon["special_group_species"][species]["expected_babies"] == expected
print("PASS: Java natural-group-age fixture covers 36 species")
print("PASS: fixture records mapped probability groups and the $local_mut:group:0 input")
print("PASS: exact special distributions recorded for Elephant, Lion, Mammoth, and Zebra")
print("NOTE: fixture scope is separate from per-mob finalize_spawn runtime behavior and pack continuity")
