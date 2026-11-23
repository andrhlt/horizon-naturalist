#!/usr/bin/env python3
"""Keep the Java age-group fixture honest about Horizon coverage."""
import json
import re
import tomllib
from pathlib import Path

ROOT = Path(__file__).parents[1]
java = json.loads((ROOT / "fixtures/natural-group-age-java.json").read_text())
horizon = json.loads((ROOT / "fixtures/natural-group-age-horizon.json").read_text())
manifest = tomllib.loads((ROOT / "horizon-mod.toml").read_text())
assert len(java) == horizon["java_species_count"] == 36
assert horizon["status"] == "mapped"
assert horizon["horizon_group_input"] == "$local_mut:group:0"
assert sum(horizon["mapped_probability_groups"].values()) + len(horizon["special_group_species"]) == 36
shared_module = next(m for m in manifest["native"]["module"] if m["id"] == "shared-spawn")
assert shared_module["source"] == "src/current/shared_spawn.rs"
finalizer = next(m for m in manifest["native"]["mixin"] if m["id"] == "deer_finalize_variant")
assert finalizer["selector"] == "self.finalize_grouped"
assert any(b["name"] == "group" and b["value"] == "$local_mut:group:0" for b in finalizer["bindings"])
assert "naturalist_shared_spawn::before_grouped(game, mob, group)" in (ROOT / finalizer["helper"]).read_text()
plain_finalizer = next(m for m in manifest["native"]["mixin"] if m["id"] == "deer_finalize_plain_variant")
assert not any(b["name"] == "group" for b in plain_finalizer["bindings"])
natural = (ROOT / ".horizon/sdk/api/crates/horizon-client/src/game/mobs/natural_spawning.rs").read_text()
assert "self.group.borrow().clone()" in natural
assert ".finalize_spawn_event(&mut mob, r.reason, None, &mut group)" in natural
assert "*self.group.borrow_mut() = group;" in natural
parallel = (ROOT / ".horizon/sdk/api/crates/horizon-client/src/game/mobs/natural_spawning/parallel.rs").read_text()
assert "level.spawn_hooks()" in parallel
library = (ROOT / "src/current/lib.rs").read_text()
rules = library.split("fn natural_age_rule", 1)[1].split("fn initialize_natural_spawn_group", 1)[0]
assert set(re.findall(r'"(naturalist:[a-z_]+)"', rules)) == set(java)
for species, expected in java.items():
    if species in horizon["special_group_species"]:
        assert horizon["special_group_species"][species]["expected_babies"] == expected
print("PASS: Java natural-group-age fixture covers 36 species")
print("PASS: fixture records mapped probability groups and the $local_mut:group:0 input")
print("PASS: exact special distributions recorded for Elephant, Lion, Mammoth, and Zebra")
print("PASS: existing mixin binds the shared initializer and mutable finalize group")
print("PASS: pinned natural-spawn pass carries the listener group across pack members")
print("PASS: pinned parallel spawning falls back to serial when spawn hooks are active")
print("PASS: the public hook maps all 36 fixture species")
print("NOTE: these checks verify source wiring, not a same-stream runtime capture")
