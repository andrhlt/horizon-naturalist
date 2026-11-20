use super::*;
use horizon_client::mobs::spawn_rules::Reason;
use std::collections::BTreeSet;

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_butterfly_group(&mut self) -> String {
        let mut variants = BTreeSet::new();
        let mut same = false;
        let mut distinct = false;
        let mut shared = true;
        let mut adults = true;
        let mut in_pool = true;
        let mut applied_palette = true;
        for sample in 0..128 {
            let mut pending = Vec::new();
            let mut selected = Vec::new();
            let mut leader_palette = None;
            for group_index in 0..6 {
                let position = [
                    56.5 + f64::from(group_index),
                    93.0,
                    58.5 + f64::from(sample),
                ];
                let request = SpawnRequest {
                    entity: "naturalist:butterfly",
                    species: Species::Parrot,
                    position,
                    yaw: 0.0,
                    group_index,
                    reason: Reason::Natural,
                };
                if self.finalize_spawned(&request, &mut pending).is_none() {
                    return "runtime did not finalize a Butterfly".to_owned();
                }
                let mob = pending.last().unwrap();
                let Some((palette, choice)) = mob
                    .ai
                    .nav
                    .__horizon_mixins_naturalist_butterfly_spawn_pool()
                else {
                    return "runtime did not assign a Butterfly palette".to_owned();
                };
                if group_index == 0 {
                    same |= palette[0] == palette[1];
                    distinct |= palette[0] != palette[1];
                    leader_palette = Some(palette.clone());
                }
                let selected_variant = palette.get(*choice).cloned();
                in_pool &= selected_variant.is_some();
                variants.extend(selected_variant.clone());
                selected.push(selected_variant);
                shared &= leader_palette.as_ref() == Some(palette);
                adults &= !mob.is_baby();
            }
            if sample < 2 {
                let before = self.mobs.len();
                self.add_spawned(pending, false);
                for (mob, choice) in self.mobs[before..].iter().zip(selected) {
                    let saved = self
                        .persist
                        .raw
                        .get(&mob.id)
                        .and_then(|tag| tag.get("Variant"));
                    applied_palette &= choice == horizon_client_core::nbt_value::text(saved);
                }
            }
        }
        serde_json::json!({
            "all_adult": adults,
            "shares_pool": shared,
            "two_draws_including_duplicates": same && distinct,
            "in_pool": in_pool,
            "applied_palette": applied_palette,
            "observed_variants": variants,
        })
        .to_string()
    }
}
