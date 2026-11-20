use super::Navigation;
use crate::mobs::Who;

pub const SNAPSHOT: u8 = 244;
pub const RAY_SNAPSHOT: u8 = 249;
pub const MAX_SCHOOL_SIZE: i32 = 24;

pub fn hold_retaliation(nav: &mut Navigation, attacker: Who) {
    *nav.__horizon_mixins_naturalist_piranha_retaliation_mut() = Some(attacker);
}

pub fn take_retaliation(nav: &mut Navigation) -> Option<Who> {
    nav.__horizon_mixins_naturalist_piranha_retaliation_mut().take()
}

pub fn has_player_retaliation(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_piranha_retaliation() == Some(Who::Player)
}
