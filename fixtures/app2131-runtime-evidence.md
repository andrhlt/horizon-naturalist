# APP-2131 runtime evidence

Status: partial evidence for review. APP-2131 remains open. Elephant/Mammoth interactions and Giraffe feeding/taming/mount checks were run headlessly on managed developer-offline candidates. Elephant rider placement, dismount, cake taming, chests, saddle, shearing and owner healing passed. Giraffe feeding, juvenile taming, healing and boarding passed; after fixing the Java string variant and mounted-control lifetime, APP-2135's source-matched candidate also passed mounted movement, dismount and control-flag clearing. Rhino shield stun with mob griefing disabled failed on an older candidate. Zebra feeding/equipment ran; that older candidate allowed Zebra×Donkey breeding and used a 0.5 jump strength where Java expects 0.7. Lion/Tiger runtime behavior and these updated Rhino/Zebra source fixes still need source-matched gameplay checks. Authenticated host/guest LAN remains separately pending. This does not claim full species parity.

## Candidate identity

- Branch: `codex/app2131-large-animal-renderers`
- Horizon: `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` (26.3)
- Launcher: `0.2.82`
- Rust: `1.99.1`; optimized profile
- Installed managed candidate: `native-0a875da-04848d315bac`
- Developer base: `developer-e05268bea02f4384b113d9b9b02f0ce6310883561ccc41fea947e097148ade16`
- Candidate Naturalist package SHA-256: `ac346ce6f245f8d78865ba2e78ef485d943c3757edc6b9d7c28a413f4feda496`
- Candidate `libnaturalist.dylib` SHA-256: `a6c05a3afeca085e8d9549d0624cb0da3edf476f173ec15c669d80b507e7c8c2`
- Candidate renderer/interaction source matches this checkout for Elephant, Giraffe, metadata and renderer registration; the candidate manifest differs only by an extra blank line from this checkout.
- Java reference: Naturalist 2.0.7 NeoForge 26.3.0.95, source commit `1471124ae26bcbf90a2106a1f68080d1dd03a53a`

The candidate was built through launcher-managed `native-build` with launcher `0.2.82`, optimized Rust `1.99.1`, and selected through launcher-managed `play`. Its manifest records the pinned Horizon revision, developer base, package hash and library hash above. Gameplay was headless and started only through the launcher-managed helper with the prepared 26.3 resources path; no managed binary was launched directly.

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

The seven current large-animal renderers (Elephant, Mammoth, Giraffe, Lion, Rhino, Tiger and Zebra) remain registered in the source and were rendered on the pinned 26.3 runtime; the static seven-species capture is [`app2131-large-animals-offscreen.png`](app2131-large-animals-offscreen.png), SHA-256 `8628dbcaa7d5773d93fd16afd4c430c8707505b31355ed10bf45b8af09e2a52a`. Its renderer, model, animation and prepared-resource inputs match the final candidate as documented below. The movement candidate `native-0a875da-9aa2d80bd80f` produced an offscreen Metal frame and an eight-frame live Elephant movement sequence. The final gameplay candidate is `native-0a875da-97b13615be3b`. The sequence is [`app2131-elephant-gait.gif`](app2131-elephant-gait.gif), SHA-256 `b086091dbb34866a4ea954f338c13dade70d19d32a53011082024909063b87fd`.

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

That runtime reported Metal at 2560×1440 and 93,577 rendered frames. Offscreen status recorded zero user-originated input. The state field `pointer_grabbed: true` is the runtime’s internal offscreen grab: pinned source `pointer_capture.rs::internal_grab` returns true for `--offscreen` and routes around native `set_cursor_grab` and `set_cursor_visible`; `window_handler.rs` creates the offscreen window invisible and inactive. A final-candidate static seven-species frame was also inspected from the managed offscreen session; its temporary screenshot file was removed when that session ended. A repeat attempt against final candidate `native-0a875da-97b13615be3b` failed before startup with: `Sign in with a Microsoft account that owns Minecraft: Java Edition to play.` No visible client was launched. The candidate comparison below establishes that the prior gait sequence uses the final package’s same renderer inputs. The pinned runtime's source and `docs/agent-control.md` specify that this mode does not present or activate a window and does not take, hide or move the system pointer. No visible client or desktop input automation was used.

## Renderer and gait input comparison

