#!/usr/bin/env python3
"""Keep the Java age-group fixture honest about Horizon coverage."""
import json
from pathlib import Path

ROOT = Path(__file__).parents[1]
java = json.loads((ROOT / "fixtures/natural-group-age-java.json").read_text())
horizon = json.loads((ROOT / "fixtures/natural-group-age-horizon.json").read_text())
assert len(java) == horizon["java_species_count"] == 36
assert horizon["status"] == "partial"
assert horizon["horizon_group_input"] == "$local_mut:group:0"
assert sum(horizon["mapped_probability_groups"].values()) + len(horizon["special_group_species"]) == 36
assert "class-specific group distributions" in horizon["reason"]
print("PASS: Java natural-group-age roster has 36 species")
print("PASS: Horizon finalize_with local maps 32 fixture species to Java-backed Ageable probabilities")
print("OPEN: 4 species use class-specific group distributions without a faithful Horizon type")
