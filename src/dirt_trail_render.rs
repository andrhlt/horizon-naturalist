use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::animal_rest_pose,
    naturalist_preview::{Channel, Clip, Interpolation, Target},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::cell::RefCell;

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(dirt_trail_models::adult());
}

impl Game<'_> {
    pub(super) fn draw_naturalist_dirt_trail(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let small = if self.remote.is_some() {
            animal_rest_pose(self, mob.id) != 0
        } else {
            mob.extra.variant != 0
        };
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            dirt_trail_animations::ADULT_SPAWN.pose(model, (render.age / 20.0).min(0.625), 1.0);
            draw.translate([0.0, -((mob.id % 3) as f32) * 0.0625 + 0.01, 0.0]);

            let yaw = (mob.id.wrapping_mul(61) % 360) as f32;
            draw.rotate_about([0.0, 1.0, 0.0], render.body_yaw - yaw, [0.0; 3]);
            let scale = if small { 0.6 } else { 1.0 };
            draw.scale([scale, -scale, -scale]);
            draw.translate([0.0, -1.5, 0.0]);
            self.native_draw_model_part(draw, model, "naturalist:entity/mole", 0xFFFFFF)
        })
    }
}
