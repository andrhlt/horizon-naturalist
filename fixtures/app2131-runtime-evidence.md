# APP-2131 runtime evidence

Status: partial evidence for review. APP-2131 remains open. The owner melon interaction passes on the final candidate. The offscreen Elephant movement capture from the prior integrated candidate applies to the final package because renderer, model, animation and prepared-resource inputs are unchanged. Authenticated host/guest LAN is blocked pending a second owning account. This does not claim full species parity.

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

The seven current large-animal renderers (Elephant, Mammoth, Giraffe, Lion, Rhino, Tiger and Zebra) remain registered in the source and were rendered on the pinned 26.3 runtime; the static seven-species capture is [`app2131-large-animals-offscreen.png`](app2131-large-animals-offscreen.png). The movement candidate `native-0a875da-9aa2d80bd80f` produced an offscreen Metal frame and an eight-frame live Elephant movement sequence. The final gameplay candidate is `native-0a875da-97b13615be3b`. The sequence is [`app2131-elephant-gait.gif`](app2131-elephant-gait.gif), SHA-256 `b086091dbb34866a4ea954f338c13dade70d19d32a53011082024909063b87fd`.

The movement capture tracked the tagged AI Elephant over eight intervals of ten ticks: position changed from `[991.6498, 64.0, 1016.3528]` to `[991.7284, 64.0, 1020.3932]`. It moved about 4.04 blocks south; its final body yaw was about `11.4°`, facing generally along that travel direction. The captured motion does not reproduce backward walking for Elephant. It does not establish gait parity for the other six species.

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

That runtime reported Metal at 2560×1440 and 93,577 rendered frames. Offscreen status recorded no user-originated input. A repeat attempt against final candidate `native-0a875da-97b13615be3b` failed before startup with: `Sign in with a Microsoft account that owns Minecraft: Java Edition to play.` No visible client was launched. The candidate comparison below establishes that the prior gait sequence uses the final package’s same renderer inputs. The pinned runtime's source and `docs/agent-control.md` specify that this mode does not present or activate a window and does not take, hide or move the system pointer. No visible client or desktop input automation was used.

## Renderer and gait input comparison

The captured `9aa2d80bd80f` movement candidate and final `97b13615be3b` gameplay candidate use the same renderer and animation source inputs. `src/current/large_renderer.rs` is SHA-256 `e85e3ce23030bfa8b86ce3f2cb8392817129bc7dd5c30eed7c485bcf6806ab52`; model inputs are `elephant_models.rs` `eb1ad0848b50dc73e14bb7b7a54920c6ed986b01d65b6ccc7b3ecf71a48afd26`, `mammoth_models.rs` `1756f0a088cf10f9bacc8e4e42ab5d4fed2f69eead7163b43eea7f9402639009`, `giraffe_models.rs` `55a6fc7561a88fd629b5b8275d8518a75952dceaed170f2e25a7f3eea702465a`, `lion_models.rs` `384dc91ae3a1692dfc983d25a70a491334f66c551d8907c798a8d7d806e6406b`, `rhino_models.rs` `891a239e13ef954067f82942255fe7b29a7670fe14cd5e1c9a5a9bdc51390493`, `tiger_models.rs` `863716fbd66e1b9b295b09155e1f07c5755b4e6b29d51c1eb1353c7e6b789dfe`, and `zebra_models.rs` `0d91c60d5bdebf2912ae12041a9717cc9b2c364fdae8a132ba5abef7fe4030e5`. The corresponding animation inputs are Elephant `8beac72f7a8e9aec64bd9d69343d40077cabec13b57620179e0061c1371687ac`, Giraffe `48ca79f558f373c8fdd9343747274ddccf725750f251a3bc7b13292de73a126d`, Lion `952e89379f98f18b4152ee93bde3aa980b46d6107a2025269dd3a4be73fd1656`, Rhino `0e67d818c068a2f2530cf9510adce9fb94cbaa35e5e4da4a662f169001157441`, Tiger `3008e3958c008a3613488ee7d50f7e1fa6d38be9e82aa52363a492ebcc5223ec`, and Zebra `41585a82f5ee8830e757b28ec5743985b5e4995738dbb27275c99bf26c25b5cc` (all files under `src/`). Mammoth uses the Elephant animation input. Both candidates are Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, optimized Rust `1.99.1`, and the same prepared 26.3 resources at `nightly-2026-11-20-0a875da/.local/horizon-client/26.3`. The package and library hashes differ between the builds; the only source edit between them was rustfmt formatting in `src/current/elephant_melon.rs`. No renderer, model, animation, or prepared-resource input changed. The prior gait evidence therefore applies to the final renderer inputs without a new client launch.

## Independent species behavior check status

Passed evidence: the final candidate builds through managed `native-build`; `cargo test --offline --lib` passes all 20 tests; the previously recorded headless Elephant owner healing, consumption, max-health and save/reopen cases pass; all seven species have offscreen static rendered output; and Elephant has the movement sequence above. The tests are not evidence that all seven species' behaviors match Java.

