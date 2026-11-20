# Current-nightly migration investigation

The `main` branch is the historical 0.7.4-preview.45 port, not a current
`.hmod`. This branch is an unfinished migration to Horizon commit
`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`. No gameplay parity or current
native build is claimed.

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

Current Horizon moved the mob implementation into the `horizon-client` game
crate. Resolve old `horizon_client_lib` paths against the current game and its
function index rather than assuming every old path is still in a lower crate.
Other lower-crate targets must meet the current replacement rules. Content
conversion must use an external `[[jar]]` reference rather than redistribute
upstream's reserved resources.
