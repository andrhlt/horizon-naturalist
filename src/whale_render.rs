use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{
    Mob, living::wrap, model_layers::ModelPart, nav::naturalist_whale_state as whale,
};
use horizon_core::geometry::Aabb;
use std::{cell::RefCell, collections::HashMap};

#[derive(Default)]
struct Pose {
    motion: whale::Motion,
    previous: whale::Motion,
    clocks: [AnimationClock; 3],
    tick: Option<u32>,
}

impl Pose {
    fn tick(&mut self, game: &Game<'_>, mob: &Mob) {
        let age = mob.age;
        if self.tick.is_some_and(|tick| age < tick) {
            *self = Self::default();
        }
        if self.tick == Some(age) {
            return;
        }
        self.previous = self.motion.clone();
        if let Some(motion) = whale::motion(&mob.ai.nav) {
            self.motion = motion.clone();
        } else {
            self.motion.tick(mob, |position| {
                let [x, y, z] = position.components().map(|value| value.floor() as i32);
                game.res.facts().get(game.world.block(x, y, z)).fluid
                    == horizon_client::blocks::Fluid::Water
            });
        }
        if self.tick.is_none() {
            self.previous = self.motion.clone();
        }
        let wet = mob.fluids.in_water;
        let moving = mob.velocity.horizontal_distance_squared() > 2.5e-5;
        for (clock, enabled) in self
            .clocks
            .iter_mut()
            .zip([!wet, wet && !moving, wet && moving])
        {
            clock.advance_with_rate(enabled, age as i32, 1.0);
        }
        self.tick = Some(age);
    }
}

thread_local! {
    static MODELS: RefCell<[ModelPart; 2]> = RefCell::new([whale_models::adult(), whale_models::baby()]);
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

impl Game<'_> {
    pub(in crate::game) fn whale_part_boxes(&self, mob: &Mob) -> [Aabb; 5] {
        if self.remote.is_some() {
            return POSES.with_borrow(|poses| {
                poses.get(&mob.id).map_or_else(
                    || whale::parts(mob, &whale::Motion::default()),
                    |pose| whale::parts(mob, &pose.motion),
                )
            });
        }
        whale::parts(
            mob,
            whale::motion(&mob.ai.nav).unwrap_or(&whale::Motion::default()),
        )
    }

    pub(super) fn tick_naturalist_whale_animations(&mut self) {
        POSES.with_borrow_mut(|poses| {
            if self.mobs.len() < poses.len() {
                poses.retain(|id, _| self.mobs.iter().any(|mob| mob.id == *id && !mob.removed));
            }
            for mob in self
                .mobs
                .iter()
                .filter(|mob| !mob.removed && mob.kind.id == whale::KIND)
            {
                poses.entry(mob.id).or_default().tick(self, mob);
            }
        });
    }

    pub(super) fn draw_naturalist_whale(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        use whale_animations::*;
        let baby = render.baby;
        let texture = appearance_texture(self, render.id, baby);
        let defaults = if baby {
            "naturalist:entity/whale/whale_baby"
        } else {
            "naturalist:entity/whale/whale"
        };
        MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(baby)];
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let pose = poses.entry(mob.id).or_default();
                pose.tick(self, mob);
                let clips = if baby {
                    [&BABY_FLOP, &BABY_SWIM_IDLE, &BABY_SWIM]
                } else {
                    [&ADULT_FLOP, &ADULT_IDLE, &ADULT_SWIM]
                };
                let gait = (render.walk_speed / 0.5).clamp(0.4, 2.0) * if baby { 2.0 } else { 1.0 };
                for (index, (clock, clip)) in pose.clocks.iter_mut().zip(clips).enumerate() {
                    clock.draw(
                        clip,
                        model,
                        render.age,
                        render.partial,
                        if index == 2 { gait } else { 1.0 },
                        1.0,
                    );
                }
                let state = &pose.motion;
                let old = &pose.previous;
                let interpolate = |a, b| a + (b - a) * render.partial;
                let angle = |a, b| a + wrap(b - a) * render.partial;
                let yaw = angle(old.yaw, state.yaw);
                let pitch: f32 = interpolate(old.pitch, state.pitch);
                let roll = f32::to_radians(interpolate(old.roll, state.roll));
                draw.rotate_about([0.0, 1.0, 0.0], wrap(render.body_yaw - yaw), [0.0; 3]);
                if let Some(root) = bone(model, "root") {
                    root.x_rot += pitch.to_radians();
                    root.z_rot += roll;
                }
                let segments: &[&str] = if baby {
                    &["skull", "tail", "flip"]
                } else {
                    &["skullRot", "tail", "tail2", "fluke"]
                };
                for (i, name) in segments.iter().enumerate() {
                    let Some(part) = bone(model, name) else {
                        continue;
                    };
                    let y = angle(old.segment_yaw[i], state.segment_yaw[i]);
                    let x = interpolate(old.segment_pitch[i], state.segment_pitch[i]);
                    let (parent_y, parent_x) = if i <= 1 {
                        (yaw, pitch)
                    } else {
                        (
                            angle(old.segment_yaw[i - 1], state.segment_yaw[i - 1]),
                            interpolate(old.segment_pitch[i - 1], state.segment_pitch[i - 1]),
                        )
                    };
                    part.y_rot += wrap(y - parent_y).to_radians();
                    part.x_rot += (x - parent_x).to_radians();
                    if i == 0 {
                        part.x_rot -= interpolate(old.front_droop, state.front_droop)
                            * std::f32::consts::FRAC_PI_4;
                        part.z_rot -= 0.35 * roll;
                    } else {
                        part.x_rot += interpolate(old.back_droop, state.back_droop)
                            * std::f32::consts::FRAC_PI_2;
                        if i + 1 == segments.len() {
                            part.z_rot += 0.4 * roll;
                        }
                    }
                }
                let fin = (pitch - interpolate(old.fin_lag, state.fin_lag)).to_radians() * 1.6;
                for name in ["leftFin", "rightFin"] {
                    if let Some(part) = bone(model, name) {
                        part.x_rot -= fin;
                    }
                }
            });
            let scale = mob.model_scale();
            draw.scale([scale, -scale, -scale]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(
                draw,
                model,
                texture.as_deref().unwrap_or(defaults),
                if render.hurt_time > 0 {
                    0xFF7070
                } else {
                    0xFFFFFF
                },
            )
        })
    }
}
