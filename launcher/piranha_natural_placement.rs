fn piranha_natural_placement(level: &GameLevel<'_, '_>, entity: &str) -> std::ops::ControlFlow<Option<(Species, f32, f32, Category)>> {
    use std::ops::ControlFlow;
    if entity == "naturalist:crab" {
        return ControlFlow::Break(level.game.res.mob_types().get(entity).map(|kind| {
            (Species::Cow, kind.width as f32, kind.height as f32, Category::from_name(&kind.category))
        }));
    }
    if !matches!(entity, "naturalist:bass" | "naturalist:piranha" | "naturalist:blobfish" | "naturalist:anglerfish" | "naturalist:starfish" | "naturalist:giant_isopod" | "naturalist:great_white_shark" | "naturalist:whale" | "naturalist:clam") {
        return ControlFlow::Continue(());
    }
    // Only TropicalFish reads Spawned.school in the stock natural spawner.
    // Bass uses its own weight, surface check, and actual Cod-based mob.
    let placement = if entity == "naturalist:bass" { Species::TropicalFish } else { Species::Tadpole };
    ControlFlow::Break(level.game.res.mob_types().get(entity).map(|kind| {
        (placement, kind.width as f32, kind.height as f32, Category::from_name(&kind.category))
    }))
}
