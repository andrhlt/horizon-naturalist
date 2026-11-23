use mod_api::{
    BlockPart, Compound, FallOnEvent, Flow, LandingWrite, LevelRandomHost, RandomSource,
    RandomTickEvent, Tag,
};
use std::sync::{LazyLock, Mutex};

/// Hatchlings waiting for the server tick after the egg block is removed.
/// Keep the dimension from the random-tick hook: Java creates the babies in
/// the same ServerLevel that owned the egg.
#[derive(Clone, Copy)]
struct PendingHatchling {
    dimension: &'static str,
    x: i32,
    y: i32,
    z: i32,
    offset: i32,
}

static PENDING_HATCHLINGS: LazyLock<Mutex<Vec<PendingHatchling>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub struct AlligatorEggPart;

fn property(state: &str, name: &str) -> Option<i32> {
    state
        .split(&format!("{name}="))
        .nth(1)?
        .split([',', ']'])
        .next()?
        .parse()
        .ok()
}

#[mod_api::horizon_part]
impl BlockPart for AlligatorEggPart {
    fn fall_on(&self, _: &mut dyn LevelRandomHost, event: &mut FallOnEvent<'_>) {
        // The block uses Horizon's turtle-egg landing rule, which already applies
        // mob_griefing, its 1-in-3 fall chance, and the Zombie/Bat exclusions.
        // Naturalist additionally protects a nest from its own Alligator type.
        if event.kind == "naturalist:alligator"
            && event.writes.iter().any(|write| {
                matches!(
                    write,
                    LandingWrite::SetBlock(_) | LandingWrite::DestroyBlock { .. }
                )
            })
        {
            event.writes.clear();
        }
    }

    fn random_tick(
        &self,
        level: &mut dyn mod_api::LevelWriteHost,
        event: &mut RandomTickEvent,
    ) -> Flow {
        let pos = [event.pos.x, event.pos.y, event.pos.z];
        let Some(state) = level
            .block_state(pos)
            .and_then(|state| level.state_string(state))
        else {
            return Flow::Continue;
        };
        if !state.starts_with("naturalist:alligator_egg[") {
            return Flow::Continue;
        }

        // The Java AlligatorEggBlock advances on stone as well as sand. Its
        // turtle-egg template's native tick requires sand, so own the lifecycle
        // here instead of letting the template silently reject valid nests.
        let Some(hatch) = property(&state, "hatch") else {
            return Flow::Continue;
        };
        let Some(eggs) = property(&state, "eggs") else {
            return Flow::Continue;
        };
        let Some(mut random) = level.random() else {
            return Flow::Continue;
        };
        if random.next_int_bound(3).unwrap_or(1) != 0 {
            return Flow::Cancel;
        }

        if hatch < 2 {
            let _ = level.set_block_property(pos, "hatch", &(hatch + 1).to_string(), 3);
            return Flow::Cancel;
        }

        let Ok(mut pending) = PENDING_HATCHLINGS.lock() else {
            return Flow::Continue;
        };
        for index in 0..eggs {
            pending.push(PendingHatchling {
                dimension: event.dimension.key().id(),
                x: pos[0],
                y: pos[1],
                z: pos[2],
                offset: 30 + 20 * index,
            });
        }
        let _ = level.remove_block(pos, false);
        Flow::Cancel
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let key = registrar.block_key("naturalist:alligator_egg");
    registrar.block_part(key, AlligatorEggPart);
}

pub fn hatch_tick(host: &mut dyn mod_api::ServerHost) {
    let Ok(mut pending) = PENDING_HATCHLINGS.lock() else {
        return;
    };
    let Some(row) = pending.first().copied() else {
        return;
    };
    let mut nbt = Compound::new();
    nbt.insert("Age".into(), Tag::Int(-24_000));
    let pos = [
        f64::from(row.x) + f64::from(row.offset) / 100.0,
        f64::from(row.y),
        f64::from(row.z) + 0.5,
    ];
    if host
        .summon(row.dimension, "naturalist:alligator", pos, nbt)
        .is_some()
    {
        pending.remove(0);
    }
}
