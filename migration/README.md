# Current-nightly migration investigation

The `main` branch is the historical 0.7.4-preview.45 port, not a current
`.hmod`. This branch is an unfinished migration to Horizon commit
`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`. The development native build has been tested, but complete gameplay parity and release readiness are not claimed.

Run `python3 migration/state_inventory.py` to locate every direct accessor of
the old injected state. The legacy manifest declares 54 fields on Navigation,
one on AvoidEntity and one on SoundEngine. The report counts direct accesses,
not indirect callers of wrappers; those callers also need review.

The current game owns mob attachment registrations through
`Mob::attachment_lifetime`; that field is lifecycle metadata, not mod storage.
Entity attachments or entity Part state can own per-mob values where a valid
entity and host are available. Values previously serialized separately, such as
the bass feeding cooldown, must continue to round-trip through the same save
semantics. Unsaved navigation state must retain its prior reset behavior.

A mechanical replacement is unsafe: old helpers often take only
`&Navigation`, and `Navigation` is cloned and used for rider redirection.
`EntityPartState` returns `HolderBusy` while an entity is out of its list for
its own tick, so that API cannot simply replace state reads in the existing AI
loop. Trace each wrapper to its mob owner and call context; move suitable
behavior into a per-mob Goal (whose instance owns its state) or another API
that actually provides its holder, and test save, unload, entity replacement,
riding and client synchronization where the behavior requires them. Never key
state by the address of a movable entity or reuse an unscoped numeric entity ID.
The sound map and AvoidEntity goal state need their own lifecycle analysis.

The current game re-exports mob code from `horizon-client-lib`; its lower-crate functions still need replacement markers. Resolve legacy targets against the pinned function catalog and verify each injection by building it. Content
conversion must use an external `[[jar]]` reference rather than redistribute
upstream's reserved resources.

## Development content stage

`python3 tools/package_current.py /path/to/naturalist-2.0.7-neoforge-26.3.jar /tmp/naturalist-content-dev.hmod` builds an API 0.2 content-only development package. The deterministic package has version `0.8.0-dev.1` and `side = "both"`. It uses the historical content generators but includes generated `data/naturalist/horizon/` registry definitions, biome spawn modifiers and conventional biome tags. It does not contain the original jar's files; copy the exact original jar to the game's `jars/<sha512>.jar` cache. The jar's SHA-512 is declared in the manifest. This is a development dependency, not a release: the content-only form contains no native gameplay or rendering.

On Horizon `0a875da`, an isolated headless world accepted all 51 generated entity IDs and loaded 1,804 items versus 1,658 in the unmodded base, a difference of 146. Entity templates are placeholders: deer uses wandering trader, and 31 attributes are omitted from content definitions because the current loader rejects them on the creature template; the package lists every omitted value in `README-omitted-attributes.txt`. The unified native project restores those values through its attribute declarations; correct entity behavior still needs the gameplay port. The current loader skips Naturalist's custom `add_animals` modifier; generated standard modifiers cover the recorded spawn-table fixture, and conventional tags supply the swamp tag. The content-only stage cannot provide the custom net recipe serializer.

For the native metadata experiment, run `python3 migration/generate_metadata.py`, then pass `--native-project migration/unified` to `package_current.py`. This creates one combined native/content development package; it still needs the original jar in the local hash cache. On the pinned client build, a diagnostic run matched all 48 supplied Java mob profiles for category, dimensions, and the eight attributes those fixtures record. The metadata source also includes dirt trail, which has no matching fixture; carried food and duck egg are not handled as mobs. For bird, butterfly, dragonfly, firefly, and hedgehog the attribute registrar uses a wandering-trader row so it can preserve the fixture’s missing attack attribute; known selected-template defaults are added explicitly. For every species, values absent from the fixtures remain unverified against Java. Food, behavior, variants, and rendering need separate ports.

## Combined development integration

`migration/unified` combines the metadata and spawn routing prototype with partial Bass and Deer implementations and capture-net support for selected species. Build with `--native-project migration/unified`. It is not a complete Naturalist gameplay port. In a headless diagnostic world, 396 Naturalist rows in the supplied biome fixture matched category, weight and group sizes after routing. A clean unified build also initialized alongside Wildlife Tuning; the ant hill features remain unsupported.
