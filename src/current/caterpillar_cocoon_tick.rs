fn caterpillar_cocoon_tick(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:caterpillar" {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            crate::game::naturalist_caterpillar_cocoon::install(mob);
        }
    } else if kind == "naturalist:butterfly" {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            crate::game::naturalist_butterfly_pollinate::install(mob);
        }
    }
}
