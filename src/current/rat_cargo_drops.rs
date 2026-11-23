use mod_api::{ItemDrop, ItemStack, ServerHost, Tag};

/// Rat cargo is custom entity state, so it is not part of vanilla's mob loot
/// list. Java spills it after loot collection, even when mob drops are off.
pub(crate) fn drops(host: &mut dyn ServerHost, event: &mut mod_api::LivingDropsEvent<'_>) {
    if event.kind != "naturalist:rat" {
        return;
    }
    let Some(entity) = host.entity(event.entity) else {
        return;
    };
    let Some(position) = entity.position(host) else {
        return;
    };
    let Some(data) = entity.nbt(host) else {
        return;
    };
    let Some(Tag::List(carried)) = data.get(&"CarriedItems".into()) else {
        return;
    };
    for value in carried {
        let Tag::Compound(stack_data) = value else {
            continue;
        };
        if let Some(stack) = ItemStack::from_nbt(host, stack_data) {
            event
                .drops
                .push(ItemDrop::new(stack, position, [0.0; 3], 10));
        }
    }
}
