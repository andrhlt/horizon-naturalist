use super::Navigation;

pub const COOLDOWN_TICKS: i32 = 2400;
pub const EAT_TICKS: i32 = 8;
pub const SNAPSHOT: u8 = 243;
pub const ANGLERFISH_SNAPSHOT: u8 = 250;
pub const CLAM_SNAPSHOT: u8 = 229;
pub const STARFISH_SNAPSHOT: u8 = 251;

pub fn cooldown(nav: &Navigation) -> i32 {
    *nav.__horizon_mixins_naturalist_catfish_cooldown()
}

pub fn set_cooldown(nav: &mut Navigation, ticks: i32) {
    *nav.__horizon_mixins_naturalist_catfish_cooldown_mut() = ticks;
}

pub fn prey(nav: &Navigation) -> Option<(u32, i32)> {
    *nav.__horizon_mixins_naturalist_catfish_pending_prey()
}

pub fn set_prey(nav: &mut Navigation, prey: Option<(u32, i32)>) {
    *nav.__horizon_mixins_naturalist_catfish_pending_prey_mut() = prey;
}
