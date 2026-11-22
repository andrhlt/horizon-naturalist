use mod_api::{AttachmentType, BlockPart, Flow, Holder, RandomSource, RandomTickEvent};

pub static PENDING: AttachmentType<Vec<[i32; 3]>> = AttachmentType {
    namespace: "naturalist",
    name: "pending_chrysalis_hatch",
    default: Vec::new,
    codec: None,
    copy_on_death: false,
    sync: false,
};

pub struct Chrysalis;

#[mod_api::horizon_part]
impl BlockPart for Chrysalis {
    fn random_tick(&self, level: &mut dyn mod_api::LevelWriteHost, event: &mut RandomTickEvent) -> Flow {
        let pos = [event.pos.x, event.pos.y, event.pos.z];
        let Some(state) = level.block_state(pos) else { return Flow::Continue; };
        let Some(serialized) = level.state_string(state) else { return Flow::Continue; };
        if !serialized.starts_with("naturalist:chrysalis[") { return Flow::Continue; }
        let age = serialized.split("age=").nth(1).and_then(|value| value.split(',').next().or_else(|| value.split(']').next())).and_then(|value| value.parse::<i32>().ok());
        let Some(age) = age else { return Flow::Continue; };
        if age < 3 && level.random().is_none_or(|mut random| random.next_int_bound(5).unwrap_or(1) != 0) { return Flow::Continue; }
        if age >= 3 {
            let mut pending = PENDING.get(level, Holder::Level).unwrap_or_default();
            if !pending.contains(&pos) { pending.push(pos); }
            let _ = PENDING.set(level, Holder::Level, pending);
            let _ = level.remove_block(pos, false);
            return Flow::Cancel;
        }
        let _ = level.set_block_property(pos, "age", &(age + 1).to_string(), 3);
        Flow::Cancel
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let key = registrar.block_key("naturalist:chrysalis");
    registrar.block_part(key, Chrysalis);
}
