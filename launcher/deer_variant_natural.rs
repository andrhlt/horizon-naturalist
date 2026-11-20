fn deer_variant_natural(game: &mut Game<'_>, from: usize, natural: bool) {
    for index in from..game.mobs.len() {
        let mob = &mut game.mobs[index];
        if let Some(variant) = mob.ai.nav.__horizon_mixins_naturalist_deer_spawn_variant_mut().take() {
            let id = mob.id;
            super::naturalist_deer_variants::put_variant(game, id, &variant);
            continue;
        }
        if !natural {
            continue;
        }
        let palette = mob
            .ai
            .nav
            .__horizon_mixins_naturalist_butterfly_spawn_pool_mut()
            .take();
        if let Some((pair, choice)) = palette {
            let id = mob.id;
            super::naturalist_deer_variants::put_variant(game, id, &pair[choice]);
        } else {
            super::naturalist_deer_variants::assign_variant_at(game, index);
        }
        horizon_client::mobs::nav::naturalist_elephants::add_spawn_bonus(
            &mut game.mobs[index], &mut game.world.random,
        );
    }
}
