use super::Navigation;
use crate::mobs::{Mob, MobWorld, PlayerView};
use horizon_core::geometry::Vec3;

pub const KIND: &str = "naturalist:crab";
pub const FOOD: &str = "minecraft:tropical_fish";
pub const WEAPON_TAGS: [&str; 5] = [
    "minecraft:swords", "minecraft:axes", "minecraft:pickaxes", "minecraft:shovels", "minecraft:hoes",
];

#[derive(Clone, Copy, Debug)]
pub struct Weapon {
    pub id: u32,
    pub at: [f64; 3],
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub from_hand: bool,
    pub no_pickup: bool,
    pub player_near: bool,
    pub dancing: bool,
    pub swing_until: u64,
    pub nearest: Option<Weapon>,
    pub target: Option<u32>,
    pub weapon: Option<Weapon>,
}

pub fn state(nav: &Navigation) -> &State {
    nav.__horizon_mixins_naturalist_crab()
}

pub fn state_mut(nav: &mut Navigation) -> &mut State {
    nav.__horizon_mixins_naturalist_crab_mut()
}

pub fn holding_weapon(mob: &Mob) -> bool {
    mob.extra.main_hand.is_some()
}

pub fn hides(mob: &Mob) -> bool {
    if mob.kind.id != KIND || mob.is_baby() || mob.extra.tame || holding_weapon(mob) {
        return false;
    }
    let status = state(&mob.ai.nav);
    status.player_near && status.nearest.is_none()
}

pub fn immobilized(mob: &Mob) -> bool {
    mob.kind.id == KIND && (hides(mob) || state(&mob.ai.nav).dancing)
}

pub fn visible(mob: &Mob, player: &PlayerView, world: &MobWorld<'_>) -> bool {
    if !player.alive || player.creative || player.spectator
        || (player.bounds.max_position().y - player.bounds.min_position().y - 1.5).abs() < 0.005
        || [player.main_hand.as_deref(), player.off_hand.as_deref()].contains(&Some(FOOD))
    {
        return false;
    }
    let size = Vec3::new(4.0, 2.0, 4.0);
    let range = (4.0 * player.visibility_percent(Some(KIND), world.items)).max(2.0);
    mob.distance_sqr(player.position) <= range * range
        && mob.bounds().inflate(size).contains(player.position)
        && world.line_of_sight_for(Some(mob.position.y),
            Vec3::new(mob.position.x, mob.eye_y(), mob.position.z),
            Vec3::new(player.eye[0], player.eye[1], player.eye[2]))
}
