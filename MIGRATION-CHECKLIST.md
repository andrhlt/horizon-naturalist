# Naturalist current-nightly migration checklist

Pinned target: Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, Minecraft 26.3. Java reference: Naturalist 2.0.7 NeoForge 26.3.0.95, source commit `1471124ae26bcbf90a2106a1f68080d1dd03a53a`.

This tracks implementation and verification evidence. A row is complete only when the Java behavior and current Horizon behavior have been compared with a reproducible fixture. Metadata registration alone is not a behavior port.

## Verified current candidate (`codex/naturalist-current-integration`)

Latest combined build: `native-0a875da-9232d5e30d7e` (Horizon 0a875da, optimized). Regenerated package: `/private/tmp/naturalist-ant-crab-duck-pet.hmod`, SHA-512 `89ddf5d1d9d6f3aef1281ead8a25281a730dd6b33e1f8f228b7f588c596337b12ad19ea4aa552a4f793652fd2e9694ab7d00b98d0c63cdfcc7c03f462137fabd`. On that exact build, combined headless checks passed: Ant gather parity and LAN food ownership; Duck first-visible capture/release plus foreign-owner water fallback and next deliberate use; Crab owner whistle toggles and immediate sneak interaction. Five prior uninstrumented Crab LAN repetitions also passed. Additional exact-build checks passed for Ant full-hill delivery and carried-food save/reopen, Crab FollowingOwner movement and save/reopen, and Duck capture save/reload. The checked-in focused LAN fixtures are `tools/launcher_crab_whistle_lan_sneak.py`, `tools/launcher_duck_bucket_lan_first.py`, `tools/launcher_duck_foreign_contract.py`, and the Duck pet LAN fixtures under `tools/`; Ant gather and food fixtures are also under `tools/`. The Linux connection result and reusable path-explicit runner are `fixtures/cross-platform-b12673.json` and `tools/launcher_cross_platform.py`. This was a connection check, not species interaction parity. `.hmod` contains 362 files and all 59 native module/mixin IDs are unique; it includes Duck pet and bucket hooks plus Crab follow.

- [x] Entity and item metadata profiles, supported attributes, biome spawn rows, and recorded ground placements (fixture-limited; see `migration/README.md`).
- [x] Deer selected variant, goals, appearance, breeding and love-persistence paths have Java fixture coverage. Natural-spawn variant assignment is not integrated; age/group RNG was compared in an isolated prototype only.
- [x] Bass behavior, appearance, schooling, bucket interactions; selected runtime and LAN checks.
- [x] Ant hill placement, sugar interaction, capture net selected traits, owner preservation; selected runtime and LAN checks.
- [x] Duck egg laying and Java-matched projectile hatch counts for recorded seeds.
- [x] Selected capture-net insect paths and Crab capture/release selected traits, including owner, equipment, default/legacy variants, LAN and save/reload (other Catchable animals remain open).
- [x] Wolf-backed Naturalist template health corrections on join, command, item config, API NBT, and spawn-egg data paths; source-identified callers and fixtures.
- [x] Vanilla egg common cooking tag overlay; Java recipe and furnace/smoker/campfire fixtures, including negative controls.

## Open parity work

- [x] Duck bucket capture/release: matching-client LAN wild/owned/adult/foreign behavior, first-visible foreign Duck click, controlled test-client hook withholding owner/tame metadata before prediction, water fallback, intentional next use, offhand/creative, dispenser, save/reload, and dimension reload; version contract documented. Capability announcement relies on the matching client and exact shared artifact.
- [ ] Duck pet/AI: click-scoped whistle/sit and offhand handling pass matching-client LAN/save fixtures without timers. Taming, healing, dye, follow movement/goals, sounds, appearance and animation remain open; Chicken-derived goal gaps remain.
- [x] Ant worker: gather parity, store/full-hill overflow, hill entry, carried-food UUID, pickup/anger, death/net release, save/reload, transfer, and target-alert fixes; headless combined Ant/Crab tests passed. Carried-food item rendering still lacks Java’s interpolated Ant-relative offset and shadow suppression. Java ant ambient/hurt/death/step events were compared against the exact build: Horizon plays the inherited wolf template sounds instead; see `fixtures/ant-java-sound.json` and `fixtures/horizon-ant-audio-b12673.json`.
- [x] Crab capture and FollowingOwner/whistle selected behavior: owner-aware movement, stop/resume, LAN owner selection, disconnect, cross-dimension and save/reload; immediate-sneak Java behavior repeated five uninstrumented times.
- [ ] Other Catchable species: Rat and Hedgehog capture/release selected fields and pet ownership; Java fixtures. Crab broader goals/rendering remain open.
- [ ] Remaining species-specific behavior by Java class: Bird, Butterfly lifecycle, alligator, hippo, shark, whale, crab, giraffe, isopod, piranha, anglerfish, clam, vulture, snails and other registered species. Track each Java interaction, AI, loot, breeding, growth and persistence behavior before marking a species complete.
- [ ] Spawn semantics: specialized aquatic/substrate predicates, natural spawn initialization for all species, and controlled Java RNG comparisons beyond current Deer/Bass samples.
- [ ] Ant hill world generation: both Java custom features remain absent; resolve an engine-supported implementation or clearly report the exact remaining engine limitation.
- [ ] Rendering and animation parity for every non-Deer/Bass species; current server-side entity registration is not visual parity.
- [ ] Full content sanity matrix: loot, recipes, food, drops, breeding, tame/owner state, save/reload, datapack reload, multiplayer behavior for every applicable species.
- [ ] Build final reviewed package from the integrated commit; rerun all retained fixtures headlessly on the exact artifact.
- [ ] Verify exact final artifact in Install → Build → Play and on isolated Linux server/client pair; no release until jointly tested.

## Delivery gate

Do not describe this migration as full parity or release-ready while any applicable row above remains open. Record exact commit, artifact hash, Java fixture, Horizon fixture, and outcome when closing each row.
