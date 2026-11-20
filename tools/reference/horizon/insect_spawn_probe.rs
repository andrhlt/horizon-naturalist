use super::*;
use crate::game::naturalist_deer_placement::{Pass, PlacementPass};
use horizon_client::mobs::{spawn_rules, spawn_rules::Reason};
use horizon_core::random::LegacyRandom;

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_insect_spawn(&self) -> serde_json::Value {
        let Some(_placement) = PlacementPass::enter(Pass::Natural) else {
            return serde_json::json!({"error": "placement already in progress"});
        };
        let level = self.natural_level();
        let mut report = serde_json::Map::new();
        for (site, [x, y, z]) in [
            ("open_mud", [2, 100, 2]),
            ("open_stone", [11, 100, 2]),
            ("dark_grass", [2, 100, 34]),
            ("dark_stone", [11, 100, 34]),
        ] {
            let mut result = serde_json::Map::new();
            let light = SpawnLevel::sky_light(&level, x, y, z)
                .max(SpawnLevel::block_light(&level, x, y, z));
            result.insert("raw_light".to_owned(), light.into());
            for (name, id) in [
                ("butterfly", "naturalist:butterfly"),
                ("dragonfly", "naturalist:dragonfly"),
                ("firefly", "naturalist:firefly"),
            ] {
                let Some((species, _, _, _)) = level.mob_type(id) else {
                    return serde_json::json!({"error": format!("missing {id} spawn type")});
                };
                let mut successes = 0;
                for seed in 0..64 {
                    let mut random = LegacyRandom::new(seed);
                    if !spawn_rules::check_spawn_rules(
                        species,
                        &level,
                        Reason::Natural,
                        &mut random,
                        x,
                        y,
                        z,
                    ) {
                        continue;
                    }
                    let request = SpawnRequest {
                        entity: id,
                        species,
                        position: [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5],
                        yaw: 0.0,
                        group_index: 0,
                        reason: Reason::Natural,
                    };
                    if self.finalize_spawned(&request, &mut Vec::new()).is_some() {
                        successes += 1;
                    }
                }
                result.insert(name.to_owned(), successes.into());
            }
            report.insert(site.to_owned(), result.into());
        }
        report.into()
    }
}
