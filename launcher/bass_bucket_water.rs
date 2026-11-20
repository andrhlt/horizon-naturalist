fn bass_bucket_water(game: &mut Game<'_>, stack: &ItemStack) -> std::ops::ControlFlow<Result<horizon_client::first_person::UseResult>> {
    use horizon_client::first_person::UseResult;
    use std::ops::ControlFlow;

    let kind = match stack.item.as_ref() {
        "naturalist:bass_bucket" => "naturalist:bass",
        "naturalist:catfish_bucket" => "naturalist:catfish",
        "naturalist:piranha_bucket" => "naturalist:piranha",
        "naturalist:blobfish_bucket" => "naturalist:blobfish",
        "naturalist:jellyfish_bucket" => "naturalist:jellyfish",
        "naturalist:starfish_bucket" => "naturalist:starfish",
        "naturalist:giant_isopod_bucket" => "naturalist:giant_isopod",
        "naturalist:anglerfish_bucket" => "naturalist:anglerfish",
        "naturalist:ray_bucket" => "naturalist:ray",
        _ => return ControlFlow::Continue(()),
    };
    let is_water = |pos: [i32; 3]| {
        let block = game.world.block(pos[0], pos[1], pos[2]);
        game.res.facts().get(block).fluid == horizon_client::blocks::Fluid::Water
    };
    // The stock client sends the bucket's UseItem after UseItemOn. Process it
    // only once so the second packet cannot refill the returned empty bucket.
    if matches!(game.guest_use, Some(GuestUse::OnBlock(_))) {
        return ControlFlow::Continue(());
    }
    let clicked = game.pick().map(|hit| {
        let [x, y, z] = hit.pos;
        let block = game.world.block(x, y, z);
        if game.res.behaviours().collision_shape(block).is_none_or(|shape| shape.is_empty()) {
            hit.pos
        } else {
            hit.beside()
        }
    });
    let pos = clicked.filter(|pos| is_water(*pos)).or_else(|| {
        (!matches!(kind, "naturalist:starfish" | "naturalist:giant_isopod")).then(|| game.bucket_hit()).flatten().map(|hit| hit.pos)
            .filter(|pos| is_water(*pos))
    });
    let Some(pos) = pos else {
        return ControlFlow::Continue(());
    };
    if game.bucket_empty_cancelled(stack, pos) {
        return ControlFlow::Break(Ok(UseResult::Fail));
    }
    let place = (pos[0], pos[1], pos[2]);
    if let Some(id) = game.spawn_mob_at(kind, place, true)
        && let Err(error) = game.mob_placed(id, stack)
    {
        return ControlFlow::Break(Err(error));
    }
    game.post_block_game_event("minecraft:entity_place", pos, game.sculk_actor(), None);
    let at = pos.map(|coordinate| f64::from(coordinate) + 0.5);
    game.audio.play("minecraft:item.bucket.empty_fish", Some(at), 1.0, 1.0);
    game.award_stat("used", &stack.item, 1);
    if !game.creative {
        *game.held_mut() = Some(ItemStack::new("minecraft:bucket", 1));
    }
    ControlFlow::Break(Ok(UseResult::Success { swing: true }))
}
