use crate::mobs::{Mob, MobWorld, PlayerView};
use super::Navigation;
use horizon_core::geometry::Vec3;

pub const KIND: &str = "naturalist:giant_isopod";

#[derive(Clone, Debug, Default)]
pub struct State {
    pub nearby: bool,
    pub hold: u8,
    pub was_hiding: bool,
}

pub fn player_near(mob: &Mob, player: &PlayerView) -> bool {
    player.alive && !player.creative && !player.spectator
        && mob.distance_sqr(player.position) <= 9.0
        && mob.bounds().inflate(Vec3::new(3.0, 2.0, 3.0)).intersects(player.bounds)
}

pub fn player_visible(mob: &Mob, player: &PlayerView, world: &MobWorld<'_>) -> bool {
    let radius = (3.0 * player.visibility_percent(Some(KIND), world.items)).max(2.0);
    player_near(mob, player) && mob.distance_sqr(player.position) <= radius.powi(2)
        && world.line_of_sight_for(Some(mob.position.y),
            Vec3::new(mob.position.x, mob.eye_y(), mob.position.z), Vec3::new(player.eye[0], player.eye[1], player.eye[2]))
}

pub fn state(nav: &Navigation) -> &State {
    nav.__horizon_mixins_naturalist_giant_isopod()
}

pub fn state_mut(nav: &mut Navigation) -> &mut State {
    nav.__horizon_mixins_naturalist_giant_isopod_mut()
}

pub fn hiding(mob: &Mob) -> bool {
    let state = state(&mob.ai.nav);
    mob.kind.id == KIND && !mob.is_baby()
        && (state.nearby || state.hold > 0)
}
