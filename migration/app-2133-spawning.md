# APP-2133 spawning evidence and pin plan

## Java predicates

The Java reference is Naturalist commit `1471124ae26bcbf90a2106a1f68080d1dd03a53a` (the commit and JAR SHA-512 are recorded in `fixtures/naturalist-app2133-acceptance-fe1b833e06f7.json`). Its spawn methods define these rules:

- Anglerfish: vanilla `WaterAnimal.checkSurfaceWaterAnimalSpawnRules`, or water at/below `sea_level - 33` with raw brightness zero.
- Blobfish: water fluid at/below `sea_level - 33`.
- Piranha: water fluid at or below sea level.

The implementation keeps each species' `InWater` placement and evaluates the additional rule in `mob_spawn_placement_check`. The pinned API already exposes `LevelRef::brightness`, `LevelRef::sea_level`, and `LevelRef::fluid_state`; no placement API extension is required for these predicates.

`python3 -B tools/test_aquatic_spawn_placements.py` checks the 7 aquatic declarations and their Java fixture cases. The Rust boundary test covers Anglerfish light and surface/deep cases and the Blobfish/Piranha sea-level boundaries.

## Bird discrepancy

Keep `fixtures/naturalist-combined-3d513df20090.json` as reusable evidence: it contains 515 Java spawn rows across 67 biomes with no row mismatches. Do not regenerate it as part of APP-2133.

The selected fixed-seed cache in `fixtures/naturalist-app2133-selected-cache-a2a655b.json` still differs for Bird at seed `49903`: Horizon has 3 entities and Java has 2. Seed `49904` has 0 on both. The recorded finalizer A/B comparison produced identical Horizon positions, so Bird finalizer behavior is ruled out as the cause. The remaining candidate is the natural-spawn selection/finalization sequence before Bird finalizer behavior.

The isolated Horizon hook work adds read-only `natural_spawn_attempt` observations at category-local, start, position, type-check, and successful-spawn stages. Successful events include the zero-based group member index passed to finalization. The hook makes no random calls and returns no decision. The serial loop still owns the same random source and draw sites; observer calls occur after the corresponding stage. Existing hooked natural spawning already selects the serial path, so this does not switch a hooked run from parallel to serial.

The isolated engine source is based on Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`; the local hook commit is `511d5f883070ba7516b74bb79f1a49eee500a495`. Its generated API surface is additive (four items, zero breaking or conditional changes) with surface hash `fcdd1a7515da5be8f29cac6e6853b52d`, against frozen surface `e05637790d2a91aaa8075e748e2ca872`. This is a local development API source identity, not a published or signed SDK artifact. The official SDK release requires four platform builds, smoke checks, and signed publication; those have not run for this local branch. The pinned frozen installation and SDK were left untouched.

The engine build check passed (`cargo check -p horizon-client --lib`). The two existing RNG guard tests are marked daily-ignored. A forced ignored run reached the shared game fixture and failed at `native_entity_refs.rs:1239` on an `Option::unwrap` before reaching these hooks. No matching-artifact fixed-seed trace has been captured yet, so the Bird 3-vs-2 residual remains undiagnosed at the individual draw level, and natural group initialization has not been verified through a matching artifact.

## Shared pin plan

The patched Horizon hook changes the generated API surface. After the engine branch is reviewed and its signed SDK is published, move only this Naturalist branch's `horizon` pin to the exact patched engine commit and record the published SDK metadata here. Keep other mods on their current pins until each has been checked against the updated SDK; do not roll a workspace-wide pin from this ticket. Do not pin Naturalist to the local development surface hash as though it were a releasable SDK.

The currently installed frozen Horizon revision `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` and its SDK remain untouched and available: game `26.3`, SDK date `2026-11-20`, Rust `1.99.1`, mixins revision `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, API surface `e05637790d2a91aaa8075e748e2ca872`, package format `6`. No candidate install or game binary is part of this repository change.