The captured `9aa2d80bd80f` movement candidate and final `97b13615be3b` candidate use the same renderer and animation source inputs. `src/current/large_renderer.rs` is SHA-256 `e85e3ce23030bfa8b86ce3f2cb8392817129bc7dd5c30eed7c485bcf6806ab52`; model inputs are `elephant_models.rs` `eb1ad0848b50dc73e14bb7b7a54920c6ed986b01d65b6ccc7b3ecf71a48afd26`, `mammoth_models.rs` `1756f0a088cf10f9bacc8e4e42ab5d4fed2f69eead7163b43eea7f9402639009`, `giraffe_models.rs` `55a6fc7561a88fd629b5b8275d8518a75952dceaed170f2e25a7f3eea702465a`, `lion_models.rs` `384dc91ae3a1692dfc983d25a70a491334f66c551d8907c798a8d7d806e6406b`, `rhino_models.rs` `891a239e13ef954067f82942255fe7b29a7670fe14cd5e1c9a5a9bdc51390493`, `tiger_models.rs` `863716fbd66e1b9b295b09155e1f07c5755b4e6b29d51c1eb1353c7e6b789dfe`, and `zebra_models.rs` `0d91c60d5bdebf2912ae12041a9717cc9b2c364fdae8a132ba5abef7fe4030e5`. The corresponding animation inputs are Elephant `8beac72f7a8e9aec64bd9d69343d40077cabec13b57620179e0061c1371687ac`, Giraffe `48ca79f558f373c8fdd9343747274ddccf725750f251a3bc7b13292de73a126d`, Lion `952e89379f98f18b4152ee93bde3aa980b46d6107a2025269dd3a4be73fd1656`, Rhino `0e67d818c068a2f2530cf9510adce9fb94cbaa35e5e4da4a662f169001157441`, Tiger `3008e3958c008a3613488ee7d50f7e1fa6d38be9e82aa52363a492ebcc5223ec`, and Zebra `41585a82f5ee8830e757b28ec5743985b5e4995738dbb27275c99bf26c25b5cc` (all files under `src/`). Mammoth uses the Elephant animation input. Both candidates are Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, optimized Rust `1.99.1`, and the same prepared 26.3 resources at `nightly-2026-11-20-0a875da/.local/horizon-client/26.3`. The package and library hashes differ between the builds; the only source edit between them was rustfmt formatting in `src/current/elephant_melon.rs`. No renderer, model, animation, or prepared-resource input changed. The prior gait evidence therefore applies to the final renderer inputs without a new client launch.

## Independent species behavior check status

Previously recorded evidence: the former `97b13615be3b` candidate builds through managed `native-build`; its recorded `cargo test --offline --lib` passed all 20 tests; Elephant owner melon healing, consumption, max-health and save/reopen cases passed; all seven species have offscreen static rendered output; and Elephant has the movement sequence above. The gameplay findings in this update are on the newer managed developer candidate `04848d315bac`; they are not evidence that all seven species' behaviors match Java.

The per-species behavior harnesses start through `tools/launcher_managed.py`, which invokes launcher-managed Play with `--headless`, a private control socket, and the manifest-derived prepared 26.3 resources path. The helper accepts any installed manifest-declared base and retains the launcher's managed cache leases. Developer-offline Play still requires an eligible developer-base candidate; a nightly-base candidate does not become offline-eligible merely by setting the developer-offline environment variable. No harness invokes `horizon-client` directly or bypasses cache leases. The six harnesses cover Elephant/Mammoth interactions and spawn rules, Giraffe taming/feeding/riding, Lion and Tiger owner/AI behaviors, Rhino charge/stun, and Zebra feeding/equipment/breeding.

An earlier candidate `native-0a875da-97b13615be3b` was built on `nightly-2026-11-20-0a875da`, so it was not eligible for the developer-offline workflow. Its ordinary managed Play retry was:

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

On launcher `0.2.82`, ordinary Play exited before startup with `Sign in with a Microsoft account that owns Minecraft: Java Edition to play.` `account list` showed cached account `LightTheTorch` (`c0afadba732a45779467cfe95babef58`) but reported `Could not check sign-in: Could not open the account lock.` The documented refresh also returned `{"state":"retry","expires_at":null,"message":"Could not open the account lock."}`. This lock error is distinct from entitlement failure and does not establish whether a second account is required for authenticated LAN. The supported APP-2134 offline flow requires rebuilding this package against installed developer base `developer-e05268bea02f4384b113d9b9b02f0ce6310883561ccc41fea947e097148ade16`, with the matching prepared 26.3 resources under that base. An attempted managed native-build was deferred while another repository's launcher build held the launcher's shared build lock. No lease or authentication check was bypassed.

