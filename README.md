# Naturalist for Horizon

> **Status:** This is the legacy 0.7.4-preview.45 port, pinned to Horizon
> `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`. Its historical macOS and LAN
> verification is described below. Migration to the current `.hmod` native
> format is incomplete: the preview relies on native state fields that current
> Horizon rejects, and lower-crate injections must use replaceable targets.
> The historical builder embeds original Java resources in its output; keep that
> output local and do not redistribute it. Original resources are separately
> licensed; see [LICENSE-Naturalist](LICENSE-Naturalist).


This legacy wildlife preview targets the official Naturalist 2.0.7 NeoForge
release on Minecraft 26.3 and Horizon Nightly `9f42064`. The launcher
package includes all 47 wildlife species and the detached Lizard Tail.

## Deer development build

```sh
~/dev/horizon-naturalist/tools/play-local.sh
```

The script uses the pinned Horizon Nightly's prepared resources and opens
an isolated world named **Naturalist Deer**. The local pack is in
`.local/content` and the built client is in `dist/horizon-client`.

## Horizon launcher preview

The private catalog at `.local/launcher-preview/local-index.json` installs
Naturalist 0.7.4-preview.45. Use Install, Build, and Play to open it on
Horizon 26.3. The preview includes the detached Lizard Tail. The preview includes adult and fawn models, apple feeding, and fawn grazing. The
brewing replay matched all nine Java Forest Dasher recipes, and drinking the
strong potion gave Speed III and Weakness II. The launcher regression also verified
venison drops and cooking. Java-format deer variants match the reference
for biome selection, explicit and legacy NBT values, breeding, and reload.
A rendered replay verified the white coat on the integrated host and LAN
guest and checked inheritance when a spawn egg is used on an adult deer.
The panic replay matched Java for the extended water search, tie order, and
random destination bounds. The herd replay matched the Java panic timers
for melee, ownerless arrows, and lava, including the timer difference
when a nearby deer ticks before the injured deer. Same-tick AI reactions
during overlapping melee and lava damage were replayed against Java. Deer retain an empty bucket,
and the breeding replay produced fawns and calves only from same-species
parents. Adult and fawn shadow radii follow the Java renderer. All six
original deer voices registered and passed a playback dispatch fixture.
Fawn parent eligibility matched Java in the Follow Parent goal fixture.
The client received the original adult and fawn hurt and death sounds
during actual damage. The latest preview also follows Java's sprint
threshold, adult and fawn collision geometry, grazing animation timer,
and animation clock. A reload test verified the deer predator entity tag
with a world data pack. A headless movement replay captured Java’s stone and grass footsteps. Deer scale matched Java adult and fawn geometry and
retained a permanent modifier after reopening the world. Normal data edits
matched Java’s live attribute merging behavior.

Headless duck replays matched Java's attributes, adult and duckling age
and health, egg timing, default variant, loot, taming, sitting, dyeing,
healing, whistle behavior, and duckling bucket pickup and release.
Thrown duck eggs matched the Java throw sound and impact particle count.
A headless replay of 2,304 actual impacts observed 274 single-duckling and
13 four-duckling hatches. The Java source uses a one-in-eight hatch roll and
a second one-in-32 roll for four ducklings.

Bass matched Java's default attributes and all three saved size variants.
Bird matched Java's default attributes, the robin, jay, and cardinal
variant IDs. The pinned Java release ignores command-supplied Bird age fields
and omits them from saves. Both use models derived from the release source. Bird seed selection, consumption cooldown, and ownerless foraging passed
against the Java reference. A juvenile followed the first eligible adult in
the headless flock replay. Both engines left tamed birds on the ground when
they overlapped their owners. In the pinned Java release, the perch goal
tries to mount the owner, but Minecraft rejects players as vehicles. A large
bass hunted a smaller one in
the headless replay, discarded the prey, dropped bone meal, and entered
cooldown. The cooldown persisted after reopening the world. School size, tier
selection, and growth matched Java in the headless comparison. Bucket
capture, release, dispenser release and legacy variant restoration passed.
Bass natural formations matched the Java tier-specific sizes and
per-pass cap in the seeded headless spawning fixture.
Boar, turkey, and capybara passed the headless attribute, feeding, breeding,
and fall comparisons. Boar lightning conversion and capybara pet behavior
passed. The insect replay covered cocoons, chrysalis hatching, pollination,
and butterflies breeding a caterpillar. The preview also includes both
scorpions, catfish, piranha, lizards, Komodo dragons, dragonflies and fireflies.

