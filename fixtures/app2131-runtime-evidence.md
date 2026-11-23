# APP-2131 runtime evidence

Status: partial evidence for review. APP-2131 remains open. The owner melon interaction passes on the final candidate. An offscreen Elephant movement capture exists from the prior integrated candidate with unchanged renderer source; recapture on the final package is pending because managed Play now requires Microsoft sign-in. Authenticated host/guest LAN is blocked pending a second owning account. This does not claim full species parity.

## Candidate identity

- Branch: `codex/app2131-large-animal-renderers`
- Horizon: `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` (26.3)
- Launcher: `0.2.82`
- Rust: `1.99.1`; optimized profile
- Installed managed candidate: `native-0a875da-97b13615be3b`
- Candidate Naturalist package SHA-256: `141dd2ea86c28b074f7b11b31a0604fe8ea09bf8f9f3c6a0ce7a0c6e719242aa`
- Candidate `libnaturalist.dylib` SHA-256: `691998f93fdc5e75efcfc58cf9906a3a049aad643692cd516f893112c8178696`
- Source SHA-256: `src/current/lib.rs` `fbf0ba843e54d4aa00ac2cac294287528a51bc85094edf90f0865cf6fd9e511f`; `src/current/elephant_melon.rs` `e3019e5421d13bd3d30e82952c0be5484a94155af40c78dc74637e6f15a61fa5`; `src/current/large_renderer.rs` `e85e3ce23030bfa8b86ce3f2cb8392817129bc7dd5c30eed7c485bcf6806ab52`
- Java reference: Naturalist 2.0.7 NeoForge 26.3.0.95, source commit `1471124ae26bcbf90a2106a1f68080d1dd03a53a`

The candidate was built and selected through the pinned launcher's `native-build --json --instance app-2131-verify --version nightly-2026-11-20-0a875da --profile optimized --trust-native-code <workspace>` flow. The installed `native-mods.json` identifies the same Horizon revision, package hash and library hash above. Gameplay and rendering were then started only through launcher-managed `play`, passing the prepared 26.3 resources path; no managed binary was launched directly.

## Elephant melon interaction

Java `Elephant.mobInteract` accepts food for healing only when the elephant is tamed, the acting player is its owner, and health is below max. Java consumes one item in survival, consumes none in creative, and heals four. The cake taming branch is separate; this check seeds an owner state and does not claim to verify the five-cake taming sequence.

The new `ElephantMelon` item part is registered on `minecraft:melon_slice`. Its server dispatch checks `naturalist:elephant`, `Tame`, the owner's four-word UUID, current health/max health and live creative state; it then consumes one from the acting player's hand when appropriate and updates entity health. The test used a real headless client target and right-click, not a fixture-side direct health write for the healing result:

- `looking-at` resolved Elephant entity `257` at 1.75 blocks while the held stack was two melon slices.
- Survival owner interaction changed health `52.0f → 56.0f` and held count `2 → 1`.
- Creative owner interaction changed health `52.0f → 56.0f` and held count stayed `2`.
- At max health `100.0f`, interaction left health at `100.0f` and held count stayed `2`.
- After setting the healed target to `56.0f`, saving, quitting, and reopening the same world on the same candidate, health remained `56.0f`.

Gameplay command used the managed path:

```sh
HORIZON_LAUNCHER_HOME="$HOME/Library/Application Support/horizon-launcher" \
HORIZON_CONTROL=/private/tmp/app2131-final-headless.sock \
/Applications/Horizon.app/Contents/MacOS/horizon-launcher play \
  --instance app-2131-verify \
  --version native-0a875da-97b13615be3b \
  --world APP-2131-headless \
  --arg=--headless --arg=--control \
  --arg=--resources \
  --arg="$HOME/Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3" \
  --arg=--render-distance --arg=4 --json
```

Headless status reported `renderer: null`, `frames: 0`, one connected player, and zero user-originated input events.

## Rendered output and gait

The seven current large-animal renderers (Elephant, Mammoth, Giraffe, Lion, Rhino, Tiger and Zebra) remain registered in the source and were previously rendered on the same pinned 26.3 runtime; the static seven-species capture is [`app2131-large-animals-offscreen.png`](app2131-large-animals-offscreen.png). The current candidate also produced an offscreen Metal frame and an eight-frame live Elephant movement sequence. The sequence is [`app2131-elephant-gait.gif`](app2131-elephant-gait.gif), SHA-256 `b086091dbb34866a4ea954f338c13dade70d19d32a53011082024909063b87fd`.

The movement capture tracked the tagged AI Elephant over eight intervals of ten ticks: position changed from `[991.6498, 64.0, 1016.3528]` to `[991.7284, 64.0, 1020.3932]`. It moved about 4.04 blocks south; its final body yaw was about `11.4°`, facing generally along that travel direction. The captured motion does not reproduce backward walking for Elephant. It does not establish gait parity for every species.

The prior candidate’s supported offscreen launch used:

```sh
HORIZON_LAUNCHER_HOME="$HOME/Library/Application Support/horizon-launcher" \
HORIZON_CONTROL=/private/tmp/app2131-final-offscreen.sock \
/Applications/Horizon.app/Contents/MacOS/horizon-launcher play \
  --instance app-2131-verify \
  --version native-0a875da-9aa2d80bd80f \
  --world APP-2131-headless \
  --arg=--offscreen --arg=--control \
  --arg=--resources \
  --arg="$HOME/Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3" \
  --arg=--render-distance --arg=4 --json
```

That runtime reported Metal at 2560×1440 and 93,577 rendered frames. Offscreen status recorded no user-originated input. A repeat against final candidate `native-0a875da-97b13615be3b` failed before startup with: `Sign in with a Microsoft account that owns Minecraft: Java Edition to play.` No visible client was launched. The prior capture remains evidence for Elephant movement on the same renderer source, but final-package gait recapture remains pending. The pinned runtime's source and `docs/agent-control.md` specify that this mode does not present or activate a window and does not take, hide or move the system pointer. No visible client or desktop input automation was used.

## LAN identity blocker

APP-2126's successful two-client checkpoint is present under `/private/tmp/app2126-lan-630xzl4c`. Its host log records the LAN server and `APP2126Guest joined the game`; the host's local player file names the player `Player54777`. Those records establish that the test harness can create distinct local test names. They do not record distinct Microsoft account UUIDs or authenticated sessions, so they are not evidence of two owning accounts under managed Play.

The pinned launcher's `play --account <WHO>` accepts only a signed-in account name or UUID. `account list` currently contains one account, `LightTheTorch` (`c0afadba732a45779467cfe95babef58`). When a second managed client uses that same profile, the host rejects it with `That name is already taken`. The developer-offline launcher mode is single-player-only. A second signed-in Microsoft account that owns Minecraft: Java Edition is required for the authenticated host/guest check; the APP-2126 local test names cannot substitute without bypassing authentication. LAN remains pending.

## Remaining coverage

Family groups, broader targeting and movement, lifecycle, drops, actual cake taming, breeding/growth, per-species gait review, non-owner rejection, and broader save/network behavior remain open. Authenticated host/guest LAN also remains open pending the second owning account.