The integrated APP-2135 candidate `native-0a875da-e30de3ca2236` declares and has its prepared 26.3 resources under installed base `nightly-2026-11-20-0a875da`; its library hash is `868bd804153d72815fa2fa3da27c917bb8e2047d31503f9105b1a7147b19c471`. The Zebra harness used the managed launcher command through `tools/launcher_managed.py` with `HORIZON_LAUNCHER_DEVELOPER_OFFLINE=1`, headless control, and that base's resources. Play exited before the world started with `Sign in with a Microsoft account that owns Minecraft: Java Edition to play.` This is a launch eligibility/authentication result, not a Zebra gameplay failure. APP-2135 confirmed the installed developer base has the frozen Horizon revision and matching exporter manifest, but a different `horizon-ctl`; no source-matched Naturalist developer-base build/runtime is claimed yet. Zebra, Lion, Tiger and updated Rhino runtime checks therefore remain pending a candidate eligible for the documented local offline flow. The target nightly artifact excludes Rhino charge-marker fix `ae3b78e`, so it cannot verify that fix even if launch eligibility were resolved.

Managed-helper results on `native-0a875da-04848d315bac`:

- Elephant and Mammoth: five-cake baby taming, survival consumption, owner melon heal, chest/saddle, owner mount, Java rider heights, shift dismount, shears and live melee fixture passed.
- Giraffe: adult breeding food, juvenile tame attempt, owner healing and owner boarding passed. The harness observed no player-controlled movement while mounted; this is an actual behavior defect. A direct hook to `horizon_client::mobs::equine::controlled` is unsupported by the pinned function index (`E_MIXIN_TARGET_MISSING`) and was removed.
- Rhino: after adding the Java-required nearby calf, facing the adult toward the player, and targeting its current entity ID, the no-grief shield-charge check still did not observe `StunTick`. The adult moved away from the player and `StunTick` was absent, so the Rhino charge/blocked-shield behavior is missing or not wired in this candidate; the corrected fixture no longer points to the earlier calf/facing setup as the cause.
- Lion and Tiger harnesses call inspection commands absent from the installed candidate; those checks remain unverified rather than failed gameplay. Zebra's runtime interaction output was retained; exact age/RNG probes were removed from the managed run.
- Zebra actual output against Java: all ten selected food/health/temper cases matched; chest and saddle interaction passed; same-species breeding passed; crossbreeding with a Donkey produced a baby (`can_mate` observed `[true, true, false]`, Java `[true, false, false]`); jump strength was `0.5` versus Java `0.7`; the entity NBT had no `Variant` field. A source change now sets the Zebra jump-strength base to `0.7`; it is not in candidate `04848d315bac` and awaits the shared integration build.

The matching developer candidate is source-matched to the current Elephant/Giraffe interaction and renderer files as checked in `horizon-native-mods/naturalist`; its package identity is recorded above. Reuse the previously captured Elephant gait only for unchanged renderer/model/animation inputs.

Still unverified: family groups; Lion/Tiger ordinary interactions and targeting; Rhino charge implementation; Zebra cross-species breeding correction, variant persistence and new jump-strength source change; Giraffe mounted movement; other species' growth/lifecycle, drops and save/reopen; authenticated LAN synchronization; and offscreen movement for the six species without gait evidence. Static seven-species output and Elephant gait are already retained.

Focused managed-headless commands for integration reuse:

```sh
CANDIDATE="$HOME/Library/Application Support/horizon-launcher/versions/native-0a875da-04848d315bac"
python3 tools/launcher_elephant.py "$CANDIDATE" fixtures/elephant-java.json --live-melee
python3 tools/launcher_giraffe.py "$CANDIDATE" fixtures/giraffe-java.json
python3 tools/launcher_rhino.py "$CANDIDATE" fixtures/rhino-java.json
python3 tools/launcher_zebra.py "$CANDIDATE" fixtures/zebra-java.json
```

All four commands use `tools/launcher_managed.py`; Rhino currently exits on the no-grief stun assertion. Zebra was changed to use real held-item inputs for its breeding scenario and to report unavailable age/RNG/variant probes as explicit coverage gaps rather than calling absent fixture commands.

## APP-2135 Giraffe Variant and rider follow-up

On candidate `native-0a875da-53ec73086c7c` / integrated source `1ccec01`, corrected runner `fa0d59e` exposed two real Giraffe defects. The earlier `b65b4ce` runner had queried lowercase `variant` and expected `minecraft:temperate`, so that initial failure was invalid. The corrected run read uppercase `Variant`; Java expects `naturalist:giraffe`, while the candidate returned numeric `0`. Java's `DataDrivenVariantAnimal` writes a namespaced string at `Variant`; Horizon's Horse-backed Giraffe serializer was emitting its coat index there. Commit `2749649` now overrides Giraffe save NBT with the preserved namespaced string (defaulting to the only pinned Java Giraffe variant, `naturalist:giraffe`).

