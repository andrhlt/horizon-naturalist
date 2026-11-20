use super::*;
use horizon_client::mobs::spawn_rules::Reason;

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_blobfish_spawn(&self) -> String {
        let sea = horizon_world::level::sea_level_of(self.level_key());
        let can_spawn = |y| {
            let mut pending = Vec::new();
            let request = SpawnRequest { entity: "naturalist:blobfish", species: Species::Tadpole,
                position: [8.5, y, 8.5], yaw: 0.0, group_index: 0, reason: Reason::Natural };
            self.finalize_spawned(&request, &mut pending).is_some()
        };
        let at = can_spawn(f64::from(sea - 33));
        let above = can_spawn(f64::from(sea - 32));
        let shape = self.natural_level().mob_type("naturalist:blobfish");
        assert!(matches!(shape, Some((Species::Tadpole, _, _, Category::WaterAmbient))));
        format!(r#"{{"at_threshold":{at},"above_threshold":{above}}}"#)
    }
}
