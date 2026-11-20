fn froglass_drop(game: &Game<'_>, context: &NativeLootDrops<'_>, drops: &mut Vec<ItemStack>) {
    use horizon_client::mobs::Who;
    use horizon_client_core::nbt_value::text;

    if context.table != "naturalist:entities/dragonfly" || context.used {
        return;
    }
    let Some((id, _)) = context.entity else {
        return;
    };
    let Some(dragonfly) = game
        .mobs
        .iter()
        .find(|mob| mob.id == id && mob.kind.id == "naturalist:dragonfly" && !mob.alive())
    else {
        return;
    };
    let Some(Who::Mob(attacker)) = dragonfly.last_damage.and_then(|(damage, _)| damage.attacker) else {
        return;
    };
    if !game.mobs.iter().any(|mob| mob.id == attacker && mob.kind.id == "minecraft:frog") {
        return;
    }
    // Horizon's loot entity predicates do not yet read entity NBT. Preserve
    // the original table's Variant test before passing on the normal drops.
    let variant = text(game.persist.raw.get(&id).and_then(|tag| tag.get("Variant")));
    let item = match variant.as_deref().unwrap_or("naturalist:blue") {
        "naturalist:blue" => "naturalist:azure_froglass",
        "naturalist:green" => "naturalist:verdant_froglass",
        "naturalist:red" => "naturalist:crimson_froglass",
        _ => return,
    };
    if !drops.iter().any(|stack| stack.item.as_ref() == item) {
        drops.push(ItemStack::new(item, 1));
    }
}
