use super::Game;
use horizon_client::mobs::{Mob, naturalist_snail::snail_mut};
use horizon_core::geometry::Vec3;

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_snail_surface(&self) -> Option<serde_json::Value> {
        let kind = self.res.mob_types().get("naturalist:snail")?.clone();
        let mut mob = Mob::new(0, kind, Vec3::new(7.2, 93.0, 8.5), 0.0);
        let world = self.mob_world_base(&[]);
        let mut surface = std::mem::take(&mut snail_mut(&mut mob).surface);
        let mut events = Vec::new();
        let mut wall_tick = -1;
        let mut wall_y = 0.0;
        let mut ceiling_tick = -1;
        let mut ceiling_y = 0.0;
        for tick in 1..=800 {
            let direction = if wall_tick < 0 {
                Vec3::new(1.0, 0.0, 0.0)
            } else {
                Vec3::new(0.0, 1.0, 0.0)
            };
            surface.desired = direction;
            surface.speed = 0.22;
            surface.goal = direction;
            if !surface.travel(&mut mob, &world, &mut events) {
                break;
            }
            surface.tick(&mut mob);
            if wall_tick < 0 && surface.normal.x == -1.0 {
                wall_tick = tick;
                wall_y = mob.position.y;
            }
            if surface.normal.y == -1.0 {
                ceiling_tick = tick;
                ceiling_y = mob.position.y;
                break;
            }
        }
        Some(serde_json::json!({
            "wall_tick": wall_tick, "wall_y": wall_y,
            "ceiling_tick": ceiling_tick, "ceiling_y": ceiling_y,
            "attached": surface.attached,
        }))
    }
}
