# APP-2131 runtime evidence

Status: partial evidence for review. APP-2131 remains open; this record does not claim full species parity.

## Candidate identity

- Branch: `codex/app2131-large-animal-renderers`
- Source base: `75cd978 Register current renderers for large Naturalist animals`, plus the working animation changes in `src/current/large_renderer.rs`.
- Horizon: `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` (26.3)
- Launcher: `0.2.82`
- Rust: `1.99.1`; optimized profile
- Installed managed candidate: `native-0a875da-351d641e0c71`
- Candidate Naturalist library SHA-256: `c9ec672ad3bd07564bd7f30510e7f23519841e57f5460b07655f4e76e4c43765`
- Candidate manifest Naturalist package SHA-256: `fd5fa37164c6fd510ee75cd7e834b24b41fce5793b5a203f02e2b8dfa076163c`
- Working source SHA-256: `e85e3ce23030bfa8b86ce3f2cb8392817129bc7dd5c30eed7c485bcf6806ab52`
- Deterministic development package SHA-256: `df281d494517ad877e37cc9ceb84c4b50da32ee021b66c120b9e1cc062a255e4`
- Java reference: Naturalist 2.0.7 NeoForge 26.3.0.95, source commit `1471124ae26bcbf90a2106a1f68080d1dd03a53a`

The candidate was built and selected for `app-2131-verify` with the launcher's `native-build --json --instance app-2131-verify --version nightly-2026-11-20-0a875da --profile optimized --trust-native-code <workspace>` flow. The candidate manifest identifies the same Horizon revision and the installed `libnaturalist.dylib` hash above.

## Passing checks

- `rustfmt --edition 2024 --config skip_children=true src/current/large_renderer.rs`, `cargo check --lib --offline`, `cargo test --lib --offline` (20 passed), and `git diff --check` passed on the working renderer implementation.
- All seven registered entity types (Elephant, Mammoth, Giraffe, Lion, Rhino, Tiger and Zebra) were present in the headless candidate; baby Elephant and Lion instances also loaded. A roaming Elephant moved from approximately z=212 to z=220 over 80 ticks.
- A managed headless save/reopen on the same candidate preserved the test Elephant as an adult at health 52.0 (`Age: 0`, `Health: 52.0f`). The headless status reported `renderer: null` and `frames: 0`, as expected for gameplay-only mode.
- Actual offscreen rendering succeeded on the same candidate. The supported launcher command was:

  ```sh
  HORIZON_LAUNCHER_HOME="$HOME/Library/Application Support/horizon-launcher" \
  HORIZON_CONTROL=/private/tmp/app2131-render.sock \
  /Applications/Horizon.app/Contents/MacOS/horizon-launcher play \
    --instance app-2131-verify \
    --version native-0a875da-351d641e0c71 \
    --world APP-2131-headless \
    --arg=--offscreen --arg=--control \
    --arg=--resources \
    --arg="$HOME/Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3" \
    --arg=--render-distance --arg=4 --json
  ```

  The runtime reported Metal, 2560×1440, over 53,000 frames, and all seven target species were placed in the rendered view. The captured frame is [`app2131-large-animals-offscreen.png`](app2131-large-animals-offscreen.png), SHA-256 `8628dbcaa7d5773d93fd16afd4c430c8707505b31355ed10bf45b8af09e2a52a`.
- The pinned runtime creates its offscreen window with `visible=false`, does not mark it active for an agent-started session, and gates physical mouse motion while offscreen. Its `docs/agent-control.md` states that offscreen mode does not take, hide or move the system pointer. The post-run frontmost application remained ChatGPT; the control status recorded zero user-originated inputs. No visible client or desktop input automation was used.

## Failed or pending checks

- **Elephant item interaction failed:** in the headless candidate, the crosshair targeted Elephant 240 at 0.75 blocks while the player held two melon slices. After a real right-click and eight ticks, health remained 52.0 instead of the Java fixture's expected 56.0, and the held melon count remained two. The Java `fixtures/elephant-java.json` reference specifies the 56.0 healing result. The separate cake attempt also produced no tame-state change. These checks are not passed.
- **LAN guest pending:** host `lan open 0 commands` succeeded on port 50462. A second instance, `app-2131-guest`, was started through the same managed launcher path and attempted `connect 127.0.0.1:50462 NatGuest`. The host rejected it with `That name is already taken`; both clients were using the only available signed-in profile, `LightTheTorch` (UUID `c0afadba732a45779467cfe95babef58`). Launcher documentation/source marks developer-offline sessions single-player-only, so that mode cannot be used to bypass this check. No guest joined; host/guest gameplay is not passed.
- The offscreen image is a stationary model frame. It does not verify walking direction, natural gait, Java pixel parity, family groups, targeting, drops, lifecycle, breeding/growth, or the full persistence matrix. The backward-walking concern remains open for a later animation review.
