//! Naturalist's Java fish use `MobSpawnType.IN_WATER`.
//!
//! The Java release also adds species-specific depth/light predicates for some
//! fish. Those predicates are not expressible by `SpawnPlacementRule`; they
//! remain documented in the fixture audit rather than being approximated here.

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    for id in [
        "anglerfish",
        "bass",
        "blobfish",
        "catfish",
        "jellyfish",
        "piranha",
        "ray",
    ] {
        registrar
            .declare_spawn_placement(
                id,
                mod_api::SpawnPlacementRule {
                    placement: mod_api::SpawnPlacementType::InWater,
                    heightmap: mod_api::Heightmap::MotionBlockingNoLeaves,
                    spawnable_on: None,
                    light: mod_api::SpawnLight::Any,
                    biome_tag: None,
                },
            )
            .expect("register Naturalist aquatic spawn placement");
    }
}