The LAN replay matched duck dye and sitting and fawn scale across two
headless clients. Deer matched Java in configured spawners and same-tick
projectile hits. Bird playback matched the daytime calls and the sampled
nighttime silence. The mob-disable settings in `config/naturalist-server.toml`
matched Java and took effect on live reload. The Capture Net replay
verified both Java save formats through real survival interactions.

Rhino matched the shield charge, stun recovery and plant destruction with
mob griefing enabled. Blobfish matched pressure conversion and bucket
round trips. Jellyfish matched its 20-tick swimming impulse, contact damage
and bucket behavior. Ray matched melee poison timing, hit rejection and both
bucket formats. Vulture matched the seven measured cases, including scavenging
and stealing one held item from a low-health player without damaging them.
Both bears matched the sampled food, taming, breeding, shearing and
berry, hive, and campfire raids. The campfire theft timing and the cubs’
flower-sniff sound cadence matched the stepped Java replays.

Anglerfish matched 17 Java checks, including depth-sensitive spawning,
prey-triggered glow, cooldown and buckets. The loaded spawn entries matched
Java across all 67 vanilla biomes for the compared Naturalist mobs, the
vanilla farm animals and the forest Fox and Rabbit additions. The stock
spawning loop enforced the measured aquatic caps. Frogs consumed Dragonflies
and Fireflies and received the 60-tick glowing effect after a Firefly meal.

The juvenile health cap, live data merging, damaged-juvenile health fraction
on maturation, and transient serialization matched Java for seven species.
The 33-species preview survived saving and reopening a headless world,
and a late-joining LAN guest received the roster and removal events.
Jellyfish damaged both players. The Catfish, Piranha and Blobfish midwater
bucket interaction passed for both players. All eight fish bucket labels
matched the Java samples. Headless LAN replays verified that Antivenom
splashes and clouds from either player's thrown potion cleared poison on
both players and left regeneration intact.

The Butterfly group replay matched Java's shared two-color palette across
128 controlled groups. Headless chrysalis placement refused a stone face
and kept the item on both client and server. An oak face accepted it with
the correct orientation. Both Bears restored actual targets after loading,
and Boar and both Bears matched universal anger on a two-player LAN.

Starfish matched 18 Java observations, including bucket round trips, colored
loot and waterlogged placement. Giant Isopod matched 23 observations,
including sight-dependent hiding, damage reduction and a LAN bucket round
trip. Clam matched the shell, injury, cooldown, launch, and treasure checks
in a headless Java comparison.

The mob profile replay matched Java’s sampled base attributes, collision
dimensions, eye height and spawn categories. Scaling retained its modifiers
through live data edits. Headless Bird poses matched 144 Java samples across
idle, walking, flight and landing. Vulture matched 144 Java model poses
including the blend into and out of perching. Fireflies spawned at midnight
in the controlled swamp and their glow state reached the tracking client.

The adult and juvenile profile replays matched Java for all 33 species and
the detached lizard tail, including what the tracking client receives.
Bird kept its Java health cap when a raw age was supplied or edited.
Boar used the standard Pig footstep at both ages, and the land footstep
replay matched Java on stone. Dragonflies and detached Lizard Tails
matched Java's zero experience reward.

