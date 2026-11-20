use super::Navigation;

pub const SNAPSHOT: u8 = 248;
pub const PULSE_INTERVAL: i32 = 20;

pub fn pulse_cooldown(nav: &Navigation) -> i32 {
    *nav.__horizon_mixins_naturalist_jellyfish_pulse_cooldown()
}

pub fn set_pulse_cooldown(nav: &mut Navigation, ticks: i32) {
    *nav.__horizon_mixins_naturalist_jellyfish_pulse_cooldown_mut() = ticks;
}
