//! Player and dispenser release for the non-Anglerfish ocean fish buckets.

pub(crate) struct OceanBucket(pub(crate) &'static str);

#[mod_api::horizon_part]
impl mod_api::ItemPart for OceanBucket {
    fn empty_contents(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EmptyContentsEvent<'_>,
    ) -> Option<bool> {
        let Some(player) = event.player else {
            // Horizon's generic dispenser bucket path owns dispenser release.
            return None;
        };
        let Some(level) = player.entity().level(host) else {
            return Some(false);
        };
        let pos = event.pos;
        let target = [pos.x, pos.y, pos.z];
        let Some(water) = host.block_state_id("minecraft:water") else {
            return Some(false);
        };
        let Some(current) = level.block(host, target) else {
            return Some(false);
        };
        let Some(current_id) = mod_api::__private::block_key_id(current) else {
            return Some(false);
        };
        if current_id != "minecraft:water" && current_id != "minecraft:air" {
            return Some(false);
        }
        if current_id == "minecraft:air"
            && !matches!(
                level.set_block(host, target, water, mod_api::BlockUpdateFlags::NEIGHBORS),
                Ok(true)
            )
        {
            return Some(false);
        }

        let position = [pos.x as f64 + 0.5, pos.y as f64, pos.z as f64 + 0.5];
        let Some(fish) = level.spawn_mob(host, self.0, position, mod_api::SpawnReason::Bucket)
        else {
            return Some(false);
        };
        let variant = event
            .stack
            .components()
            .map(|components| components.as_nbt())
            .and_then(|components| components.get(&"minecraft:bucket_entity_data".into()))
            .and_then(|tag| match tag {
                mod_api::Tag::Compound(data) => data.get(&"Variant".into()),
                _ => None,
            })
            .cloned();
        if let Some(variant) = variant {
            let nbt: mod_api::Compound = [("Variant".into(), variant)].into();
            if !fish.merge_nbt(host, nbt) {
                return Some(false);
            }
        }
        Some(true)
    }
}

pub(crate) fn register(registrar: &mut dyn mod_api::Registrar) {
    for (item, species) in [
        ("naturalist:blobfish_bucket", "naturalist:blobfish"),
        ("naturalist:giant_isopod_bucket", "naturalist:giant_isopod"),
        ("naturalist:jellyfish_bucket", "naturalist:jellyfish"),
        ("naturalist:ray_bucket", "naturalist:ray"),
        ("naturalist:starfish_bucket", "naturalist:starfish"),
    ] {
        let key = registrar.item_key(item);
        registrar.item_part(key, OceanBucket(species));
    }
}