Snail matched the wall-to-ceiling crawl and all 33 sampled model poses,
including body tilt and tail lag. Live feeding, breeding, egg hatching,
dyeing and Capture Net round trips passed. Crab matched 47 Java observations
covering five variants, weapons, taming, capture and release, and other
behavior. A headless LAN guest tamed and round-tripped an owned yellow
Crab with a damaged axe. Blobfish targeted a live Crab and ignored Cod
in the same test. Vulture's perch and flight transition reached the
tracking client.

A Snail Shell preserves its color, rotation and stored plant across a world
reopen. Mining, command destruction and piston movement produced the
colored shell and the exact plant in the headless Java replays.
All 16 dyes passed actual guest crafting. A late LAN guest received the
placed Shell’s exact color, orientation, and stored flower. The optional
crush behavior matched Java in the local and LAN fall replays, including
the Feather Falling, named, persistent and Capture Net release controls. The
live-data tests matched breeding state and its absence for all 31 installed
entity types. Actual hurt sound category and volume matched the seven Java
species tested.

Zebra matched its loaded adult and juvenile dimensions, fixed-seed spawn
attributes, and feeding and taming outcomes. An actual LAN guest put on
its chest and saddle, used its inventory, rode, jumped and dismounted.

Giraffe matched the sampled feeding, taming and healing interactions.
The terrain solver matched 48 Java samples on stone and slabs. Actual
host and guest riding passed, including the Java seat height and the
guest's steering and dismounting.

Alligator matched 27 Java observations and nine live prey, egg defense
and aggression cases. Four eggs stacked and hatched on stone, and all
four babies tracked across LAN. Controlled trampling matched Java's
Cow, Alligator, and mob-griefing-disabled cases.

The loaded variant pools matched Java for entry order and weights across
all 67 Minecraft biomes. Ordinary spawn selection matched 1,008 seeded
variant draws and 12 follow-up level RNG values against Java. Exact
randomness across full natural spawn passes, group finalization, and
further behavior and visual coverage remain under comparison.

Generate the private package and local catalog from the pinned jar with
`python3 tools/package_launcher.py /path/to/naturalist.jar .local/launcher-preview`.
Start the launcher with `HORIZON_MOD_INDEX` pointing to that absolute
`local-index.json` path, then use Install, Build, and Play. This catalog is
local to your machine. The package contains assets from your Java jar.

Headless probes that need internal state have their native test hooks in
`tools/reference/horizon`. Append that directory’s manifest fragment and
copy its Rust files into `launcher/` in a scratch copy before packaging a
fixture version. The Bird, Vulture, Snail and Giraffe pose replays also replace the matching
files in `src/` with their counterparts in `tools/reference/horizon/render-fixtures`.
The variant-pool replay uses the test copy of `deer_variants.rs` from the same
directory in `launcher/`. For the Clam launch-motion replay, switch the manifest’s `clam-tick` helper to
`clam_tick_motion_fixture.rs` in that scratch copy. The normal package omits
all of these hooks.

## Reproduce the build

