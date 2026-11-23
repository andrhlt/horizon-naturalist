# Pod C iteration manifest

Frozen source branch: `codex/naturalist-pod-c`
Frozen source commit: `75a04af`
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
- PASS: `CARGO_NET_OFFLINE=true cargo test --lib --quiet` (46 tests)
- BLOCKED: natural-group and seam source checks require Horizon engine source files, which are not included in the pinned SDK API package.

Reusable build candidate (read-only; do not mutate or rebuild):

- `native-0a875da-8df2890cec01`
- `/Users/andrewholt/Library/Application Support/horizon-launcher/versions/native-0a875da-8df2890cec01`
- Source/build identity: Pod A `01a0587`, manifest `59215fc`
- Launcher: `/Applications/Horizon.app/Contents/MacOS/horizon-launcher`
- Build route: `horizon-launcher native-build --version nightly-2026-11-20-0a875da --trust-native-code .`
- Runtime route: `tools/launcher_selected_headless.py` with isolated `horizon-ctl` sockets

Delivery checklist:

- [x] Integrate reviewed owned commits with shared `lib.rs` and spawn ownership resolved.
- [x] Resolve duplicate native registration IDs found by the pinned build validator.
- [x] Preserve contributor attribution and individual PRs.
- [ ] Obtain one coordinated heavy-build slot for this exact Pod C source.
- [ ] Publish immutable Pod C artifact/resource/socket manifest.
- [ ] Dispatch the same artifact to APP-2130, APP-2133, and APP-2158 owners.
- [ ] Collect independent headless, LAN, persistence, and offscreen results.
- [ ] Queue fixes into the next frozen batch; no gap waivers.
- [ ] Open one combined Pod C PR only after all three acceptance checklists pass.

Known honest gaps: species-specific natural initialization and variant parity,
exact Bird RNG/variant behavior, full ocean gameplay/persistence/render parity,
and actual Bass natural groups remain runtime-gated. Exact RNG limitations stay
tracked and are not converted into passing claims.
