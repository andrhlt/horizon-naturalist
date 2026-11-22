//! Shared aquatic spawn placement for the APP-2130 ocean wildlife slice.
//!
//! Naturalist's Java entities all use an in-water placement owner before their
//! species-specific predicates run.  Keeping that part in one registration
//! makes the native path agree with the generated biome spawn tables while
//! leaving depth, substrate, group and variant predicates to the eventual
//! species ports.

const OCEAN_ENTITIES: [&str; 8] = [
    "anglerfish",
    "blobfish",
    "giant_isopod",
    "great_white_shark",
    "jellyfish",
    "ray",
    "starfish",
    "whale",
];

pub(crate) fn register(registrar: &mut dyn mod_api::Registrar) {
    for id in OCEAN_ENTITIES {
        registrar
            .declare_spawn_placement(
                id,
                mod_api::SpawnPlacementRule {
                    placement: mod_api::SpawnPlacementType::InWater,
                    heightmap: mod_api::Heightmap::OceanFloor,
                    spawnable_on: None,
                    light: mod_api::SpawnLight::Any,
                    biome_tag: None,
                },
            )
            .expect("register Naturalist aquatic spawn placement");
    }
}
