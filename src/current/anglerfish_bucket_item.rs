//! Marks the Anglerfish bucket for server handled release and dispenser use.
pub struct AnglerfishBucket;

#[mod_api::horizon_part]
impl mod_api::ItemPart for AnglerfishBucket {
    fn empty_contents(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EmptyContentsEvent<'_>,
    ) -> Option<bool> {
        // Let the existing native dispenser handler own that path. Player use
        // goes through Horizon's transactional EmptyContents result, which
        // replaces the held stack with an empty bucket only after Some(true).
        let Some(player) = event.player else {
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
        let Some(fish) = level.spawn_mob(
            host,
            "naturalist:anglerfish",
            position,
            mod_api::SpawnReason::Bucket,
        ) else {
            return Some(false);
        };
        let patch = event.stack.components().map(|components| components.as_nbt());
        let component = |name: &str| {
            patch
                .and_then(|patch| patch.get(&name.into()))
                .and_then(|tag| match tag {
                    mod_api::Tag::Compound(fields) => fields.get(&"Variant".into()),
                    _ => None,
                })
                .cloned()
        };
        let variant = component("minecraft:bucket_entity_data")
            .or_else(|| component("minecraft:custom_data"));
        if let Some(variant) = variant {
            let nbt: mod_api::Compound = [("Variant".into(), variant)].into();
            if !fish.merge_nbt(host, nbt) {
                return Some(false);
            }
        }
        Some(true)
    }

    fn use_item(
        &self,
        host: mod_api::HostSide<'_>,
        _: &mut mod_api::UseItemEvent<'_>,
    ) -> Option<mod_api::InteractionResult> {
        match host {
            mod_api::HostSide::Client(_) => Some(mod_api::InteractionResult::Success),
            mod_api::HostSide::Server(_) => None,
        }
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let item = registrar.item_key("naturalist:anglerfish_bucket");
    registrar.item_part(item, AnglerfishBucket);
}
