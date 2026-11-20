use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture, naturalist_preview::{Clip, Channel, Interpolation, Target}};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::cell::RefCell;

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(starfish_models::adult());
}

impl Game<'_> {
    pub(super) fn draw_naturalist_starfish(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let texture = appearance_texture(self, mob.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/starfish/orange");
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            let age = render.age / 20.0;
            let weight = 1.0 - 0.5_f32.powf(render.age.max(0.0));
            starfish_animations::ADULT_IDLE.pose(model, age, weight);
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, model, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
