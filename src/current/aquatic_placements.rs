//! Naturalist's Java fish use `MobSpawnType.IN_WATER`, with additional rules
//! for three species applied by the post-placement hook.

pub fn anglerfish_spawn_ok(
    surface_rule: bool,
    y: i32,
    sea_level: Option<i32>,
    water: bool,
    block_light: Option<i32>,
    sky_light: Option<i32>,
) -> bool {
    surface_rule
        || sea_level.is_some_and(|sea| y <= sea - 33)
            && water
            && block_light == Some(0)
            && sky_light == Some(0)
}

pub fn anglerfish_surface_spawn_ok(
    in_water: bool,
    y: i32,
    sea_level: Option<i32>,
    water_below: bool,
    water_block_above: bool,
) -> bool {
    in_water
        && sea_level.is_some_and(|sea| y >= sea - 13 && y <= sea)
        && water_below
        && water_block_above
}

pub fn deep_water_spawn_ok(
    default_result: bool,
    y: i32,
    sea_level: Option<i32>,
    depth: i32,
) -> bool {
    default_result && sea_level.is_some_and(|sea| y <= sea - depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn java_aquatic_spawn_boundaries() {
        // Captured from Naturalist 2.0.7's Java predicates. The engine's
        // default surface result is true only for the surface-water case.
        assert!(anglerfish_spawn_ok(
            false,
            27,
            Some(63),
            true,
            Some(0),
            Some(0)
        ));
        assert!(!anglerfish_spawn_ok(
            false,
            27,
            Some(63),
            true,
            Some(15),
            Some(0)
        ));
        assert!(!anglerfish_spawn_ok(
            false,
            40,
            Some(63),
            true,
            Some(0),
            Some(0)
        ));
        assert!(anglerfish_spawn_ok(
            true,
            54,
            Some(63),
            true,
            Some(0),
            Some(0)
        ));
        assert!(anglerfish_surface_spawn_ok(true, 54, Some(63), true, true));
        assert!(!anglerfish_surface_spawn_ok(true, 49, Some(63), true, true));
        assert!(!anglerfish_surface_spawn_ok(
            true,
            54,
            Some(63),
            false,
            true
        ));
        assert!(deep_water_spawn_ok(true, 30, Some(63), 33));
        assert!(!deep_water_spawn_ok(true, 31, Some(63), 33));
        assert!(deep_water_spawn_ok(true, 33, Some(63), 0));
        assert!(deep_water_spawn_ok(true, 63, Some(63), 0));
        assert!(!deep_water_spawn_ok(true, 64, Some(63), 0));
        assert!(!deep_water_spawn_ok(false, 30, Some(63), 33));
    }
}

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
