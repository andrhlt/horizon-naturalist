use mod_api::{
    AttachmentCodec, AttachmentType, BlockKey, BlockPart, CanSurviveEvent, Direction, Flow, Holder,
    RandomSource, TickBlockEvent, UpdateShapeEvent,
};

static HATCHES: AttachmentType<Vec<[i32; 4]>> = AttachmentType {
    namespace: "naturalist",
    name: "pending_snail_hatches",
    default: Vec::new,
    codec: Some(AttachmentCodec {
        encode: encode_hatches,
        decode: decode_hatches,
    }),
    copy_on_death: false,
    sync: false,
};

#[derive(Clone)]
pub struct SnailEggs {
    egg: BlockKey,
}

#[mod_api::horizon_part]
impl BlockPart for SnailEggs {
    fn can_survive(
        &self,
        level: &dyn mod_api::LevelHost,
        event: &mut CanSurviveEvent,
    ) -> Option<bool> {
        let Some(state) = level.block_state([event.pos.x, event.pos.y - 1, event.pos.z]) else {
            return Some(false);
        };
        let Some(serialized) = level.state_string(state) else {
            return Some(false);
        };
        Some(!serialized.starts_with("minecraft:air"))
    }

    fn neighbor_changed(
        &self,
        level: &mut dyn mod_api::LevelTicksHost,
        event: &mut mod_api::NeighborChangedEvent,
    ) {
        if event.changed != self.egg {
            return;
        }
        let pos = [event.pos.x, event.pos.y + 1, event.pos.z];
        let Some(state) = level.block_state(pos) else {
            return;
        };
        if !level
            .state_string(state)
            .is_some_and(|name| name.starts_with("naturalist:snail_eggs["))
        {
            return;
        }
        // Java schedules 600..2399 ticks on placement. The callback's supported
        // mutable host exposes the queue but not the level RNG, so use a stable
        // position mix that stays inside Java's delay interval.
        let mix = pos[0].wrapping_mul(73_856_093)
            ^ pos[1].wrapping_mul(19_349_663)
            ^ pos[2].wrapping_mul(83_492_791);
        let delay = 600 + (mix.unsigned_abs() % 1800) as i32;
        let _ = level.schedule_block_tick(pos, self.egg, delay);
    }

    fn tick_block(
        &self,
        level: &mut dyn mod_api::LevelWriteHost,
        event: &mut TickBlockEvent,
    ) -> Flow {
        let pos = [event.pos.x, event.pos.y, event.pos.z];
        let Some(state) = level.block_state(pos) else {
            return Flow::Continue;
        };
        if !level
            .state_string(state)
            .is_some_and(|name| name.starts_with("naturalist:snail_eggs["))
        {
            return Flow::Continue;
        }
        let Some(mut random) = level.random() else {
            return Flow::Continue;
        };
        let mut count = random.next_int_bound(4).unwrap_or(0) + 2;
        count = count.clamp(2, 5);
        if !level.set_random(random) {
            return Flow::Continue;
        }
        let mut pending = HATCHES.get(level, Holder::Level).unwrap_or_default();
        if !pending.iter().any(|row| row[..3] == pos) {
            pending.push([pos[0], pos[1], pos[2], count]);
            if HATCHES.set(level, Holder::Level, pending).is_err() {
                return Flow::Continue;
            }
        }
        if !level.remove_block(pos, false) {
            return Flow::Continue;
        }
        Flow::Cancel
    }

    fn update_shape(
        &self,
        level: &mut dyn mod_api::LevelTicksHost,
        event: &mut UpdateShapeEvent,
    ) -> Option<mod_api::BlockState> {
        if event.direction == Direction::Down {
            let Some(serialized) = level.state_string(event.neighbour) else {
                return None;
            };
            if serialized.starts_with("minecraft:air") {
                return Some(event.neighbour);
            }
        }
        None
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let key = registrar.block_key("naturalist:snail_eggs");
    let part = SnailEggs { egg: key };
    registrar.block_part(key, part.clone());
    for tag in [
        "minecraft:dirt",
        "minecraft:base_stone_overworld",
        "minecraft:sand",
        "minecraft:logs",
        "minecraft:planks",
    ] {
        registrar.block_part_tag(tag, part.clone());
    }
}

pub fn hatch_queue(host: &mut dyn mod_api::ServerHost) -> Vec<[i32; 4]> {
    HATCHES.get(host, Holder::Level).unwrap_or_default()
}

pub fn save_hatch_queue(host: &mut dyn mod_api::ServerHost, pending: Vec<[i32; 4]>) {
    let _ = HATCHES.set(host, Holder::Level, pending);
}

fn encode_hatches(pending: &Vec<[i32; 4]>) -> Result<mod_api::Compound, mod_api::AttachmentError> {
    let flat: Vec<i32> = pending.iter().flat_map(|row| row.iter().copied()).collect();
    Ok(mod_api::Compound::from([(
        "rows".into(),
        mod_api::Tag::IntArray(flat),
    )]))
}

fn decode_hatches(data: &mod_api::Compound) -> Result<Vec<[i32; 4]>, mod_api::AttachmentError> {
    let Some(mod_api::Tag::IntArray(flat)) = data.get(&"rows".into()) else {
        return Err(mod_api::AttachmentError::Decode);
    };
    if flat.len() % 4 != 0 {
        return Err(mod_api::AttachmentError::Decode);
    }
    Ok(flat
        .chunks_exact(4)
        .map(|row| [row[0], row[1], row[2], row[3]])
        .collect())
}
