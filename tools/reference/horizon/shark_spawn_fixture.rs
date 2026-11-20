use super::*;

impl Game<'_> {
    pub(in crate::game) fn shark_fixture_spawn(&self, position: [f64; 3]) -> bool {
        let request = SpawnRequest {
            entity: "naturalist:great_white_shark",
            species: Species::Tadpole,
            position,
            yaw: 0.0,
            group_index: 0,
            reason: horizon_client::mobs::spawn_rules::Reason::Natural,
        };
        self.finalize_spawned(&request, &mut Vec::new()).is_some()
    }
}
