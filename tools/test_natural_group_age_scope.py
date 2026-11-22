#!/usr/bin/env python3
"""Keep the Java age-group fixture honest about Horizon coverage."""
import json
from pathlib import Path

ROOT = Path(__file__).parents[1]
java = json.loads((ROOT / "fixtures/natural-group-age-java.json").read_text())
horizon = json.loads((ROOT / "fixtures/natural-group-age-horizon.json").read_text())
assert len(java) == horizon["java_species_count"] == 36
assert horizon["status"] == "blocked"
assert horizon["horizon_group_input"] is None
assert horizon["horizon_observed_groups"] is None
print("PASS: Java natural-group-age roster has 36 species")
print("OPEN: no Horizon counterpart; finalize_spawn receives group=None at every live site")
