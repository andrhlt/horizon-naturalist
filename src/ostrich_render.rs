use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{appearance_texture, capybara_appearance},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart, render::DYE_NAMES};
use std::{cell::RefCell, collections::HashMap};

use ostrich_animations::*;

thread_local! {
    static ADULT: RefCell<ModelPart> = RefCell::new(ostrich_models::adult());
    static BABY: RefCell<ModelPart> = RefCell::new(ostrich_models::baby());
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

struct Pose {
    age: i32,
    clocks: [AnimationClock; 7],
    cue_phase: [Option<f32>; 4],
}

impl Default for Pose {
    fn default() -> Self {
        Self { age: -1, clocks: Default::default(), cue_phase: [None; 4] }
    }
}

impl Pose {
    fn tick(&mut self, mob: &Mob, sitting: bool, flags: u8, ridden: bool) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::default();
        }
        if age == self.age {
            return;
        }
        self.age = age;
        let flapping = !mob.on_ground && !mob.fluids.in_water && !mob.is_baby();
        let sitting = !flapping && sitting;
        let moving = !flapping && !sitting
            && mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z > 1.0e-6;
        let running = moving && (ridden || flags & 2 != 0);
        let bury = !flapping && !sitting && !moving && flags & 1 != 0;
        let all = [flags & 4 != 0, sitting, flapping, bury, !flapping && !sitting && !moving && !bury,
                   moving && !running, running];
        for (index, (clock, active)) in self.clocks.iter_mut().zip(all).enumerate() {
            clock.advance_with_rate(active, age, if index == 0 { 1.0 } else if index == 1 { 0.25 } else { 0.5 });
        }
    }

    fn cues(&mut self) -> Vec<(&'static str, f32)> {
        let tracks: [(usize, f32, &'static str, &[(f32, f32)]); 4] = [
            (5, 1.0, "naturalist:entity.ostrich.step", &[(0.0, 0.35), (0.54, 0.75)]),
            (6, 0.6667, "naturalist:entity.ostrich.step", &[(0.0, 0.35), (0.33, 0.75)]),
            (3, 0.0, "naturalist:entity.ostrich.bury", &[(0.04, 1.0)]),
            (0, 0.0, "naturalist:entity.ostrich.attack", &[(0.0, 1.0)]),
        ];
        let mut sounds = Vec::new();
        for (index, (clock, period, sound, keys)) in tracks.into_iter().enumerate() {
            let now = self.clocks[clock].started_phase_ms().map(|time| time as f32 / 1000.0);
            let old = std::mem::replace(&mut self.cue_phase[index], now);
            let Some(now) = now else { continue };
            let old = old.filter(|old| *old <= now).unwrap_or(-0.0001);
            for &(key, volume) in keys {
                let crossed = if period == 0.0 {
                    old < key && key <= now
                } else {
                    ((now - key) / period).floor() > ((old - key) / period).floor()
                };
                if crossed {
                    sounds.push((sound, volume));
                }
            }
        }
        sounds
    }

    fn draw(&mut self, model: &mut ModelPart, render: &NativeMobRender<'_>) {
        let (clips, rates): (&[(usize, &Clip)], [f32; 2]) = if render.baby {
            (&[(1, &BABY_SIT_IDLE), (4, &BABY_IDLE), (5, &BABY_WALK), (6, &BABY_RUN)], [3.0, 2.0])
        } else {
            (&[(0, &ADULT_ATTACK), (1, &ADULT_SIT), (2, &ADULT_FLAP), (3, &ADULT_BURY_HEAD),
               (4, &ADULT_IDLE), (5, &ADULT_WALK_B), (6, &ADULT_RUN_B_1)], [2.4, 2.0])
        };
        let pace = render.walk_speed.clamp(0.4, 2.0);
        for &(clock, clip) in clips {
            let rate = match clock {
                5 => rates[0] * pace,
                6 => rates[1] * pace,
                _ => 1.0,
            };
            let weight = if clock == 4 { 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0) } else { 1.0 };
            self.clocks[clock].draw(clip, model, render.age, render.partial, rate, weight);
        }
    }
}

fn head_look(model: &mut ModelPart, render: &NativeMobRender<'_>, hiding: bool, ridden: bool) {
    if (render.baby || !hiding || ridden) && let Some(skull) = bone(model, "skull") {
        let (pitch, yaw) = if ridden {
            ((render.pitch * 2.0).clamp(-30.0, 60.0), render.head_yaw.clamp(-60.0, 60.0))
        } else {
            (render.pitch, render.head_yaw)
        };
        skull.x_rot += pitch.to_radians();
        skull.y_rot += yaw.to_radians();
    }
}

impl Game<'_> {
    pub(super) fn tick_naturalist_ostrich_animations(&mut self) {
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| mob.kind.id == "naturalist:ostrich" && !mob.removed) {
                let (_, sitting, bits) = capybara_appearance(self, mob.id);
                let ridden = self.riding == Some(mob.id) || self.ridden_by_guest(mob.id) || mob.extra.rider.is_some();
                let pose = poses.entry(mob.id).or_default();
                pose.tick(mob, sitting, bits, ridden);
                for (sound, volume) in pose.cues() {
                    self.audio.play(sound, Some(mob.position.components()), volume, 1.0);
                }
            }
            if poses.len() > 1024 {
                poses.retain(|id, _| self.mobs.iter().any(|mob| mob.id == *id && !mob.removed));
            }
        });
    }

    pub(super) fn draw_naturalist_ostrich(
        &self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw,
    ) -> bool {
        let asset = if render.baby { &BABY } else { &ADULT };
        let texture = appearance_texture(self, mob.id, render.baby);
        let texture = texture.as_deref().unwrap_or(if render.baby {
            "naturalist:entity/ostrich/ostrich_baby"
        } else {
            "naturalist:entity/ostrich/ostrich"
        });
        let (dye, sitting, bits) = capybara_appearance(self, mob.id);
        let ridden = self.riding == Some(mob.id) || self.ridden_by_guest(mob.id) || mob.extra.rider.is_some();
        asset.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let pose = poses.entry(mob.id).or_default();
                pose.tick(mob, sitting, bits, ridden);
                pose.draw(model, render);
            });
            if let Some(saddle) = bone(model, "saddle") {
                saddle.visible = mob.extra.saddled;
            }
            head_look(model, render, bits & 1 != 0, ridden);
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xff7070 } else { 0xffffff };
            let rendered = self.native_draw_model_part(draw, model, texture, tint);
            if let Some(dye) = dye.and_then(|index| DYE_NAMES.get(usize::from(index))) {
                let baby = if render.baby { "_baby" } else { "" };
                self.native_draw_model_part(draw, model, &format!("naturalist:entity/ostrich/dye/{dye}{baby}"), tint);
            }
            rendered
        })
    }
}
