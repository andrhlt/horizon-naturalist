use super::*;
use horizon_client::mobs::spawn_rules::Reason;

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_piranha_spawn(&self) -> String {
        let sea = horizon_world::level::sea_level_of(self.level_key());
        let can_spawn = |x, y, z| {
            let mut pending = Vec::new();
            let request = SpawnRequest { entity: "naturalist:piranha", species: Species::Tadpole,
                position: [x, y, z], yaw: 0.0, group_index: 0, reason: Reason::Natural };
            self.finalize_spawned(&request, &mut pending).is_some()
        };
        let deep = can_spawn(8.5, f64::from(sea - 30), 8.5);
        let high = can_spawn(8.5, f64::from(sea + 1), 8.5);
        let dry = can_spawn(7.5, f64::from(sea - 30), 8.5);
        let shape = self.natural_level().mob_type("naturalist:piranha");
        assert!(matches!(shape, Some((Species::Tadpole, _, _, Category::WaterAmbient))));
        format!(r#"{{"deep_water":{deep},"above_sea_level":{high},"dry":{dry}}}"#)
    }
}
