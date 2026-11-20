use super::{
    Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture, naturalist_preview::{Channel, Clip, Interpolation, Target},
};
use horizon_client::mobs::{goals::naturalist_firefly_behavior::glow_frame, model_layers::ModelPart};
use std::cell::RefCell;

thread_local! {
    static ADULT: RefCell<ModelPart> = RefCell::new(firefly_models::adult());
    static BABY: RefCell<ModelPart> = RefCell::new(firefly_models::baby());
}

impl Game<'_> {
    pub(super) fn draw_naturalist_firefly(&self, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let texture = appearance_texture(self, render.id, render.baby);
        let texture = texture.as_deref().unwrap_or(if render.baby {
            "naturalist:entity/firefly_baby"
        } else {
            "naturalist:entity/firefly"
        });
        let model = if render.baby { &BABY } else { &ADULT };
        model.with_borrow_mut(|model| {
            model.reset_all_poses();
            if render.baby {
                let idle = 1.0 - (render.walk_speed * 2.5).min(1.0);
                firefly_animations::BABY_IDLE.pose(model, render.age / 20.0, idle);
                if render.walk_speed > 0.0 {
                    firefly_animations::BABY_WALK.pose(model, render.walk_position / 20.0, 1.0);
                }
            } else {
                firefly_animations::ADULT_FLY.pose(model, render.age / 20.0, 1.0);
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            let drawn = self.native_draw_model_part(draw, model, texture, tint);
            let (frame, emissive) = self.mobs.iter().find(|mob| mob.id == render.id)
                .map(glow_frame).unwrap_or((0, false));
            self.draw_naturalist_firefly_frame(draw, model, frame, emissive);
            drawn
        })
    }
}