The repository's per-species launcher behavior scripts (`launcher_elephant.py`, `launcher_giraffe.py`, `launcher_lion.py`, `launcher_rhino.py`, `launcher_tiger.py`, and `launcher_zebra.py`) call `horizon-ctl launch` directly. They are not used because they do not go through the required launcher-managed Play/cache-lease path. A managed final-candidate Play retry was:

```sh
HORIZON_LAUNCHER_HOME="$HOME/Library/Application Support/horizon-launcher" \
/Applications/Horizon.app/Contents/MacOS/horizon-launcher play \
  --instance app-2131-verify \
  --version native-0a875da-97b13615be3b \
  --world APP-2131-headless \
  --arg=--headless --arg=--control --arg=--resources \
  --arg="$HOME/Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3" \
  --arg=--render-distance --arg=4 --json
```

On launcher `0.2.82`, Play exited before startup with `Sign in with a Microsoft account that owns Minecraft: Java Edition to play.` `account list` showed cached account `LightTheTorch` (`c0afadba732a45779467cfe95babef58`) but reported `Could not check sign-in: Could not open the account lock.` The documented `horizon-launcher account refresh --json LightTheTorch` recovery returned `{"state":"retry","expires_at":null,"message":"Could not open the account lock."}`. No new species gameplay session was started and no lease check was bypassed.

Still unverified on the current candidate: family groups; species targeting and live AI movement; non-melon interaction rejection and remaining feeding/taming/mounting interactions; lifecycle including growth and breeding; drops; spawn predicates and variants; per-species save/reopen; LAN synchronization with distinct authenticated users; and offscreen movement sequences for Mammoth, Giraffe, Lion, Rhino, Tiger, and Zebra. These are separate from the Elephant melon cases and existing rendered static output.

## LAN identity blocker

APP-2126's successful two-client checkpoint is present under `/private/tmp/app2126-lan-630xzl4c`. Its host log records the LAN server and `APP2126Guest joined the game`; the host's local player file names the player `Player54777`. Those records establish that the test harness can create distinct local test names. They do not record distinct Microsoft account UUIDs or authenticated sessions, so they are not evidence of two owning accounts under managed Play.

The supported managed setup is two separate launcher-managed Play sessions on the pinned candidate, each with its own instance, control socket and signed-in account. `play --account <WHO>` selects a signed-in account name or UUID; launcher help says multiple games can play as multiple accounts. Use the same prepared 26.3 resources on both processes. For example, with `HOST_ACCOUNT` and `GUEST_ACCOUNT` set to different signed-in Java-owning accounts:

```sh
LAUNCHER=/Applications/Horizon.app/Contents/MacOS/horizon-launcher
RESOURCES="$HOME/Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3"
HORIZON_LAUNCHER_HOME="$HOME/Library/Application Support/horizon-launcher" HORIZON_CONTROL=/private/tmp/app2131-host.sock "$LAUNCHER" play --instance app2131-host --version native-0a875da-97b13615be3b --account "$HOST_ACCOUNT" --world APP-2131-LAN --arg=--headless --arg=--control --arg=--resources --arg="$RESOURCES" --arg=--render-distance --arg=4 --json
"$HOME/Library/Application Support/horizon-launcher/versions/native-0a875da-97b13615be3b/horizon-ctl" --socket /private/tmp/app2131-host.sock lan open 0 commands
HORIZON_LAUNCHER_HOME="$HOME/Library/Application Support/horizon-launcher" HORIZON_CONTROL=/private/tmp/app2131-guest.sock "$LAUNCHER" play --instance app2131-guest --version native-0a875da-97b13615be3b --account "$GUEST_ACCOUNT" --arg=--headless --arg=--control --arg=--resources --arg="$RESOURCES" --arg=--render-distance --arg=4 --json
"$HOME/Library/Application Support/horizon-launcher/versions/native-0a875da-97b13615be3b/horizon-ctl" --socket /private/tmp/app2131-guest.sock connect "127.0.0.1:${PORT}"
```

The host opens its test world to LAN; then the guest control socket connects to the reported host port. Both sessions must retain managed Play so cache leases are acquired. The same profile was rejected by the host as `That name is already taken`; therefore a distinct authenticated identity is needed, and the guest also needs a signed-in Microsoft account that owns Minecraft: Java Edition. The developer-offline launcher mode is single-player-only. APP-2126’s separate local names are not authentication. Its saved checkpoint logs show Horizon `0.1.0`, distinct cache directories and local names; they do not show distinct Microsoft UUIDs or prove this is a supported authenticated LAN result. No authentication bypass is used. LAN remains pending.

## Remaining coverage

The unverified cases are listed under “Independent species behavior check status.” Authenticated host/guest LAN remains open pending a second signed-in account that owns Minecraft: Java Edition.
