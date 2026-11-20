fn bass_bucket_restore(game: &mut Game<'_>, id: u32, stack: &ItemStack) -> std::ops::ControlFlow<Result<()>> {
    use horizon_client::mobs::nav::naturalist_bass_state as bass;
    use std::ops::ControlFlow;

    if stack.item.as_ref() == "naturalist:giant_isopod_bucket" {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
            mob.extra.from_bucket = true;
        }
        let saved = stack.data().and_then(|data| data.other("minecraft:bucket_entity_data"))
            .and_then(|tag| tag.get("Variant"));
        let variant = if let Some(legacy) = horizon_client::save::number(saved) {
            ["naturalist:brown", "naturalist:blue"][legacy.rem_euclid(2) as usize].to_owned()
        } else {
            horizon_client_core::nbt_value::text(saved).unwrap_or_else(|| "naturalist:brown".to_owned())
        };
        super::naturalist_deer_variants::put_variant(game, id, &variant);
        return ControlFlow::Continue(());
    }
    if stack.item.as_ref() == "naturalist:starfish_bucket" {
        let saved = stack.data().and_then(|data| data.other("minecraft:bucket_entity_data"))
            .and_then(|tag| tag.get("Variant"));
        let variant = if let Some(legacy) = horizon_client::save::number(saved) {
            ["naturalist:orange", "naturalist:purple", "naturalist:blue", "naturalist:red"][legacy.rem_euclid(4) as usize].to_owned()
        } else {
            horizon_client_core::nbt_value::text(saved).unwrap_or_else(|| "naturalist:orange".to_owned())
        };
        super::naturalist_deer_variants::put_variant(game, id, &variant);
        return ControlFlow::Continue(());
    }
    if stack.item.as_ref() == "naturalist:anglerfish_bucket" {
        let saved = stack.data().and_then(|data| data.other("minecraft:bucket_entity_data"))
            .and_then(|tag| tag.get("Variant"));
        let variant = if let Some(legacy) = horizon_client::save::number(saved) {
            ["naturalist:red", "naturalist:glow"][legacy.rem_euclid(2) as usize].to_owned()
        } else {
            horizon_client_core::nbt_value::text(saved).unwrap_or_else(|| "naturalist:red".to_owned())
        };
        super::naturalist_deer_variants::put_variant(game, id, &variant);
        return ControlFlow::Continue(());
    }
    if stack.item.as_ref() == "naturalist:ray_bucket" {
        let saved = stack.data().and_then(|data| data.other("minecraft:bucket_entity_data"))
            .and_then(|tag| tag.get("Variant"));
        let variant = if let Some(legacy) = horizon_client::save::number(saved) {
            ["naturalist:eagle_ray", "naturalist:mobula_ray", "naturalist:stingray"][legacy.rem_euclid(3) as usize].to_owned()
        } else {
            horizon_client_core::nbt_value::text(saved).unwrap_or_else(|| "naturalist:eagle_ray".to_owned())
        };
        super::naturalist_deer_variants::put_variant(game, id, &variant);
        return ControlFlow::Continue(());
    }
    if stack.item.as_ref() == "naturalist:jellyfish_bucket" {
        let saved = stack.data().and_then(|data| data.other("minecraft:bucket_entity_data"))
            .and_then(|tag| tag.get("Variant"));
        let variant = if let Some(legacy) = horizon_client::save::number(saved) {
            ["naturalist:white", "naturalist:orange", "naturalist:pink", "naturalist:blue", "naturalist:green"][legacy.rem_euclid(5) as usize].to_owned()
        } else {
            horizon_client_core::nbt_value::text(saved).unwrap_or_else(|| "naturalist:white".to_owned())
        };
        super::naturalist_deer_variants::put_variant(game, id, &variant);
        return ControlFlow::Continue(());
    }
    if stack.item.as_ref() == "naturalist:blobfish_bucket" {
        let saved = stack.data().and_then(|data| data.other("minecraft:bucket_entity_data"))
            .and_then(|tag| tag.get("Variant"));
        let variant = horizon_client_core::nbt_value::text(saved).unwrap_or_else(|| "naturalist:blobfish".to_owned());
        super::naturalist_deer_variants::put_variant(game, id, &variant);
        return ControlFlow::Continue(());
    }
    let default_variant = match stack.item.as_ref() {
        "naturalist:catfish_bucket" => Some("naturalist:catfish"),
        "naturalist:piranha_bucket" => Some("naturalist:piranha"),
        _ => None,
    };
    if let Some(default_variant) = default_variant {
        super::naturalist_deer_variants::put_variant(game, id, default_variant);
        return ControlFlow::Continue(());
    }
    if stack.item.as_ref() != "naturalist:bass_bucket" {
        return ControlFlow::Continue(());
    }
    let saved = stack.data().and_then(|data| data.other("minecraft:bucket_entity_data"))
        .and_then(|tag| tag.get("Variant"));
    let variant = if let Some(legacy) = horizon_client::save::number(saved) {
        ["naturalist:bass", "naturalist:bass_medium", "naturalist:bass_large"][legacy.rem_euclid(3) as usize].to_owned()
    } else {
        horizon_client_core::nbt_value::text(saved).unwrap_or_else(|| "naturalist:bass".to_owned())
    };
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id && mob.kind.id == "naturalist:bass") {
        mob.persistent = false;
        horizon_client::entity_nbt::apply_item_components(mob, stack);
        mob.extra.from_bucket = true;
        mob.extra.variant = bass::tier(&variant);
        super::naturalist_deer_variants::put_variant(game, id, &variant);
    }
    ControlFlow::Break(game.mob_ambient(id))
}
