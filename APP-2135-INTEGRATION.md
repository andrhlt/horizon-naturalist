# APP-2135 integration checkpoint

Candidate source: `andrmholt/app-2135-naturalist-verify-full-integration` (isolated worktree), current commit `2d6d57a` plus diagnostic fixture updates.
The current source includes the refreshed handoff heads for ocean `bb021967`, freshwater/Alligator
`7f30ea2`, large animals `8d9f0c5`, insects `b042209`, Duck `fb6298a`, and reptiles/Tortoise
`8c4d833`, shared-spawn source `29adc01`, Crab `356508a`, Ant `107c67e`, and
Hedgehog `845dfa5`, with integration fixes in `ab19708`, `b8c6463`, `422a264`,
`e66b34c`, `d22af89`, Shark `e07f553`, Starfish `83c181a`, Jellyfish `bb81363`,
and corrected Giraffe/insect fixtures `fa0d59e`/`a8077cf`. The latest candidate is
`native-0a875da-2c623e51691d`.

Pinned runtime inputs:

- Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, MC `26.3`
- SDK `sdk.json` SHA-256 `a0ec3d4b8651b3e9a322e8ec35a5917bc8f290ac904ade5bcd5bd5b5aacdae84`
- SDK surface `e05637790d2a91aaa8075e748e2ca872`, mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`
- Rust `1.99.1`; launcher `/Applications/Horizon.app/Contents/MacOS/horizon-launcher`

Supported combined checks:

```sh
CARGO_NET_OFFLINE=true cargo test --lib --quiet
/Applications/Horizon.app/Contents/MacOS/horizon-launcher native-build \
  --version nightly-2026-11-20-0a875da --trust-native-code .
```

Both commands pass on this candidate (`32/32` Rust tests; native build/install succeeds).
The supported offscreen route is through `tools/launcher_selected_headless.py` or the family
launchers, passing `--offscreen`; no accountless-client flag or auth bypass is part of setup.

| Family | Implemented source in candidate | Verified here | Remaining gate / known gap |
|---|---|---|---|
| Ocean | bucket capture/release, aquatic placement/limits, swimmer goals, combat wrappers, Isopod hiding/shell/Cod food, Starfish support/drop, Shark cooldown, Jellyfish pulse | native build; source checks; prior six offscreen captures | headless capture/conservation, Ray/Anglerfish, Whale/Isopod/Cod/Starfish/Shark/Jellyfish behavior, LAN |
| Freshwater/coastal | Alligator breeding/laying, Capybara persistence/use, Hippo/Piranha/Catfish/Clam/Bass paths | persistence passed: all six UUIDs and saved fields matched after reopen; Capybara owner interaction passed; six offscreen frames passed | headless Alligator lay/stomp and aquatic LAN; cleanup emitted a non-fatal temporary-world “No world” message |
| Shared spawning | 36 age-group policies and serial carry wiring | native build; Rust/source checks | controlled loaded-chunk natural spawn and matching artifact runtime |
| Large animals | renderers, Giraffe interaction/control registration, Lion/Tiger interaction/death hook, Rhino/Zebra source | corrected fixture shows raw `Variant=0`, mounted control flag false, and mounted movement false; dismount/flag clear pass | restore Giraffe variant/control/movement; Zebra/Rhino/Lion/Tiger gameplay; Rhino charge/shield remains disabled because its injected module uses unsupported cohort fields |
| Insects | Butterfly/Dragonfly/Firefly placements, Chrysalis lifecycle, Snail queue/placement, Scorpion hooks | corrected fixture proves exactly one Butterfly after hatch and duplicate window; adult Age is `0`, entity persists, but NoAI is `1b` again after 16 ticks and no cocoon is placed | fix adult lifecycle/NoAI reversion, then rerun Snail/Scorpion/offscreen; Firefly juvenile rendering remains APP-2159 |
| Duck | renderer/appearance, bucket/follow/breed/variant source | native build | headless follow/whistle/parent/save-reopen and offscreen capture |
| Reptiles | Komodo/Lizard/Mole/Tortoise source, Tortoise fall-on hook | native build; Rust checks | headless interactions/egg landing/save-reopen; Tortoise `stepOn` parity is unsupported by the pinned BlockPart API |
| Crab/Ant/Rat/Hedgehog | Crab/Ant/Hedgehog focused handoffs consumed; Rat remains from prior source | native rebuild passes after pinned API fixes; Crab fixture reaches gameplay but its `naturalist_fixture_crab` probe is not registered; Hedgehog reaches gameplay but `naturalist:hedgehog` is absent from the staged inventory | add/use supported Crab probe and verify Hedgehog item registration/resource-pack path; then run Ant/Rat persistence and host/guest checks |

Runtime notes: `tools/launcher_managed.py` and the insect runner were corrected locally to resolve
resources from the candidate manifest's frozen base instead of an artifact-local directory; this
was required for the supported pinned route. The remaining failures above are gameplay assertions,
not authentication or cache-lease failures. Initial sandbox launches hit the normal cache-lease
permission boundary; the same commands passed with the approved managed escalation.

The latest integrated runtime attempt also exposed two compile defects in the focused handoffs
(missing scoped `DamageLevel` and an explicit Ant yaw cast); both are fixed and the fresh native
build passes. The Crab/Hedgehog fixture failures are currently harness/registration observations,
not counted as gameplay passes or definitive species defects.

The latest persistence rerun passed on `native-0a875da-2c623e51691d`: all six freshwater UUIDs
and saved fields matched after reopen, and Capybara owner interaction succeeded. Temporary-world
cleanup again emitted only the non-fatal launcher “No world” message.

Adult cocoon diagnostic details: after the live `Age:0`/`NoAI:0b` setup and 16 ticks,
`Age` remained `0`, `NoAI` read back as `1b`, and `entity @e[tag=fixture_adult]` remained
present. The target at `[9,100,8]` remained air; `[10,100,8]` and `[10,99,8]` remained
the two oak-log support blocks. The resulting fixture was
`adult_makes_cocoon=false`, `caterpillar_discarded=false`, `chrysalis_faces_log=false`,
with exactly one Butterfly after hatch and after the duplicate window.

This is an integration checkpoint, not a claim of full Naturalist parity or final five-mod
certification. Missing ordinary gameplay and runtime evidence remain release blockers.
