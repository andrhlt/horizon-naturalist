use super::Game;

// Wolf's NBT reader changes unowned max health to vanilla's 8. Naturalist's
// Wolf-backed species have their own defaults and must keep saved overrides.
pub(super) fn restore(
    game: &mut Game<'_>,
    entity: u32,
    kind: &str,
    raw: Option<&horizon_core::nbt::Tag>,
) {
    use horizon_client::mobs::finalize::{Attr, default_value, set_base};
    use horizon_core::nbt::Tag;
    if !matches!(
        kind,
        "naturalist:ant" | "naturalist:crab" | "naturalist:hedgehog"
    ) {
        return;
    }
    let saved = raw
        .or_else(|| game.persist.raw.get(&entity))
        .and_then(|raw| {
            let Tag::List(attributes) = raw.get("attributes")? else {
                return None;
            };
            attributes.iter().find_map(|attribute| {
                let Tag::String(id) = attribute.get("id")? else {
                    return None;
                };
                (id.to_string_lossy() == "minecraft:max_health")
                    .then(|| {
                        attribute
                            .get("base")
                            .and_then(horizon_world::datafix::dynamic::as_number)
                            .map(|n| n.double_value())
                    })
                    .flatten()
            })
        });
    if let Some(mob) = game
        .mobs
        .iter_mut()
        .find(|mob| mob.id == entity && mob.extra.owner.is_none())
    {
        if let Some(base) = saved.or_else(|| default_value(mob, Attr::MaxHealth)) {
            set_base(mob, Attr::MaxHealth, base);
        }
    }
}
pub(super) type Nbt = horizon_core::nbt::Tag;