Make a fresh copy of Horizon at `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`.
Use the [published NeoForge jar](https://www.curseforge.com/minecraft/mc-mods/naturalist/files/9280579).
The preparation script verifies its SHA-512, extracts the local assets and data,
and applies the engine changes to the copy.

```sh
python3 tools/prepare.py /path/to/naturalist.jar .local/content --engine /path/to/staged-horizon
cd /path/to/staged-horizon
cargo build --locked --profile fast -p horizon-official
```

Use the matching Nightly's prepared Minecraft resources. Launch the resulting
client with `--dev-mod /absolute/path/to/.local/content`. This build uses a
source overlay on the pinned engine. Run the build against a fresh source copy.

## Java reference and Horizon replay

`tools/java_reference.py` runs the installed NeoForge reference server in an
isolated world and captures attributes, saved state, avoidance, breeding, and
all nine Forest Dasher brewing outcomes.
`tools/horizon_replay.py` repeats those checks in Horizon and reopens the
world. `tools/client_smoke.py` exercises the rendered client, apple feeding,
the spawn egg, LAN tracking, death drops, and venison cooking.

```sh
python3 tools/java_reference.py /path/to/isolated-neo-server /path/to/jdk-27 fixtures/deer-java.json
python3 tools/horizon_replay.py /path/to/horizon-ctl /path/to/horizon-official /path/to/resources .local/content fixtures/deer-java.json
```

Replay the Java-compatible deer variant pack using the same harness.
Use `--grazing` and `fixtures/deer-grazing-java.json` on both commands
to compare the fawn's grazing, grass conversion, and growth.

```sh
python3 tools/java_reference.py /path/to/isolated-neo-server /path/to/jdk-27 fixtures/deer-variants-java.json --variants
python3 tools/horizon_replay.py /path/to/horizon-ctl /path/to/horizon-official /path/to/resources .local/content fixtures/deer-variants-java.json --variants
```

The Java harness also supports `--behavior` with
`fixtures/deer-behavior-java.json`. This uses a temporary test mod to
probe the original herd timer and standing or sneaking target eligibility.
`tools/deer_herd_probe.py` compares the standalone Horizon overlay against this
fixture. It currently reports a difference in when nearby deer inherit a panic
timer. The Java deer share it during the damage call. The older standalone
overlay relays it on the next tick. The launcher preview includes the
phase-aware implementation described above.

Replay brewing on a launcher-built version with
`python3 tools/launcher_deer_smoke.py /path/to/native-version fixtures/deer-java.json`.

The model and keyframe tables come from the release-era Java source at
`1471124ae26bcbf90a2106a1f68080d1dd03a53a`. To regenerate them, run
`python3 tools/convert_models.py /path/to/Naturalist src/deer_models.rs`.
The source license is included as `LICENSE-Naturalist`. Original textures
and sound files are supplied by each player's local jar.

## Parity scope

The deer source supplies the attributes, goal priorities, spawn rules, potion effects and
animation data. The release jar supplies the shared test reference and the
original textures, sounds, loot and cooking recipes. The local build targets
the default deer and Java-format third-party variant packs. The variant replay
checks biome tags, priority, weight-zero exclusion, the global fallback, old
numeric IDs, unknown IDs, inheritance, and persistence after reload. The
rendered client smoke test's `--appearance` mode captures the default and
custom coat on the host and the custom coat on a LAN guest. It also checks
that a spawn egg used on an adult produces a fawn with the adult's variant. Brewed potion identities match Java for all three
strengths and bottle types. A rendered smoke test verified the strong potion's
effects and preserved its identity in a LAN guest's inventory. Exact spawn rates, movement paths and
animation timing across the two engines have not been measured.

## Full wildlife roster preview

Naturalist 0.7.4-preview.5 passed the stock Horizon launcher’s Install,
Build, and headless Play on Nightly 9f42064. All 47 wildlife species
survived save and reopen, paired with a headless LAN guest, and were
removed from that guest after server removal. The preview also summoned
the detached Lizard Tail and Dirt Trail and retained Duck Egg support.
All 2,296 installed content files matched the archive byte for byte.

The adult profiles and adult and juvenile geometry matched Java for all
48 wildlife and tail entities. The loaded spawn tables matched across 67 biomes.
A direct headless ray comparison confirmed the Java juvenile Mole hitbox
boundaries. The Wolf started digging after acquiring the Mole, which
remained underground on tick 49 and emerged on tick 50. Elephant and
Mammoth passed the original inventory actions on host and LAN guest.
Rat passed offspring ownership, guest whistle handling, and cargo
persistence and drops. Ostrich matched the tested breeding, egg laying,
hatching, rider jump, and egg-break anger and retained its anger target
after reopening. All 22 Froglass and Shellstone blocks loaded, three
Dragonfly variants produced the Java Froglass drops, and a LAN guest
crafted all three sampled Shellstone recipes.

Full 1:1 Java parity remains in progress. Open comparisons include random
draw ordering during natural spawn groups, some animation and sound
paths, and additional original content.

### Effects update

Naturalist 0.7.4-preview.6 passed stock headless Play and all 47 entities
survived reopening and late LAN tracking. Wolves played the original
13 substrate sounds over the 50-tick Mole extraction. Ostrich egg
trampling played the original sound for both LAN players and recorded
the landing player as the attacker. Whale spray followed the original
look direction and sent the same positioned packets to a nearby guest.

## Snail breeding and cooking preview

Naturalist 0.7.4-preview.7 passed the stock launcher’s Install, Build,
and headless Play. All 47 wildlife species survived save and reopen
and appeared for a late LAN guest. All 2,308 installed content files
matched the package.

Two Snails now produce one gravid parent with no parent age cooldown,
matching Java. A Snail and a Cow both remain in love without breeding.
The merged build also passed egg hatching and invalid food checks.

Duck Eggs work in cakes and pumpkin pies. The cooked egg works in
the furnace, smoker, and campfire and provides 5 food and 6 saturation.
Hide smelts into Leather in a furnace.

## Shark and Whale multipart preview

Naturalist 0.7.4-preview.8 passed stock Install, Build, and headless
Play. All 47 wildlife species matched through save, reopen, and late LAN,
and the installed 2,308 content files matched the package.

The detached Shark and Whale hitboxes now use Java's float geometry
and trigonometry. The isolated geometry fixture matched all 390 measured
box faces. The merged stock headless build confirmed both animals'
detached contacts for the host and LAN guest, including simultaneous
contacts on opposite sides of the same head. Guest motion packets matched
the Java impulses after protocol quantization.

## Ant Carried Food preview

Naturalist 0.7.4-preview.9 passed stock Install, Build, and headless
Play. All 47 wildlife species survived save and reopen and reached a
late-joining LAN guest. The installed 2,309 content files matched
the package.

The Carried Food entity now matches the headless Java land and water
checks. An unattached stack stays stationary through tick 60 and becomes
a regular pickup-delay-10 item on tick 61. A real carrying Ant preserves
the same food UUID and position offset after a world reopen. With 260
ordinary items and the parallel tick verifier enabled, the Carried Food
stayed still and an ordinary submerged item still moved.

## Plush Bear and Wild Ones preview

Naturalist 0.7.4-preview.10 passed stock Install, Build, and headless
Play. All 47 wildlife species survived save and reopen and reached a
late LAN guest. All 2,326 installed content files matched the package.

The Plush Bear matches the tested Java recipe, four placements and
collision shapes, and bare-handed mining drop. Ten Java rest checks
passed both solo and on a LAN guest. A Plush held in either hand allows
rest around ordinary hostile mobs. An angry Zombified Piglin still
blocks rest. Wild Ones played in the jukebox, produced comparator
strength 15, and dropped from a Fox spawned with an egg in the merged
build.

## Shark player targeting preview

Naturalist 0.7.4-preview.11 passed stock Install, Build, and headless
Play. All 47 wildlife species survived save and reopen and reached a
late LAN guest. All 2,326 installed content files matched the package.

The Great White Shark now finds the submerged player when a closer
player is on dry land. The real headless LAN comparison confirmed the
Java target choice with the host and guest in both roles, including the
control where the submerged player was closer.

## Duck Egg and fish bucket defaults

Naturalist 0.7.4-preview.12 passed stock Install, Build, and headless
Play. All 47 wildlife species survived save and reopen and reached a
late LAN guest. The materialized item registry matched the pinned Java
stack sizes, durability, rarity, and nutrition for all 141 items in
this preview.

Duck Eggs now stack to 64. Nine Naturalist fish buckets no longer inherit
the vanilla Cod Bucket's food component. The Bass variant and health
round trip and the three midwater bucket interactions still passed in
the merged headless build.

## Capture Net preview

Naturalist 0.7.4-preview.13 passed stock Install, Build, and headless
Play. All 47 wildlife species survived save and reopen and reached a
late LAN guest. The registered defaults of all 144 packaged items
matched the Java stack, durability, rarity, and nutrition checks.

The Capture Net now captures and releases vanilla Bees, Silverfish,
and Endermites with the original mob UUID. A mounted Bee produces Dirt
and takes no net durability. Capturing Cod drops a Cod Bucket and uses
the first Water Bucket in inventory order, including the off hand.
Creative still requires the ingredient and leaves it in the inventory.
All these capture and release checks passed solo and on LAN.

## Complete original item registry preview

Naturalist 0.7.4-preview.15.2 passed stock Install, Build, and headless
Play from an older native preview. The build refreshed nine outdated
brewing aliases and preserved the previous version. All 47 wildlife
species survived save and reopen and reached a late LAN guest. All
2,368 installed content files matched the archive.

All 146 registered item defaults matched the pinned Java stack size,
durability, rarity, and nutrition measurements. The archive includes
all 114 original recipe files, 47 entity loot tables, 30 block loot
tables, and 146 item definitions byte for byte.

Glow Goop matched the tested layer, light, outline, wet placement,
mining, and replacement behavior. Its furnace, smoker, campfire, and
Spectral Arrow recipes passed in live stations. The navigation replay
confirmed that a Cow can path through the Goop layer. The tooltip
option keeps its Java default and reloads on the client.

The Knapsack matches the tested Java capture and release behavior in
either hand, including saved names, UUIDs, health, age, baby vehicles
and their passengers, and oversized item stacks. The three-Hide recipe
passed. Captured data survived reopening the world, and the same
physical interactions passed for a LAN guest.

## Piranha player targeting preview

Naturalist 0.7.4-preview.16 passed stock Install, Build, and headless
Play. All 47 wildlife species survived save and reopen and reached a
late LAN guest. All 2,368 installed content files matched the archive.

Piranhas now acquire an eligible submerged player even with a nearer
dry player. Both headless LAN roles passed actual bites and the
farther-dry control. A separate fixed-position measurement confirmed
the first target was the swimmer in both roles.

Preview 17 corrects Alligator and Hippo player targeting. Daytime Alligators and Hippos detect eligible submerged players even when a dry player is closer. At night, an Alligator can select a nearer visible dry player. Nest defense keeps its higher Java priority, and a Hippo excludes its owner. The merged preview passed the stock headless launcher workflow, direct LAN contact checks, and save/reopen with all 47 wildlife types delivered to a late guest.

Preview 18 corrects Java parent selection at the real entity-box boundary and prevents cross-species mating through borrowed vanilla AI templates. Headless Java fixtures matched initial parent pursuit and family outcomes. Later wandering can affect exact steering. A real LAN guest fed a Rat and a Cow without producing offspring, then fed two Rats that produced a baby with the expected owner. Egg-bearing controls preserved Java's age and love state. The merged build passed stock Install, Build and Play, then a save/reopen and late LAN check covering all 47 wildlife types.

Preview 19 fixes Duck downward motion with AI disabled and preserves the dyed layer during the Java hurt flash. The merged installed build matched all seven reference motion samples for both enabled and disabled AI and passed adult egg laying, juvenile egg exclusion, and the loot check. The exact rendering implementation matched 480 Java poses and the dye and base layers' overlay behavior in a diagnostic build. Eight direct comparison suites and the 47-species save/reopen and late LAN checks passed on the clean merged build.

Preview 20 carries the exact player identity when a Piranha follower copies its leader's target. If a player hurts the follower, both fish retaliate against that attacker as in Java. The fixed-placement comparison verified both host/guest role assignments and a prior attack against the leader. In a live natural-school LAN replay, both Piranhas advanced toward the swimmer and injured them while the nearer dry player stayed at full health. The merged preview passed stock Install, Build and Play and the saved-world late LAN replay for all 47 wildlife types.

Preview 21 corrects family identity for Elephant, Mammoth and Giant Isopod and prevents vanilla Cows from breeding with Hippos, Alligators or Moles. Elephant calves can follow adult Mammoths, matching the Java class relationship. The reverse direction remains ineligible. The Java sprint condition now applies to unmounted Elephants and Mammoths during sufficiently fast active paths, with exact early coordinates in the headless comparison. The movement-speed command readback and nonplayer rider controls have separate outstanding differences. The merged preview passed stock Install, Build and Play, four direct Java suites, and saved-world late LAN tracking for all 47 wildlife types.

Preview 22 fixes Firefly grass-hiding flight, grass particle spread, and daylight and fixed-light decisions. The clean merged LAN replay matched the pinned Java flight positions and particle origins exactly, with disappearance on tick 27 for the two adults and tick 30 for the juvenile. Both fixtures emitted three bursts and three neutral hide sounds. All three Fireflies stayed visible under the twilight canopy without a hide packet, and the Nether brightness control suppressed bright-room glow while preserving glow in darkness. The stock headless launcher and saved-world late LAN roster also passed.

Preview 23 reports the live Elephant and Mammoth sprint modifier through the movement-speed command. Both species matched Java's sprint and ordinary values in the merged headless replay, and their opening movement positions remained exact. The launcher Install, Build and Play check and the full 47-species save/reopen and late LAN check passed.

Preview 24 finalizes naturally spawning Deer with the level's random stream. Variant selection, juvenile rolls, follow-range modifiers, and handedness use the pinned Java draw order. The seeded headless forest comparison matched 33 real accepted Deer, including 23 group followers. Ordinary world spawning also matched the sampled family draws. The clean merged build produced a four-Deer group with the expected saved variant and attribute fields, passed stock Install, Build and Play, and retained all 47 species through save/reopen and a late LAN guest. Chunk-generation finalization is tracked separately.

Preview 25.2 makes Deer chunk-generation finalization use the same region random stream and single-precision input rounding as Minecraft 26.3 Java. It also applies the corrected modifier rounding to ordinary natural spawning. All 640 captured region member rows matched, including both random implementations and the tagged custom coat data pack. The clean merged build generated a real three-Deer group before the first periodic creature spawn pass and preserved a sampled deer's UUID, coat, modifier, and handedness across reopen. The projectile herd and saved scale checks passed. Stock Install, Build and Play and all 47 wildlife types through late LAN tracking also passed.

Preview 26 suppresses Dragonfly water-entry splash particles and sound to match Java. In the merged headless LAN replay, all 63 position and velocity samples remained exact. The Dragonfly emitted no water-entry particles or splash sound on either view. The vanilla pig control retained its bubbles, splashes, and guest sound. The stock launcher check and the saved-world late LAN roster passed.

Preview .27 makes vanilla Cow calves follow Cow parents and ignore nearby Pigs, Snakes, and Dirt Trails. The herd replay also verified that an injured Elephant calf alerts adult Mammoths, while an injured Mammoth calf alerts its own species and leaves Elephants calm.

Preview .28 matches Java's Bird collision. Birds do not add an outward push, can still be displaced by other mobs, and take vanilla cramming damage unless protected. Perching and flap reactions passed in the headless replay.

Preview .29 matches the Java Alligator’s transition from water targeting to nest defense. Physical LAN tests covered host and guest swimmers, a dry player approaching a newly placed egg, and same-tick melee with the Java damage and cooldown. Adjacent Piranha swim combat and Hippo owner protection passed again.

Preview .30 uses the Java calf animation clips and movement blend for Mammoth calves. The headless pose replay compared body, trunk, and ear angles across 24 idle poses on the calf model with the Java and former adult clips, with a maximum difference of 4 × 10⁻⁹ radians. The pose replay uses the pinned Java model and the Horizon renderer’s animation tables and evaluator.

Preview .31 matches Java when a Chicken pilots an Elephant or Mammoth calf. Both vehicles stay in place, keep the passenger’s seat at the Java offset, and still move towards a parent after the pilot is absent. Vultures now apply eaten-item effects and return containers, including honey bottles and stew bowls, and respect chorus-fruit teleportation.

Preview .32 finalizes Deer created by ordinary summon commands and spawn eggs with Java's world random stream. The seeded replay compares coat, follow-range modifier, handedness, age, yaw, and the next random draw. Explicit summon NBT retains the Java behavior of skipping spawn finalization.

Preview .33 preserves the actual attacker ID when Elephants and Mammoths retaliate or defend an injured calf. Headless LAN checks covered host and guest attackers with a nearer bystander. The mob remains angry at the original UUID and retains its saved expiry if the attacker crosses a Nether portal, matching Java.

Preview .35 matches the original Java tracking settings for all 51 Naturalist entity types. It also uses Java's placement precision and world random stream for ordinary Deer spawners. The seeded tests covered spawners with default and empty weighted potentials and mobs with explicit spawn NBT.

Preview .37 preserves Bird plumage chosen during chunk generation. The headless taiga replays matched the Java Blue Jay and custom data-pack variants, and each tracked Horizon Bird retained its UUID and plumage on reopen. Birds also matched the measured Java takeoff and retreat response to Survival and Creative players. The reference and Horizon worlds generated different Bird counts.

Preview .38 makes Capybaras follow their actual LAN owner when another player is closer. Ten headless host and guest replays sampled the active goals every tick. The owner-follow goal ran only when enabled, and the sit and temptation controls kept their own goals.

Preview .40 follows the pinned Java creature modifier order and Bird generation placement rules. Bird weights, group sizes, and modifier IDs are preserved. Across 67 compared biome tables, the tested creature rows match Java in order. In headless frozen world generation, seed 49903 produced two Birds at the same coordinates in both games, and seed 49904 produced zero in both. Default and custom plumage still survived a world reopen.

## Private release candidate 0.7.4-preview.45

The stock Horizon launcher freshly installed this archive, built it with
the Performance profile, and opened it through headless Play on Apple
silicon macOS. The exact archive is 9,046,394 bytes with SHA-256
`d5912dcc22defab0387ffa133920eb8ec6de6fe0c5e7966f82f53b412ffdb151`.
The built client identifies as `native-9f42064-14fe6532043b`. All 2,369 installed
content files matched the archive. Stock Play spawned all 51 registered
entity types. In the .40-to-.45 world upgrade, all 47 wildlife UUIDs
survived and appeared for a late LAN guest. Their removal reached the
guest after death. A purple Snail Shell retained its rotation and stored
flower on upgrade and produced the expected drops.

Bird chunk generation matches Java's group draw order in the tested
samples. The diagnostic replay matched 64 groups and 320 Birds for
plumage, transient age, follow range, handedness, and the next random
value. The fixture-free build matched the compared creature rows in 67
biomes and the Bird positions and counts in both frozen test regions.
Birds omit the same age fields as Java after summon, edit, and reopen,
with Deer retaining their ages in the same test.

The 39-case diagnostic replay covered the sampled behaviors and
separate checks covered Elephant and Mammoth LAN inventory and Firefly
glow. The Lizard feeding check missed a heal during the parallel run. The
same fixture and two instrumented runs passed in isolation. The original
missed interaction remains unexplained. The clean build also passed the seven supplemental gameplay replays,
including Ant Hill drops, Butterfly groups, Ostrich riding, and Rat LAN
interactions.

The archive contains assets from the original Java release and includes
the upstream notice. The [upstream license](https://raw.githubusercontent.com/starfish-studios/Naturalist/master/LICENSE)
reserves the rights to the listed resource directories. Redistribution
of bundled assets needs a verified basis. Interactive visuals and
audible playback remain for the joint test. Windows and Linux are
unverified. Other Java behavior remains unverified, including live,
periodic Bird group finalization.
