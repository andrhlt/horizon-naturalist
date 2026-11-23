# APP-2135 integration checkpoint

Candidate source: `andrmholt/app-2135-naturalist-verify-full-integration` (isolated worktree), current commit `00d453cf3f14542f22a0290c9818e10b5767f419`.
The current source includes the refreshed handoff heads for ocean `bb021967`, freshwater/Alligator
`7f30ea2`, large animals `8d9f0c5`, insects `b042209`, Duck `fb6298a`, and reptiles/Tortoise
`8c4d833`, plus the shared-spawn source `29adc01`. The candidate artifact is
`native-0a875da-53ec73086c7c`; `libnaturalist.dylib` SHA-256 is
`1e0c4066aee3d3121fb706d7946d3b172d535e59dd3c6b8f688c24e908eedfed`.

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
| Ocean | bucket capture/release, aquatic placement/limits, swimmer goals, combat wrappers, Isopod hiding/shell/Cod food, Starfish color-drop fallback | native build; 32 Rust tests; freshwater persistence harness; six offscreen captures | headless capture/conservation, Ray landed-hit and Anglerfish cooldown, Whale/Isopod/Cod/Starfish behavior, LAN |
| Freshwater/coastal | Alligator breeding/laying, Capybara persistence/use, Hippo/Piranha/Catfish/Clam/Bass paths | persistence passed: all six UUIDs and saved fields matched after reopen; Capybara owner interaction passed; six offscreen frames passed | headless Alligator lay/stomp and aquatic LAN; cleanup emitted a non-fatal temporary-world “No world” message |
| Shared spawning | 36 age-group policies and serial carry wiring | native build; Rust/source checks | controlled loaded-chunk natural spawn and matching artifact runtime |
| Large animals | renderers, Giraffe interaction/control registration, Lion/Tiger interaction/death hook, Rhino/Zebra source | native build; Giraffe fixture launches but fails expected summoned Variant observation | Giraffe Variant/movement, Zebra/Rhino/Lion/Tiger gameplay; Rhino charge/shield behavior is not enabled because its injected module uses unsupported cohort fields |
| Insects | Butterfly/Dragonfly/Firefly placements, Chrysalis lifecycle, Snail queue/placement, Scorpion hooks | corrected fixture route launches; Chrysalis removal and adult cocoon pass, but baby cocoon is false and two butterflies hatch instead of one | fix lifecycle/count defects, then rerun Snail/Scorpion and offscreen; Firefly juvenile rendering remains APP-2159 |
| Duck | renderer/appearance, bucket/follow/breed/variant source | native build | headless follow/whistle/parent/save-reopen and offscreen capture |
| Reptiles | Komodo/Lizard/Mole/Tortoise source, Tortoise fall-on hook | native build; Rust checks | headless interactions/egg landing/save-reopen; Tortoise `stepOn` parity is unsupported by the pinned BlockPart API |
| Crab/Ant/Rat/Hedgehog | focused source handoffs are available (PRs #31/#32/#33) | not yet consumed in this candidate; Crab/Hedgehog runners hardcode missing artifact-local resources without the managed jar setup | consume exact heads, then shared runtime and persistence checks |

Runtime notes: `tools/launcher_managed.py` and the insect runner were corrected locally to resolve
resources from the candidate manifest's frozen base instead of an artifact-local directory; this
was required for the supported pinned route. The remaining failures above are gameplay assertions,
not authentication or cache-lease failures. Initial sandbox launches hit the normal cache-lease
permission boundary; the same commands passed with the approved managed escalation.

This is an integration checkpoint, not a claim of full Naturalist parity or final five-mod
certification. Missing ordinary gameplay and runtime evidence remain release blockers.
