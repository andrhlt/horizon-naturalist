use mod_api::{
    AttachmentType, BlockPart, Compound, FallOnEvent, Flow, Holder, LandingWrite, LevelRandomHost,
    RandomSource, RandomTickEvent, Tag,
};

/// Hatchlings waiting for the server tick after the egg block is removed.
/// Each row is x, y, z and the horizontal offset in hundredths of a block.
static PENDING_HATCHLINGS: AttachmentType<Vec<[i32; 4]>> = AttachmentType {
    namespace: "naturalist",
    name: "pending_alligator_hatchlings",
    default: Vec::new,
    codec: None,
    copy_on_death: false,
    sync: false,
};

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

        let mut pending = PENDING_HATCHLINGS
            .get(level, Holder::Level)
            .unwrap_or_default();
        for index in 0..eggs {
            pending.push([pos[0], pos[1], pos[2], 30 + 20 * index]);
        }
        let _ = PENDING_HATCHLINGS.set(level, Holder::Level, pending);
        let _ = level.remove_block(pos, false);
        Flow::Cancel
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let key = registrar.block_key("naturalist:alligator_egg");
    registrar.block_part(key, AlligatorEggPart);
}

pub fn hatch_tick(host: &mut dyn mod_api::ServerHost) {
    let Ok(mut pending) = PENDING_HATCHLINGS.get(&mut *host, Holder::Level) else {
        return;
    };
    let Some(row) = pending.first().copied() else {
        return;
    };
    let Some(level) = host.level("minecraft:overworld") else {
        return;
    };
    let mut nbt = Compound::new();
    nbt.insert("Age".into(), Tag::Int(-24_000));
    let pos = [
        f64::from(row[0]) + f64::from(row[3]) / 100.0,
        f64::from(row[1]),
        f64::from(row[2]) + 0.5,
    ];
    if host
        .summon(level.dimension_id(), "naturalist:alligator", pos, nbt)
        .is_some()
    {
        pending.remove(0);
        let _ = PENDING_HATCHLINGS.set(host, Holder::Level, pending);
    }
}
