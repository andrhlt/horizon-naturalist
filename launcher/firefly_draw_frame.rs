use super::{BlockEntityDraw, Game, ModelPart, draw_model_part};

impl Game<'_> {
    pub(in crate::game) fn draw_naturalist_firefly_frame(&self, draw: &mut BlockEntityDraw, part: &ModelPart, frame: u8, emissive: bool) {
        for (key, glow) in [
            ("naturalist:entity/firefly_glow", false),
            ("naturalist:entity/firefly_glow_e", true),
        ] {
            if glow && !emissive {
                continue;
            }
            let Some(mut sprite) = self.gui.atlas.sprites.get(key).copied() else { continue };
            let height = (sprite.v1 - sprite.v0) / 30.0;
            sprite.v0 += height * f32::from(frame);
            sprite.v1 = sprite.v0 + height;
            let original_light = draw.light;
            if glow {
                draw.light = 240 | (240 << 8);
            }
            draw_model_part(draw, part, sprite, 0xFFFFFF);
            draw.light = original_light;
        }
    }
}
