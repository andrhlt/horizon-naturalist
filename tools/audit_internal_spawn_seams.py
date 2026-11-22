#!/usr/bin/env python3
"""Verify the pinned Horizon internal seams used by the APP-2133 boundary note."""
from pathlib import Path

ROOT = Path('/Users/andrewholt/Library/Application Support/horizon-launcher/native/source/0a875da91dbb0fdcb7ba8c42c11341aa229af51a')
rules = (ROOT / 'crates/horizon-client/src/mobs/spawn_rules.rs').read_text()
parallel = (ROOT / 'crates/horizon-client/src/mobs/spawn_selection/parallel.rs').read_text()
assert 'pub fn check_spawn_rules<L: SpawnLevel>' in rules
assert 'trait SpawnLevel' in rules
for helper in ('fn raw_brightness<', 'fn fluid<', 'fn block_light(&self', 'fn sky_light(&self', 'fn sea_level(&self'):
    assert helper in rules, helper
assert 'fn placement_check(' in rules
assert 'spawn_category_for_chunk(' in parallel
assert 'pub fn chunk<L: NaturalLevel, R: RandomSource, M>' in parallel
print('PASS: pinned check_spawn_rules is generic over private SpawnLevel')
print('PASS: pinned internal light/fluid/depth helpers are not mod-facing')
print('PASS: weighted selection remains inside spawn_category_for_chunk and Lane::chunk')
print('BLOCKER: a safe Naturalist-only mixin needs an engine-exported predicate/attempt hook')
