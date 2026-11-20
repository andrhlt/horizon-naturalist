use super::{
    Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture, naturalist_preview::{Channel, Clip, Interpolation, Target},
};
use horizon_client::mobs::model_layers::ModelPart;
use std::cell::RefCell;

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(caterpillar_models::adult());
}

impl Game<'_> {
    pub(super) fn draw_naturalist_caterpillar(&self, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let texture = appearance_texture(self, render.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/caterpillar");
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            let idle_strength = 1.0 - (render.walk_speed * 2.5).min(1.0);
            caterpillar_animations::ADULT_IDLE.pose(model, render.age / 20.0, idle_strength);
            if render.walk_speed > 0.0 {
                caterpillar_animations::ADULT_CRAWL.pose(model, render.walk_position / 20.0, 1.0);
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, model, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
