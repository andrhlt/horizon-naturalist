#!/usr/bin/env python3
"""Check the Java fish placement contract against the native declarations.

This deliberately checks only the common `IN_WATER` contract. Species-specific
depth/light predicates are reported as open because the mod API's placement
rule has no depth field and its placement-check LevelRef exposes no light value.
"""
import json
from pathlib import Path

ROOT = Path(__file__).parents[1]
fixture = json.loads((ROOT / "fixtures/aquatic-spawn-java.json").read_text())
source = (ROOT / "src/current/aquatic_placements.rs").read_text()
species = sorted(fixture["cluster_limits"])
assert len(species) == 7, species
for kind in species:
    assert f'"{kind.split(":", 1)[1]}"' in source, kind
assert source.count("SpawnPlacementType::InWater") == 1
assert source.count("SpawnLight::Any") == 1
print(f"PASS: {len(species)} Java fish species have explicit InWater placement")
print("OPEN: Anglerfish darkness/depth and Blobfish/Piranha depth remain species predicates")
print("OPEN: Horizon SpawnPlacementRule has no depth field; LevelRef has no light-value accessor")
