//! Starfish block attachment is inherited from the vanilla resin-clump
//! template. This module covers its separate variant-preserving entity drops.

use mod_api::{Identifier, ItemDrop, ItemStack, ServerHost, Tag};

fn death_item(variant: Option<&str>) -> &'static str {
    match variant {
        Some("naturalist:blue") => "naturalist:blue_starfish",
        Some("naturalist:purple") => "naturalist:purple_starfish",
        Some("naturalist:red") => "naturalist:red_starfish",
        _ => "naturalist:orange_starfish",
    }
}

pub(crate) fn drops(host: &mut dyn ServerHost, event: &mut mod_api::LivingDropsEvent<'_>) {
    if event.kind != "naturalist:starfish" {
        return;
    }
    let Some(entity) = host.entity(event.entity) else {
        return;
    };
    let Some(position) = entity.position(host) else {
        return;
    };
    let variant = entity
        .nbt(host)
        .and_then(|data| match data.get(&"Variant".into()) {
            Some(Tag::String(value)) => Some(value.to_string_lossy()),
            _ => None,
        });
    let item_id = death_item(variant.as_deref());
    let Some(identifier) = Identifier::parse(item_id).ok() else {
        return;
    };
    let Some(key) = host.item_key(&identifier) else {
        return;
    };

    // Replace only Naturalist's own variant drop so repeated listeners or a
    // pre-existing table entry cannot duplicate the Starfish item.
    event.drops.retain(|drop| {
        !matches!(
            drop.stack.item().id(host).as_str(),
            "naturalist:orange_starfish"
                | "naturalist:blue_starfish"
                | "naturalist:purple_starfish"
                | "naturalist:red_starfish"
        )
    });
    event
        .drops
        .push(ItemDrop::new(ItemStack::of(key), position, [0.0; 3], 10));
}

#[cfg(test)]
mod tests {
    use super::death_item;

    #[test]
    fn death_item_tracks_variant_and_uses_java_fallback() {
        assert_eq!(
            death_item(Some("naturalist:blue")),
            "naturalist:blue_starfish"
        );
        assert_eq!(
            death_item(Some("naturalist:purple")),
            "naturalist:purple_starfish"
        );
        assert_eq!(
            death_item(Some("naturalist:red")),
            "naturalist:red_starfish"
        );
        assert_eq!(
            death_item(Some("other:unknown")),
            "naturalist:orange_starfish"
        );
        assert_eq!(death_item(None), "naturalist:orange_starfish");
    }

    #[test]
    fn colored_starfish_blocks_keep_multiface_template_behavior() {
        let blocks = [
            include_str!("../../data/naturalist/horizon/block/orange_starfish.json"),
            include_str!("../../data/naturalist/horizon/block/blue_starfish.json"),
            include_str!("../../data/naturalist/horizon/block/purple_starfish.json"),
            include_str!("../../data/naturalist/horizon/block/red_starfish.json"),
        ];
        for block in blocks {
            assert!(block.contains("\"template\": \"minecraft:resin_clump\""));
        }
    }
}
