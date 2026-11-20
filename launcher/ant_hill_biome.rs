fn ant_hill_biome(world: &mut World<'_>, position: [i32; 3], feature: FeatureIdentity) {
    use super::{
        block_feature::naturalist_ant_hill_feature as hill, feature_graph::FeatureDefinition,
        placement::PlacementModifier,
    };

    let decoration = world.decoration;
    let Some(name) = decoration.names.get(feature.0 as usize) else {
        return;
    };
    let Some(big) = hill::size(name) else { return };
    let Some(Ok(placed)) = decoration.features.get(feature.0 as usize) else {
        return;
    };
    if !matches!(placed.feature.as_ref(), FeatureDefinition::NoOp)
        || !matches!(
            placed.placement.as_slice(),
            [
                PlacementModifier::Rarity(_),
                PlacementModifier::InSquare,
                PlacementModifier::Heightmap(_),
                PlacementModifier::Biome
            ]
        )
    {
        return;
    }
    let Ok(biome) = world.biome_identity(position) else {
        return;
    };
    let mask = &decoration.biome_features[biome.0 as usize];
    if !mask
        .get(feature.0 as usize / 64)
        .is_some_and(|bits| (bits >> (feature.0 % 64)) & 1 != 0)
    {
        return;
    }
    let Some((index, step)) = decoration.plan.seed(feature) else {
        return;
    };
    let (Ok(seed), Ok(bottom)) = (world.world_seed(), world.min_section_y()) else {
        return;
    };
    let origin = [
        world.center.x.wrapping_shl(4),
        bottom.wrapping_shl(4),
        world.center.z.wrapping_shl(4),
    ];
    let Ok(hill) = hill::build(big, &decoration.bindings) else {
        return;
    };

    // That Java placement chain emits at most one position. Replaying its three
    // pre-biome modifiers recovers the exact RNG state at the accepted position.
    let mut random = WorldgenRandom::new(XoroshiroRandom::new(0));
    let seed = random.set_decoration_seed(seed, origin[0], origin[2]);
    random.set_feature_seed(seed, index as i32, step as i32);
    let _ = FeaturePlacer::new().place_with_context(
        &placed.placement[..3],
        world,
        &mut random,
        origin,
        |world, random, candidate| {
            if candidate == position {
                hill.place(world, random, candidate, |world, at| {
                    if world.is_full_chunk(at) {
                        return Ok(());
                    }
                    use crate::nbt::Tag;
                    let nbt = [
                        ("id".into(), Tag::String("naturalist:ant_hill".into())),
                        ("Storage".into(), Tag::List(Vec::new())),
                    ].into_iter().collect();
                    world.set_block_entity(at, nbt)
                        .map_err(super::block_feature::BlockFeatureError::Query)
                })
            } else {
                Ok(false)
            }
        },
    );
}
