pub(crate) fn route(biomes: &mut std::sync::Arc<horizon_client::biome::Biomes>) {
    for biome in &mut std::sync::Arc::make_mut(biomes).list {
        let mut moved = Vec::new();
        for (category, entries) in &mut biome.spawns {
            entries.retain(|entry| {
                let desired = match entry.entity.as_str() {
                    "naturalist:anglerfish" => "water_ambient",
                    "naturalist:bass" => "water_ambient",
                    "naturalist:blobfish" => "water_ambient",
                    "naturalist:butterfly" => "ambient",
                    "naturalist:catfish" => "water_ambient",
                    "naturalist:clam" => "water_ambient",
                    "naturalist:dragonfly" => "ambient",
                    "naturalist:firefly" => "ambient",
                    "naturalist:giant_isopod" => "water_ambient",
                    "naturalist:great_white_shark" => "water_creature",
                    "naturalist:jellyfish" => "water_ambient",
                    "naturalist:piranha" => "water_ambient",
                    "naturalist:ray" => "water_ambient",
                    "naturalist:starfish" => "water_ambient",
                    "naturalist:whale" => "water_creature",
                    _ => return true,
                };
                if category == desired {
                    return true;
                }
                moved.push((desired, entry.clone()));
                false
            });
        }
        for (category, entry) in moved {
            biome
                .spawns
                .entry(category.to_owned())
                .or_default()
                .push(entry);
        }
    }
}
