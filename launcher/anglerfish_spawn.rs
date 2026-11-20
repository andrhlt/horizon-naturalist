use super::{Game, SpawnLevel};

pub(crate) fn deep_glow(game: &Game<'_>, [x, y, z]: [i32; 3]) -> bool {
    let level = game.natural_level();
    y <= level.sea_level() - 33
        && level.block_light(x, y, z).max(level.sky_light(x, y, z)) == 0
        && game.res.facts().get(game.world.block(x, y, z)).name.as_ref() == "minecraft:water"
}

pub(crate) fn placement(game: &Game<'_>, [x, y, z]: [i32; 3]) -> bool {
    let sea = horizon_world::level::sea_level_of(game.level_key());
    let facts = game.res.facts();
    let near_surface = (sea - 13..=sea).contains(&y)
        && facts.get(game.world.block(x, y - 1, z)).fluid == horizon_client::blocks::Fluid::Water
        && facts.get(game.world.block(x, y + 1, z)).name.as_ref() == "minecraft:water";
    near_surface || deep_glow(game, [x, y, z])
}
