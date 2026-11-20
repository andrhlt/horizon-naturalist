use super::Navigation;

pub fn cooldown(navigation: &Navigation) -> i32 {
    *navigation.__horizon_mixins_naturalist_bass_eat_cooldown()
}

pub fn set_cooldown(navigation: &mut Navigation, ticks: i32) {
    *navigation.__horizon_mixins_naturalist_bass_eat_cooldown_mut() = ticks;
}

pub fn tier(variant: &str) -> u8 {
    match variant {
        "naturalist:bass_medium" => 1,
        "naturalist:bass_large" => 2,
        _ => 0,
    }
}

pub const SMALLEST_SNAPSHOT: u8 = 240;

pub const MAX_SCHOOL_SIZE: i32 = 5;

pub fn snapshot_tier(seen: &crate::mobs::Seen) -> Option<u8> {
    if seen.species != crate::mobs::Species::Cod {
        return None;
    }
    seen.variant.checked_sub(SMALLEST_SNAPSHOT).filter(|tier| *tier <= 2)
}

#[derive(Clone, Debug)]
pub struct NaturalGroup {
    pub variant: String,
    pub target: u32,
    pub index: u32,
    pub leader_position: [f64; 3],
}

pub fn group(navigation: &Navigation) -> Option<&NaturalGroup> {
    navigation.__horizon_mixins_naturalist_bass_spawn_group().as_ref()
}

pub fn set_group(navigation: &mut Navigation, group: NaturalGroup) {
    *navigation.__horizon_mixins_naturalist_bass_spawn_group_mut() = Some(group);
}

pub fn take_group(navigation: &mut Navigation) -> Option<NaturalGroup> {
    navigation.__horizon_mixins_naturalist_bass_spawn_group_mut().take()
}
