# APP-2133 shared spawning audit

Pinned references: Naturalist 2.0.7 NeoForge 26.3.0.95 (`1471124ae26bcbf90a2106a1f68080d1dd03a53a`) and Horizon 26.3 (`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`).

## Verified baseline

- Added `tools/verify_spawn_registry.py` as a source-only acceptance gate for
  environments without the pinned Horizon SDK checkout. It validates that the
  48 Java profile owners and 48 Java variant-pool owners have Horizon entity
  definitions, allows the intentional Bird/Snake per-pool modifier repeats,
  and checks that fixture-covered natural-spawn species have a modifier. The
  current tree passes with 48 metadata species, 48 variant species, 50 mob
  entity definitions, and 52 ordered modifiers (45 fixture species included).
  Reproduce with `python3 -B tools/verify_spawn_registry.py`.

- The standard biome modifier rows are emitted in Java `forEachSpawn` order by `tools/pack_spawn_tables.py`. Exact integrated-candidate diagnostics in `fixtures/naturalist-combined-3d513df20090.json` report zero row or category mismatches for 67 biomes.
- Added `tools/compare_spawn_bundles.py` to rerun this comparison against a full Horizon biome bundle. Using `fixtures/bird-spawn-tables-java.json`, candidate `c7c460f00d11` has 0 mismatched biomes and 0 mismatched categories across all 67 Java biome fixtures (515 total Java rows). The preceding `fa3fed977e48` bundle has 30 mismatched biomes / 33 categories. This confirms that row ordering was a real and corrected cause of the earlier fixed-seed Bird delta. The comparator checks exact order for Java-listed species in each biome; it does not claim to detect every possible extra vanilla row.
- Reproduction command: `python3 -B tools/compare_spawn_bundles.py fixtures/bird-spawn-tables-java.json <candidate-worldgen/biomes.json> --candidate <candidate-id>`. The saved c7 bundle is `bird-biomes-native-0a875da-c7c460f00d11.json` in the historical bird-density audit directory; the exact-candidate JSON summary is `fixtures/naturalist-combined-3d513df20090.json`.
- The 67-biome spawn order reference is retained in `fixtures/bird-spawn-tables-java.json`. `fixtures/land-spawn-tables-java.json` captures Java spawn tables with a different row order and serves a separate table audit. `fixtures/bird-spawn-tables.json` is the selected-biome weight audit, not a full Horizon bundle dump.
- Shared ground placement is declared for Deer and 18 land species in `src/current/land_placements.rs`. The one-heightmap, one-substrate, bright-light rule is established for those species; fixture coverage does not establish every terrain edge.
- The Java aquatic tables and cluster limits are retained in `fixtures/aquatic-spawn-java.json`. Predicate probes are present in species-specific Java fixtures, including `fixtures/anglerfish-java.json`, `fixtures/hippo-spawn-java.json`, and `fixtures/insect-spawn-java.json`.
- `fixtures/natural-group-age-java.json` records 128 seeded trials of four-member groups for 36 ageable species. This is reference data only: there is no corresponding integrated Horizon per-species seeded-finalization comparison.
- `fixtures/variant-order-java.json` enumerates 48 species' Java variant pools. It is not by itself proof that Horizon registers or chooses all those variants correctly.
- Registry roster audit against `migration/entity-metadata.json`: all 48 Java profile species and all 48 Java variant-pool species are represented. `naturalist:dirt_trail` is the sole metadata-only entry and is a non-mob; no missing species or variant-pool owner was identified in these fixtures.
- Deer spawn variant selection is hooked for grouped and plain finalizers (`src/current/deer_finalize_variant.rs`, `src/current/deer_finalize_plain_variant.rs`). No shared natural-spawn initialization hook currently covers Bird and the remaining variant-bearing species.

## Bird RNG finding

The modifier ordering correction is proven to remove the spawn-row-order mismatch. It does not close fixed-seed world generation: on seed `49903`, the normal-probability sample produced 3 Horizon Birds and 2 Java Birds; with controlled 0.5 probability, it produced 29 Horizon Birds and 32 Java Birds. Exact Java position overlap rose from 10 to 21 after ordering. Seed `49904` produced zero Birds on both. See `fixtures/naturalist-combined-3d513df20090.json` and the archived run logs under the bird-density audit referenced there.

These density observations do not isolate which residual draw differs. `fixtures/bird-region-java.json` provides a seed-injected five-member Java finalization sample, including each member's variant, age, follow-range bonus, handedness, and final RNG state. A matching Horizon finalization capture on the same source artifact and injected stream is needed before changing spawn RNG or group initialization. Do not treat the observed count delta alone as evidence for a particular RNG fix.

## Remaining work

- Add a controlled same-stream Horizon counterpart to the Java natural group age and Bird region probes, keyed to the exact integrated source and engine revisions.
- Use those comparisons to implement and verify shared natural age, variant, group-data, and random-draw behavior. Preserve command, bucket, breeding, and load spawn reasons.
- Compare all applicable Java biome, substrate, fluid, light, depth, surface, and cluster predicates against the Horizon candidate; current fixture coverage is selective.
- Audit the pinned Java entity and variant registries against the current declarations and data, then track any real omissions separately.

No implementation, exact-candidate build, or spawn-runtime parity claim is made by this audit.