APP-2135 integrated `2749649` and passed the focused headless Giraffe gate on fresh candidate `native-0a875da-5a8e87e09f34`, Horizon `0a875da` / MC 26.3. `Variant="naturalist:giraffe"`, owner mount, mounted control flag, mounted movement, dismount, and control flag clearing all passed. The built library SHA-256 is `fdb5c4807b251414d4afa1abfa3eb9e9d004f2e202667e3ac167679ce48d3a62`; APP-2135 recorded this in PR #34 at `d1d6022`. The rider fix accounts for players in `Game.riding` / the passenger list; checking only `Mob.rider` had cleared the virtual saddle on the first mounted tick. This confirms the focused Giraffe gate, not the other APP-2131 species gates or authenticated LAN.

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

## Pod B source-matched candidate — launch eligibility

Pod B built `native-0a875da-2cb7d9690c66` from source snapshot `f21814f7a488eb0a8df8ab0bc93377f39ea068b4` (source-tree SHA-256 `ba4821322f395cbddc382db8a8fcfc99f6a8a2476cb0d06177a06b7899a0d892`), Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, MC 26.3, Rust 1.99.1. Its manifest uses base `nightly-2026-11-20-0a875da`; `libnaturalist.dylib` SHA-256 is `c00477439be3ed852ab95710c6170c2481fd56703b97ecb9379d5cc2de01ab74`. This is the Pod B integrated package, including the APP-2131 compatibility fixes through Zebra installer commit `1d2cc9d` (represented by the equivalent source snapshot commit `f21814f`).

The managed headless `launcher_rhino.py` launch against this candidate did not reach the world. Launcher version is 0.2.82. Exact command argument vector (with the recorded temporary socket path):

```text
["/Applications/Horizon.app/Contents/MacOS/horizon-launcher", "play", "--instance", "app-2131-verify", "--version", "native-0a875da-2cb7d9690c66", "--world", "Naturalist Rhino Fixture d50e73ae", "--json", "--creative", "--arg", "--headless", "--arg", "--control", "--arg", "--control-socket", "--arg", "/private/var/folders/kh/01bq8xn56w555wp18nv342m40000gn/T/nat-rhino-qsmobh0i/rhino.sock", "--arg", "--resources", "--arg", "/private/tmp/pod-b-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3", "--arg", "--test-terrain", "--arg", "--render-distance", "--arg", "4"]
```

It exited 1 with `Sign in with a Microsoft account that owns Minecraft: Java Edition to play`. Setting `HORIZON_LAUNCHER_DEVELOPER_OFFLINE=1` does not make a nightly-based descendant eligible: the pinned launcher CLI permits that mode only for an installed developer-provenance base and blocks LAN hosting in that mode. I also tried the documented APP-2134 offline `horizon-ctl launch` route with the candidate's matching prepared resources; the lease manager refused with `this managed installation has unregistered cache directories; use Horizon Launcher Play from the same launcher data directory with the intended instance and installed version`. No direct game binary launch or lease/auth bypass was used.

This is a launch-eligibility result, not a species gameplay result. Next action: Pod B must prepare the same source snapshot through managed `native-build` against installed developer base `developer-e05268bea02f4384b113d9b9b02f0ce6310883561ccc41fea947e097148ade16`, then publish its exact candidate/resources manifest for the headless species checks. Rhino charge/stun, Lion/Tiger gameplay, Zebra breeding/jump, save/reopen and authenticated LAN remain unverified on the Pod B source-matched candidate.

## Pod B developer candidate — focused behavior and persistence

The managed developer candidate `native-0a875da-a595fffd11c1` (source `90e23ca80d7e090cac9f4c131c77eece98116ce0`, Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, MC 26.3, Rust 1.99.1) contains the Giraffe cleanup registration from `5486c66`. Using `NATURALIST_TEST_LAUNCHER_HOME=/private/tmp/pod-b-dev-launcher NATURALIST_TEST_INSTANCE=horizon TMPDIR=/private/tmp python3 tools/launcher_giraffe.py /private/tmp/pod-b-dev-launcher/versions/native-0a875da-a595fffd11c1 fixtures/giraffe-java.json` through launcher-managed headless Play passed feeding, juvenile taming, owner healing, mounting, mounted movement, dismount and control-flag clearing.

A second launcher-managed headless Play session reopened the same deterministic world (`APP2131-Elephant-Persistence-5`) on that candidate. Elephant owner UUID, `Age=0`, `Health=56`, `ChestedHorse=1`, saddle equipment and `TamingFood=5` were present after reopen and matched the pre-save state; the managed run exited 0. The test helper accepts `NATURALIST_TEST_WORLD_NAME` to select that same world for the reopen session while preserving launcher Play and managed cache leases. This is representative Elephant ownership, health, taming-progress and equipment persistence, not a full persistence matrix.

Developer-offline mode is single-player-only, so authenticated LAN remains pending a second signed-in owning account. The earlier seven-species offscreen rendered frames and Elephant movement sequence remain reusable because renderer/model/animation inputs are unchanged; latest-candidate interactive LAN and a new all-species visual capture were not run.
