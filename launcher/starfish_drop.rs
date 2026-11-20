fn starfish_drop(game: &Game<'_>, context: &NativeLootDrops<'_>, drops: &mut Vec<ItemStack>) {
    if context.table != "naturalist:entities/starfish" {
        return;
    }
    let Some((id, _)) = context.entity else { return };
    let identifier = game.persist.raw.get(&id).and_then(|entity| entity.get("Variant"));
    let variant = horizon_client_core::nbt_value::text(identifier)
        .and_then(|id| horizon_world::identifier::Identifier::parse(&id).ok());
    let color = match variant.as_ref().map(|variant| variant.path()) {
        Some("purple") => "naturalist:purple_starfish",
        Some("blue") => "naturalist:blue_starfish",
        Some("red") => "naturalist:red_starfish",
        _ => "naturalist:orange_starfish",
    };
    drops.push(ItemStack::new(color, 1));
}
