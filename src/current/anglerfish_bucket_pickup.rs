//! Atomic player capture for Naturalist's aquatic fish buckets.

use mod_api::{Compound, Identifier, ItemStack, Tag};

fn bucket_item(kind: &str) -> Option<(&'static str, &'static str)> {
    Some(match kind {
        "naturalist:anglerfish" => ("naturalist:anglerfish_bucket", "naturalist:red"),
        "naturalist:blobfish" => ("naturalist:blobfish_bucket", "naturalist:blobfish"),
        "naturalist:giant_isopod" => ("naturalist:giant_isopod_bucket", "naturalist:brown"),
        "naturalist:jellyfish" => ("naturalist:jellyfish_bucket", "naturalist:white"),
        "naturalist:ray" => ("naturalist:ray_bucket", "naturalist:eagle_ray"),
        "naturalist:starfish" => ("naturalist:starfish_bucket", "naturalist:orange"),
        _ => return None,
    })
}

pub(crate) fn interact(
    host: mod_api::HostSide<'_>,
    event: &mut mod_api::EntityInteractEvent<'_>,
) -> Option<mod_api::InteractionResult> {
    let (bucket, default_variant) = bucket_item(event.kind)?;
    let water_bucket = event.stack?;
    let item_id = match &host {
        mod_api::HostSide::Server(host) => water_bucket.item().id(&**host),
        mod_api::HostSide::Client(host) => water_bucket.item().id(&**host),
    };
    if item_id != "minecraft:water_bucket" {
        return None;
    }

    match host {
        mod_api::HostSide::Client(_) => Some(mod_api::InteractionResult::Success),
        mod_api::HostSide::Server(host) => {
            let player = host.players().acting_player()?;
            let slot = match event.hand {
                mod_api::Hand::Main => player.selected_slot(host)? as u32,
                mod_api::Hand::Off => 40,
            };
            let held = player
                .inventory_slots(host)?
                .into_iter()
                .find(|(candidate, _)| *candidate == slot)
                .map(|(_, stack)| stack)?;
            if held.item().id(host) != "minecraft:water_bucket" || held.count() != 1 {
                return None;
            }

            let entity = host.entity(u32::try_from(event.entity).ok()?)?;
            if entity.kind(host)?.id(host) != event.kind {
                return None;
            }
            let variant = entity
                .nbt(host)
                .and_then(|data| data.get(&"Variant".into()).cloned())
                .unwrap_or_else(|| Tag::String(default_variant.into()));
            let bucket_key = host.item_key(&Identifier::parse(bucket).ok()?)?;
            let mut filled = ItemStack::new(bucket_key, 1)?;
            let entity_data: Compound = [("Variant".into(), variant.clone())].into();
            let custom_data: Compound = [("Variant".into(), variant)].into();
            let components: Compound = [
                (
                    "minecraft:bucket_entity_data".into(),
                    Tag::Compound(entity_data),
                ),
                ("minecraft:custom_data".into(), Tag::Compound(custom_data)),
            ]
            .into();
            filled.apply_components(&components).ok()?;

            // Write the filled bucket before removing the fish. If either write
            // is refused, leave the original bucket and fish in place.
            player.set_inventory_slot(host, slot, Some(filled)).ok()?;
            if !entity.discard(host) {
                let _ = player.set_inventory_slot(host, slot, Some(held));
                return None;
            }
            Some(mod_api::InteractionResult::SuccessServer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bucket_item;

    #[test]
    fn aquatic_species_have_their_bucket_and_default_variant() {
        for (kind, bucket) in [
            ("naturalist:anglerfish", "naturalist:anglerfish_bucket"),
            ("naturalist:blobfish", "naturalist:blobfish_bucket"),
            ("naturalist:giant_isopod", "naturalist:giant_isopod_bucket"),
            ("naturalist:jellyfish", "naturalist:jellyfish_bucket"),
            ("naturalist:ray", "naturalist:ray_bucket"),
            ("naturalist:starfish", "naturalist:starfish_bucket"),
        ] {
            assert_eq!(bucket_item(kind).unwrap().0, bucket);
        }
        assert!(bucket_item("minecraft:cod").is_none());
    }
}
