#!/usr/bin/env python3
"""Check the Java fish placement and specialized predicate implementation."""
import json
from pathlib import Path

ROOT = Path(__file__).parents[1]
fixture = json.loads((ROOT / "fixtures/aquatic-spawn-java.json").read_text())
anglerfish = json.loads((ROOT / "fixtures/anglerfish-java.json").read_text())
blobfish = json.loads((ROOT / "fixtures/blobfish-java.json").read_text())
piranha = json.loads((ROOT / "fixtures/piranha-java.json").read_text())
source = (ROOT / "src/current/aquatic_placements.rs").read_text()
hook = (ROOT / "src/current/lib.rs").read_text()
species = sorted(fixture["cluster_limits"])
assert len(species) == 7, species
for kind in species:
    assert f'"{kind.split(":", 1)[1]}"' in source, kind
assert source.count("SpawnPlacementType::InWater") == 1
assert source.count("SpawnLight::Any") == 1
for predicate in (
    "anglerfish_spawn_ok",
    "anglerfish_surface_spawn_ok",
    "deep_water_spawn_ok",
    "sea - depth",
):
    assert predicate in source, predicate
for predicate in (
    '"naturalist:anglerfish"',
    '"naturalist:blobfish"',
    '"naturalist:piranha"',
    "mod_api::LightLayer::Block",
    "mod_api::LightLayer::Sky",
    ".sea_level(host)",
    ".fluid_state(host, at)",
):
    assert predicate in hook, predicate
assert anglerfish["dark_spawn"] is True
assert anglerfish["bright_spawn"] is False
assert anglerfish["middle_spawn"] is False
assert anglerfish["surface_spawn"] is True
assert blobfish["at_threshold"] is True
assert blobfish["above_threshold"] is False
assert piranha["deep_water"] is True
assert piranha["above_sea_level"] is False
assert piranha["dry"] is False
print(f"PASS: {len(species)} Java fish placements and Anglerfish/Blobfish/Piranha predicates")
