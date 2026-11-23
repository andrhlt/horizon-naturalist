use mod_api::{AttachmentCodec, AttachmentType, BlockPart, CanSurviveEvent, Direction, Holder, ItemPart, OnBlockPlacedByEvent, RandomSource, ServerHost, UpdateShapeEvent};

static HATCHES: AttachmentType<Vec<[i64; 4]>> = AttachmentType {
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
pub struct SnailEggs;

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


pub struct SnailEggItem;

#[mod_api::horizon_part]
impl ItemPart for SnailEggItem {
    fn on_block_placed_by(&self, host: &mut dyn ServerHost, event: &mut OnBlockPlacedByEvent<'_>) {
        let Some(level) = event
            .placer
            .and_then(|placer| placer.level(host))
            .or_else(|| host.level("minecraft:overworld"))
        else {
            return;
        };
        let Some(now) = level.game_time(host) else {
            return;
        };
        let Some(delay) = level.with_random(host, |random| {
            random.next_int_bound(1800).unwrap_or(0) + 600
        }) else {
            return;
        };
        schedule_hatch(host, event.pos, now + i64::from(delay));
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let key = registrar.block_key("naturalist:snail_eggs");
    let part = SnailEggs;
    registrar.block_part(key, part.clone());
    let item = registrar.item_key("naturalist:snail_eggs");
    registrar.item_part(item, SnailEggItem);
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

pub fn schedule_hatch(host: &mut dyn mod_api::ServerHost, pos: [i32; 3], due: i64) {
    let mut pending = HATCHES.get(host, Holder::Level).unwrap_or_default();
    pending.retain(|row| row[..3] != pos.map(i64::from));
    pending.push([i64::from(pos[0]), i64::from(pos[1]), i64::from(pos[2]), due]);
    let _ = HATCHES.set(host, Holder::Level, pending);
}

pub fn hatch_queue(host: &mut dyn mod_api::ServerHost) -> Vec<[i64; 4]> {
    HATCHES.get(host, Holder::Level).unwrap_or_default()
}

pub fn save_hatch_queue(host: &mut dyn mod_api::ServerHost, pending: Vec<[i64; 4]>) {
    let _ = HATCHES.set(host, Holder::Level, pending);
}

fn encode_hatches(pending: &Vec<[i64; 4]>) -> Result<mod_api::Compound, mod_api::AttachmentError> {
    let flat: Vec<i64> = pending.iter().flat_map(|row| row.iter().copied()).collect();
    Ok(mod_api::Compound::from([(
        "rows".into(),
        mod_api::Tag::LongArray(flat),
    )]))
}

fn decode_hatches(data: &mod_api::Compound) -> Result<Vec<[i64; 4]>, mod_api::AttachmentError> {
    let Some(mod_api::Tag::LongArray(flat)) = data.get(&"rows".into()) else {
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
