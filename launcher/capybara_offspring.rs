fn capybara_offspring(game: &mut Game<'_>, from: usize) {
    for mob in game.mobs.iter_mut().skip(from).filter(|mob| matches!(mob.kind.id.as_str(), "naturalist:capybara" | "naturalist:rat")) {
        let Some(tag) = game.persist.raw.get(&mob.id) else { continue };
        let Some(horizon_world::nbt::Tag::IntArray(owner)) = tag.get("Owner") else { continue };
        let Ok(owner) = <[i32; 4]>::try_from(owner.as_slice()) else { continue };
        mob.extra.tame = true;
        mob.extra.owner = Some(owner);
        if mob.kind.id == "naturalist:capybara" {
            mob.extra.color = horizon_client_core::nbt_value::number(tag.get("DyeColor"))
                .and_then(|value| u8::try_from(value).ok()).unwrap_or(14);
        }
    }
}
