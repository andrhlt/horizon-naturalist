use mod_api::{BlockPart, CanSurviveEvent, UpdateShapeEvent};

pub struct SnailEggs;

#[mod_api::horizon_part]
impl BlockPart for SnailEggs {
    fn can_survive(&self, level: &dyn mod_api::LevelHost, event: &mut CanSurviveEvent) -> Option<bool> {
        let Some(state) = level.block_state([event.pos.x, event.pos.y - 1, event.pos.z]) else {
            return Some(false);
        };
        let Some(serialized) = level.state_string(state) else { return Some(false); };
        Some(!serialized.starts_with("minecraft:air"))
    }

    fn update_shape(&self, level: &mut dyn mod_api::LevelTicksHost, event: &mut UpdateShapeEvent) -> Option<mod_api::BlockState> {
        if matches!(event.direction, mod_api::Direction::Down | mod_api::Direction::Up) {
            let Some(serialized) = level.state_string(event.neighbour) else { return None; };
            if serialized.starts_with("minecraft:air") {
                return Some(event.neighbour);
            }
        }
        None
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let key = registrar.block_key("naturalist:snail_eggs");
    registrar.block_part(key, SnailEggs);
}
