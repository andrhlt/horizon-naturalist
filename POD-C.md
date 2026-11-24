# Pod C iteration manifest

Frozen source branch: `codex/naturalist-pod-c`
Frozen source commit: `fb45546a2d869ba21d8c316eeec762af735967b1`
Base: `origin/main` / `3d03fb7107f9c93f5bc96d5e18602c0921d7d752`

Included reviewed heads:

- APP-2130 ocean: `b13a934e80516336b0ca8408ecf900996b6fd153` (PR #19)
- APP-2133 spawning: `29adc01a034367d9582fec11cafc59148acf8d5a` (PR #22)
- APP-2158 Bass/Fox/Rabbit: `990fe207df4c647a03f614844acad097673b37ca` (PR #35)

Pinned cohort: Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, MC 26.3,
matching SDK, Rust 1.99.1, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`.

Fast checks:

- PASS: `git diff --check`
- PASS: `python3 -B tools/test_aquatic_spawn_placements.py`
- PASS: `CARGO_NET_OFFLINE=true cargo test --lib --quiet` (48 tests)
- BLOCKED: natural-group and seam source checks require Horizon engine source files, which are not included in the pinned SDK API package.

Integration correction: owner runtime startup identified duplicate Anglerfish,
Blobfish, Jellyfish, and Ray placement IDs. Reviewed fix `3cb4303` leaves
those IDs with `ocean_placements` and keeps Bass, Catfish, and Piranha in
`aquatic_placements`; the source check now covers both registrars.

Next frozen batch includes focused owner fixes: APP-2158 Fox/Rabbit server
config registration/reload filtering (`16e569f`) and APP-2130 transactional
aquatic bucket capture with rollback on discard failure (`2c6081b`). The prior
runtime artifact remains preserved and is not acceptance evidence for these
changes.

This batch additionally scopes the forest filter to `minecraft:forest`
(`60dbde4`) and implements Bass bucket release with variant restoration
(`d154478`).

Build-scope note: the pinned manifest validator found stale registrations for
unavailable, non-Pod-C Catfish, Clam, Capybara, Piranha, Alligator, and Hippo
source files. Those native blocks were removed from this Pod C candidate only;
their gameplay ownership and acceptance remain with their contributing pods.
No APP-2130 ocean, APP-2133 shared-spawn, or APP-2158 Bass/Fox/Rabbit
registration was removed.

Reusable build candidate (read-only; do not mutate or rebuild):

- `native-0a875da-8df2890cec01`
- `/Users/andrewholt/Library/Application Support/horizon-launcher/versions/native-0a875da-8df2890cec01`
- Source/build identity: Pod A `01a0587`, manifest `59215fc`
- Launcher: `/Applications/Horizon.app/Contents/MacOS/horizon-launcher`
- Build route: `horizon-launcher native-build --version nightly-2026-11-20-0a875da --trust-native-code .`
- Runtime route: `tools/launcher_selected_headless.py` with isolated `horizon-ctl` sockets

Pod C immutable build manifest:

- Artifact: `native-0a875da-18ab1318b90b` (developer-offline)
- Artifact path: `/private/tmp/pod-c-dev-launcher/versions/native-0a875da-18ab1318b90b`
- Source: `dde2c2c9c1d839ef2d6431adfdd0ddf84fcdb466`; native manifest mod SHA-256 `99663c79ad49c704af55db926ec58e0d80f5be91b322fc8fd1b5c26ba7ff490f`
- `libnaturalist.dylib`: `9af309fdbc5a0898cce028374084c6691e28a6557d4e21661a36b7d50e8d9247`
- `libhorizon_patch.dylib`: `55fae06acca8a60ecc82b2c5d23d335dbdd4645592c6de6a0814e0a32eea1e74`
- `native-mods.json`: `dc0fdcbebf4e2abd7f10c21bcfb2ab1cba1c325fccf5978110a80b5723167210`
- Client/server: `horizon-client` `012bf6dbfecb095bfb27f9b84452105639501cc5444aca15517360cd90547935`; `horizon-server` `2b0f3d7bd318f52a8385b7d7b28db78d13f7752d25fd30d824d487aef5fe1f1d`
- Developer base: `developer-e05268bea02f4384b113d9b9b02f0ce6310883561ccc41fea947e097148ade16`; prepared resources: `/private/tmp/pod-c-dev-launcher/versions/developer-e05268bea02f4384b113d9b9b02f0ce6310883561ccc41fea947e097148ade16/.local/horizon-client/26.3`
- Pinned Java jar: `/Users/andrewholt/Library/Application Support/horizon/jars/b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd.jar`; SHA-256 `f85d8823002483a9d2822d2fd9f3cb880809d4c2eabf1cf19e24d6a14e2874e9`
- SDK surface/toolchain: Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, MC 26.3, mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, Rust 1.99.1; SDK surface `e05637790d2a91aaa8075e748e2ca872`
- Supported build command: `HORIZON_LAUNCHER_HOME=/private/tmp/pod-c-dev-launcher HORIZON_HOME=/private/tmp/pod-c-dev-horizon HORIZON_LAUNCHER_DEVELOPER_OFFLINE=1 CARGO_TARGET_DIR=/private/tmp/pod-c-dev-target HORIZON_NO_AUDIO=1 /Applications/Horizon.app/Contents/MacOS/horizon-launcher native-build --version developer-e05268bea02f4384b113d9b9b02f0ce6310883561ccc41fea947e097148ade16 --instance horizon --trust-native-code --profile optimized .`
- Supported headless route: `HORIZON_LAUNCHER_HOME=/private/tmp/pod-c-dev-launcher HORIZON_HOME=/private/tmp/pod-c-dev-horizon HORIZON_LAUNCHER_DEVELOPER_OFFLINE=1 horizon-launcher play --instance horizon --version native-0a875da-f6c6f46cc7d0`, with unique world/socket and `horizon-ctl`; keep each owner isolated.

Delivery checklist:

- [x] Integrate reviewed owned commits with shared `lib.rs` and spawn ownership resolved.
- [x] Resolve duplicate native registration IDs found by the pinned build validator.
- [x] Preserve contributor attribution and individual PRs.
- [ ] Obtain one source-matched heavy build for this exact Pod C source.
- [x] Publish immutable Pod C artifact/resource/socket manifest (corrected candidate after duplicate-registration startup failure).
- [x] Dispatch the same artifact to APP-2130, APP-2133, and APP-2158 owners.
- [ ] Collect independent headless, LAN, persistence, and offscreen results.
- [ ] Queue fixes into the next frozen batch; no gap waivers.
- [ ] Open one combined Pod C PR only after all three acceptance checklists pass.

Known honest gaps: species-specific natural initialization and variant parity,
exact Bird RNG/variant behavior, full ocean gameplay/persistence/render parity,
and actual Bass natural groups remain runtime-gated. Exact RNG limitations stay
tracked and are not converted into passing claims.
