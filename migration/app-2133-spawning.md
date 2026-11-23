# APP-2133 spawning handoff

## Frozen baseline and scope

Java reference: Naturalist `1471124ae26bcbf90a2106a1f68080d1dd03a53a` (2.0.7 NeoForge 26.3.0.95). Horizon remains `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, Minecraft 26.3, SDK Rust 1.99.1. No engine patch publication or pin change is part of this PR.

## Implemented behavior

- The seven fixture-rostered aquatic species use Java's `IN_WATER` placement and natural cluster caps: Anglerfish 8, Bass 6, Blobfish 8, Catfish 8, Jellyfish 8, Piranha 24, Ray 8.
- The existing `mob_spawn_placement_check` handles Natural Anglerfish, Blobfish, and Piranha candidates. Anglerfish accepts the Java surface-water case or deep water at/below sea level minus 33 with block and sky light zero. Blobfish requires the default water-placement result and Y at/below sea level minus 33. Piranha requires the default water-placement result and Y at/below sea level.
- These checks use existing pinned APIs for the candidate, default placement result, sea level, fluid/block state, brightness, and `SpawnCheck`. The callback can accept or reject a concrete candidate; it cannot force a species to be selected, expose every attempt/RNG draw, or report spawn-group state.

## Verification and remaining gaps

Current source checks pass:

- `CARGO_NET_OFFLINE=true cargo test --lib --quiet` — 26 tests.
- `python3 -B tools/test_aquatic_spawn_placements.py` — seven placement declarations and Java predicate cases.
- `python3 -B tools/test_aquatic_spawn_limits.py` — seven aquatic natural group caps.
- `python3 -B tools/test_bird_spawn_coverage.py` — 67 biomes, 72 Bird rows.
- `python3 -B tools/verify_bird_variant_pools.py` — 36/36 biome pools.
- `python3 -B tools/test_natural_group_age_scope.py` — 36 species and Java-backed special group distributions.

Reusable table evidence remains `fixtures/naturalist-combined-3d513df20090.json` (515 Java spawn rows across 67 biomes, zero row mismatches). Do not regenerate it for this ticket.

Earlier matching frozen-SDK headless evidence on candidate `native-0a875da-408c6cec89f6` observed natural Butterfly and Firefly spawns and Bird initialization. Its installed `libnaturalist.dylib` SHA-256 was `baa44853edf3af9d179acd2ad4d0f85c8796baa44a1d31dcdde4b120c1e8dc1e`. This is prior candidate evidence; the current merged PR head still needs the centralized APP-2135 artifact/build and runtime checks.

The public `finalize_spawn` listener now maps all 36 Java age-group species, including Elephant, Lion, Mammoth, and Zebra's member-indexed transitions. In the pinned natural spawner, `group_index == 0` starts with no group, later members receive `self.group` from the prior candidate, the event mutates that same group, and the returned finalizer group is saved for the next member. The unit tests cover the mapped group sequences; `tools/test_natural_group_age_scope.py` checks the manifest wiring, pinned source flow, and all 36 mapped species. The existing `shared_spawn.rs` helper is registered as a native module and called through the existing `deer_finalize_variant` mixin's mutable `finalize_grouped` local for other paths using `finalize_with`. Natural Bird variant selection remains a gap because the natural spawner proceeds from `finalize_spawn` directly to the finalizer, while the public event cannot edit a Variant field. APP-2135 should verify current source on its combined candidate. No engine source change or deployment pin is needed.

The earlier high-weight river Piranha probe observed no aquatic entities after 1,200 ticks but did not establish that a valid Piranha candidate was attempted, so it is inconclusive. A fresh headless launch of the registered matching candidate is currently blocked at Horizon Launcher's Microsoft account ownership gate. Do not bypass authentication or managed cache leases. APP-2135 owns the next shared controlled aquatic run using loaded chunks, correct biome/fluid/light/depth, and the normal production spawn path.

Bird seed `49903` remains Horizon 3 vs Java 2; `49904` is 0 vs 0. Exact cross-engine Bird count/RNG parity is a separate full-parity follow-up absent a demonstrated player-visible defect.

## Ownership and next action

APP-2133 code is on [PR #22](https://github.com/andrhlt/horizon-naturalist/pull/22), based on current main `3d03fb7107f9c93f5bc96d5e18602c0921d7d752`. This branch owns `src/current/aquatic_placements.rs`, the aquatic placement callback/registration in `src/current/lib.rs`, Natural Bass's natural cluster cap in `src/current/bass_hunt.rs`, the focused predicate test, and this spawning record/checklist entry. Full-package build, shared cache use, and the integrated aquatic runtime result belong to APP-2135. Keep APP-2133 In Review until its gameplay acceptance gates receive matching integrated runtime evidence.
