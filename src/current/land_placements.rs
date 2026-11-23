//! Naturalist.java registerSpawnPlacements / NaturalistAnimal.java shared ground rule.
//! Deer is registered with its initial gameplay implementation; this covers the other 18 users.
pub fn register(registrar: &mut dyn mod_api::Registrar) {
    for id in [
        "snail",
        "bear",
        "rhino",
        "lion",
        "elephant",
        "zebra",
        "giraffe",
        "boar",
        "lizard",
        "komodo_dragon",
        "tortoise",
        "mole",
        "rat",
        "black_bear",
        "tiger",
        "ostrich",
        "turkey",
        "capybara",
        "hedgehog",
    ] {
        registrar
            .declare_spawn_placement(
                id,
                mod_api::SpawnPlacementRule {
                    placement: mod_api::SpawnPlacementType::OnGround,
                    heightmap: mod_api::Heightmap::MotionBlockingNoLeaves,
                    spawnable_on: Some("minecraft:substrate_overworld".to_owned()),
                    light: mod_api::SpawnLight::Bright,
                    biome_tag: None,
                },
            )
            .expect("register Naturalist ground spawn placement");
    }
}
