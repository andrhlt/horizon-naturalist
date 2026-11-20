use super::*;
use horizon_client::mobs::spawn_rules::Reason;

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_clam_spawn(&self) -> String {
        use horizon_client::mobs::spawn_selection::{max_cluster_size, naturalist_aquatic_spawn_limits};
        let level = self.natural_level();
        let cluster = naturalist_aquatic_spawn_limits::natural_pass(|| {
            let (species, _, _, _) = level.mob_type("naturalist:clam").unwrap();
            max_cluster_size(species)
        });
        let placement = |y| {
            let request = SpawnRequest { entity: "naturalist:clam", species: Species::Tadpole,
                position: [8.5, y, 8.5], yaw: 0.0, group_index: 0, reason: Reason::Natural };
            self.finalize_spawned(&request, &mut Vec::new()).is_some()
        };
        serde_json::json!({
            "cluster": cluster, "supported_water": placement(30.0),
            "open_water": placement(31.0), "sea_surface": placement(63.0),
        }).to_string()
    }
}
