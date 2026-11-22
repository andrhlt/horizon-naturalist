# APP-2130 ocean wildlife evidence map

Checkpoint: `c92470b` (APP-2130 renderer checkpoint, based on APP-2120 integration `2cae30328d3de64a8b35e830d83e121482543a44`).
Pinned Java: Naturalist 2.0.7 NeoForge 26.3.0.95, `1471124ae26bcbf90a2106a1f68080d1dd03a53a`.
Pinned Horizon: `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` (26.3).

## Scope and current evidence

The checkpoint contains Java behavior fixtures and Java-derived model/animation sources for this slice. APP-2130 now registers native client renderers for all eight types, reuses the Java-derived models/keyframes, synchronizes saved `Variant` data for texture selection, selects the Whale baby mesh, and registers a shared in-water placement owner for all eight entities. This is still a renderer/placement slice; the species behavior listed in the table remains open.

| Species | Java fixture(s) | Major behavior represented in reference | Current checkpoint status |
| --- | --- | --- | --- |
| Anglerfish | `anglerfish-java.json`, `aquatic-spawn-java.json` | Variant, retaliation, extended-range hunt, cooldown, depth/light spawn | Renderer registered; default/red and glow variant mapping implemented; behavior and variant runtime comparison open |
| Blobfish | `blobfish-java.json`, `blobfish-live-prey-java.json` | Depth appearance, live-prey target, aquatic placement | Renderer registered; gray/pink texture mapping implemented; depth state, prey target, and behavior open |
| Giant Isopod | `giant-isopod-java.json`, `isopod-family-java.json` | Variant, grouped age, hiding, shell defense, food, bucket round trip | Renderer registered; brown/blue texture mapping implemented; behavior, baby scaling, and family logic open |
| Great White Shark | `shark-java.json`, `shark-current-java.json`, `shark-parts-isolated-java.json`, `shark-obstruction-java.json`, `shark-player-kill-java.json`, `shark-threat-java.json`, `shark-wet-guest-java.json` | Multipart hit regions, hunt and satiation, dry damage, wet targeting, kill cooldown, spawn water depth | Renderer registered; single-body rendering only; multipart hit regions and all listed behavior open |
| Jellyfish | `jellyfish-java.json` | Pulse timing, contact sting, nearby effects, variant and bucket round trip | Renderer registered; five texture variants mapped; pulse/contact effects and bucket behavior open |
| Ray | `ray-java.json` | Retaliation, sting poison, protected target, variant and bucket round trip | Renderer registered; eagle/mobula/stingray texture mapping implemented; behavior and bucket handling open |
| Starfish | `starfish-java.json` | Water placement, attachment, immobility, variant, death drops, bucket round trip | Renderer registered; four texture variants mapped; attachment, drops, and bucket behavior open |
| Whale | `whale-java.json`, `whale-mate-java.json`, `whale-particle-java.json` | Multipart body, baby growth, aquatic breathing, mating, food/healing, surface/breach/dive movement and effects | Adult/baby mesh selection and renderer registered; multipart body and all listed behavior open |

The shared spawning owner now supplies the common `InWater`/`OceanFloor` placement rule for all eight entities. It does not yet implement Java's species-specific depth, substrate, group composition, light, or variant initialization predicates; those remain required before any parity row can close.

## Pinned build and runtime check

- `git rev-parse HEAD` → `2cae30328d3de64a8b35e830d83e121482543a44`.
- Prepared the exact launcher pin with `horizon-launcher prepare --download --version nightly-2026-11-20-0a875da`. The Mac SDK contains a client package but no Mac server package. The original launcher's instances and worlds were left intact; test assets and world data live under `/private/tmp/app2130-*`.
- Downloaded the pinned Naturalist 2.0.7 NeoForge 26.3 jar into the isolated test home. SHA-512 verified as `b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd`.
- Optimized build on Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`: `native-0a875da-58082b8cb2c4`. `libnaturalist.dylib` SHA-256: `58be55bfd10c27b87dbcb98e2dcbb82acd1a04ded74c1f03d0d45b3ebedc0c33`; `native-mods.json` SHA-256: `8a0e2ffecc924c43c28d2b078dd06e3b893a5797530e2d043c7399e1f473a46b`.
- The final build `native-0a875da-58082b8cb2c4` launched the isolated `APP2130Runtime` world on prepared 26.3 resources. Content and registries loaded; the world reached `playing` on Metal with ocean species present. Jellyfish, an isopod, a shark, a ray and a whale are visible in the underwater frame: `/private/tmp/app2130-final.png`. This is presence/model evidence only; it is not a Java pixel comparison.
- The frame does not verify unchecked animation states. Idle/swim clips are wired from Java keyframes, but no controlled capture has compared timing, interpolation, baby scale, variant textures, hurt/death overlays, bucket states, attachment poses, or multipart Shark/Whale parts against Java.
- `horizon-launcher native-build --server-output /private/tmp/app2130-server ...` remains unavailable on this Mac because the SDK has no Mac server game package. Matching Java/Horizon behavior tests, Linux-server connection, and full variant/render comparisons have not been run.

Do not close APP-2130 parity rows or describe these species as fully ported. Add Java/Horizon scenario outcomes, persistence checks, supported client/server verification, and rendered-frame comparisons as applicable. The Mac SDK has no server package; use the documented Linux SDK/build route from APP-2120 for server and LAN checks rather than treating the Mac limitation as a server impossibility.
