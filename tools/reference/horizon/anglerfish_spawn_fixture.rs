use super::*;
use horizon_client::mobs::spawn_rules::Reason;

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_anglerfish_deep(&mut self) -> String {
        let mut pending = Vec::new();
        let request = SpawnRequest { entity: "naturalist:anglerfish", species: Species::Tadpole,
            position: [8.5, 27.0, 8.5], yaw: 0.0, group_index: 0, reason: Reason::Natural };
        assert!(self.finalize_spawned(&request, &mut pending).is_some());
        self.add_spawned(pending, false);
        horizon_client_core::nbt_value::text(self.persist.raw.get(&self.next_entity)
            .and_then(|tag| tag.get("Variant"))).unwrap_or_default()
    }

    pub(in crate::game) fn naturalist_fixture_aquatic_tables(&self) -> String {
        let biomes = world_biomes!(self);
        let mut rows = std::collections::BTreeMap::new();
        for name in ["ocean", "warm_ocean", "frozen_ocean", "deep_frozen_ocean", "river", "swamp", "jungle"] {
            let biome = biomes.get(biomes.id(&format!("minecraft:{name}")));
            let mut table = std::collections::BTreeMap::new();
            if let Some(spawns) = biome.spawns.get("water_ambient") {
                for entry in spawns.iter().filter(|entry| entry.entity.starts_with("naturalist:")) {
                    table.insert(entry.entity.clone(), [entry.weight, entry.min, entry.max]);
                }
            }
            rows.insert(name, table);
        }
        serde_json::to_string(&rows).unwrap()
    }

    pub(in crate::game) fn naturalist_fixture_aquatic_limits(&self) -> String {
        use horizon_client::mobs::spawn_selection::{max_cluster_size, naturalist_aquatic_spawn_limits};
        let level = self.natural_level();
        let mut caps = std::collections::BTreeMap::new();
        naturalist_aquatic_spawn_limits::natural_pass(|| {
            for entity in ["naturalist:anglerfish", "naturalist:bass", "naturalist:blobfish",
                "naturalist:catfish", "naturalist:jellyfish", "naturalist:piranha", "naturalist:ray",
                "minecraft:cod", "minecraft:tadpole"]
            {
                let (species, _, _, _) = level.mob_type(entity).unwrap();
                caps.insert(entity, max_cluster_size(species));
            }
        });
        serde_json::to_string(&caps).unwrap()
    }

    pub(in crate::game) fn naturalist_fixture_bass_termination(&self) -> String {
        use horizon_client::mobs::spawn_selection::naturalist_aquatic_spawn_limits::natural_pass;
        use horizon_client::mobs::nav::naturalist_bass_state as bass;
        let mut maximum = 0;
        let mut total = 0;
        let mut groups_at_target = 0;
        let mut extra_callbacks = 0;
        for seed in 0..256 {
            let mut random = LegacyRandom::new(seed);
            let players = vec![SpawnPlayer { position: [8.5, 92.0, 6.5], spectator: false }];
            let level = self.natural_level();
            let mut state = SpawnState::new(&level, 289, players, std::iter::empty::<Counted<'_>>(), |_x, _z| true);
            let mut pending = Vec::new();
            let mut completed = false;
            let mut spawn = |l: &GameLevel<'_, '_>, r: &SpawnRequest| {
                if r.entity != "naturalist:bass" {
                    return None;
                }
                if r.group_index == 0 {
                    completed = false;
                } else if completed {
                    extra_callbacks += 1;
                }
                let result = l.game.finalize_spawned(r, &mut pending);
                if let Some(result) = result {
                    l.pending.borrow_mut().push(result.bounds);
                    let group = bass::group(&pending.last().unwrap().ai.nav).unwrap();
                    if group.index + 1 == group.target {
                        groups_at_target += 1;
                        completed = true;
                    }
                }
                result
            };
            natural_pass(|| spawn_selection::spawn_category_for_position(&level, &mut random, &mut state,
                Category::WaterAmbient, (3, 0), [48, 55, 8], &mut spawn, &mut |_| {}));
            maximum = maximum.max(pending.len());
            total += pending.len();
        }
        serde_json::json!({"maximum_per_pass": maximum, "total_bass": total,
            "groups_at_target": groups_at_target, "attempts_after_target": extra_callbacks}).to_string()
    }
}
