use super::{Aabb, Game, Vec3, splash_distance_sqr};
use crate::game::naturalist_antivenom;

pub(in crate::game) fn clear_player_at(game: &mut Game<'_>, at: Vec3, age: u32) {
    let Some(player) = game.player.as_ref() else {
        return;
    };
    let w = f64::from(player.dimensions.width) / 2.0;
    let position = player.position;
    let bounds = Aabb::new(
        Vec3::new(position.x - w, position.y, position.z - w),
        Vec3::new(position.x + w, position.y + f64::from(player.dimensions.height), position.z + w),
    );
    if splash_distance_sqr(at, age, bounds).is_some() {
        naturalist_antivenom::clear_player(game);
    }
}
