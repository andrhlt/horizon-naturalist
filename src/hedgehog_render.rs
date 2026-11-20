use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{appearance_texture, capybara_appearance},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart, render::DYE_NAMES};
use std::{cell::RefCell, collections::HashMap};

use hedgehog_animations::{
    ADULT_HIDE, ADULT_IDLE, ADULT_IDLE_EVENT, ADULT_ROLL_AIR, ADULT_ROLL_GROUND, ADULT_SIT,
    ADULT_UNHIDE, ADULT_WALK,
};

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(hedgehog_models::adult());
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

struct Pose {
    clocks: [AnimationClock; 6],
    tick: i32,
    hidden: bool,
    hide_started: i32,
    unhide_started: i32,
    rolled_ticks: u16,
    idle_event: u8,
    random: horizon_core::random::LegacyRandom,
}

impl Pose {
    fn new(id: u32) -> Self {
        Self {
            clocks: Default::default(),
            tick: -1,
            hidden: false,
            hide_started: 0,
            unhide_started: i32::MIN / 2,
            rolled_ticks: 0,
            idle_event: 0,
            random: horizon_core::random::LegacyRandom::new(i64::from(id)),
        }
    }

    fn tick(&mut self, mob: &Mob, sitting: bool, bits: u8) {
        use horizon_core::random::RandomSource;
        let tick = mob.age as i32;
        if tick < self.tick {
            *self = Self::new(mob.id);
        }
        if tick == self.tick {
            return;
        }
        self.tick = tick;
        let air = bits & 2 != 0;
        let hidden = !air && bits & 1 != 0;
        if hidden != self.hidden {
            if hidden {
                self.hide_started = tick;
            } else {
                self.unhide_started = tick;
            }
        }
        self.hidden = hidden;
        let unhiding = !air && !hidden && tick - self.unhide_started < 9;
        let sitting = !air && !hidden && !unhiding && sitting;
        let moving = mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z > 1e-6;
        let ground = !air && !hidden && !unhiding && !sitting && moving && bits & 4 != 0;
        let posing = air || hidden || unhiding || sitting || ground;
        let rolled = air || hidden || ground || (unhiding && tick - self.unhide_started < 8);
        self.rolled_ticks = if rolled { self.rolled_ticks.saturating_add(1) } else { 0 };
        let idle = !posing && !moving;
        if !idle {
            self.idle_event = 0;
        } else if self.idle_event > 0 {
            self.idle_event -= 1;
        } else if self.random.next_int_bound(1000).ok() == Some(0) {
            self.idle_event = 18;
        }
        let active = [sitting, ground, air, idle && self.idle_event > 0, idle && self.idle_event == 0, !posing && moving];
        for (index, clock) in self.clocks.iter_mut().enumerate() {
            let rate = if index == 0 { 0.25 } else { 0.5 };
            clock.advance_with_rate(active[index], tick, rate);
        }
    }

    fn draw(&mut self, model: &mut ModelPart, mob: &Mob, render: &NativeMobRender<'_>) {
        let gait = ((mob.attrs.speed * 8.64).min(1.0) * 0.65) as f32;
        let walk = 2.5 * (render.walk_speed / gait.max(0.05)).clamp(0.4, 2.0);
        let fade = 1.0 - (render.walk_speed * 2.5).min(1.0);
        let clips = [&ADULT_SIT, &ADULT_ROLL_GROUND, &ADULT_ROLL_AIR, &ADULT_IDLE_EVENT, &ADULT_IDLE, &ADULT_WALK];
        for (index, clock) in self.clocks.iter_mut().enumerate() {
            let speed = if index == 5 { walk } else { 1.0 };
            let scale = if index == 4 { fade } else { 1.0 };
            clock.draw(clips[index], model, render.age, render.partial, speed, scale);
            if index == 0 {
                if self.hidden {
                    ADULT_HIDE.pose(model, (render.age - self.hide_started as f32).max(0.0) / 20.0, 1.0);
                } else if self.tick - self.unhide_started < 9 {
                    ADULT_UNHIDE.pose(model, (render.age - self.unhide_started as f32).max(0.0) / 20.0, 1.0);
                }
            }
        }
        if let Some(rolled) = model.get_child_mut("root").and_then(|root| root.get_child_mut("rolled")) {
            rolled.visible = self.rolled_ticks >= 3;
        }
    }
}

impl Game<'_> {
    pub(super) fn tick_naturalist_hedgehog_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| mob.kind.id == "naturalist:hedgehog" && !mob.removed) {
                let (_, sitting, bits) = capybara_appearance(self, mob.id);
                poses.entry(mob.id).or_insert_with(|| Pose::new(mob.id)).tick(mob, sitting, bits);
            }
            if poses.len() > 1024 {
                poses.retain(|id, _| self.mobs.iter().any(|mob| mob.id == *id && !mob.removed));
            }
        });
    }

    pub(super) fn draw_naturalist_hedgehog(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let texture = appearance_texture(self, mob.id, false);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/hedgehog/brown");
        let (dye, sitting, bits) = capybara_appearance(self, mob.id);
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let pose = poses.entry(mob.id).or_insert_with(|| Pose::new(mob.id));
                pose.tick(mob, sitting, bits);
                pose.draw(model, mob, render);
            });
            if render.baby {
                draw.scale([0.5; 3]);
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xff7070 } else { 0xffffff };
            let rendered = self.native_draw_model_part(draw, model, texture, tint);
            if let Some(dye) = dye.and_then(|index| DYE_NAMES.get(usize::from(index))) {
                self.native_draw_model_part(draw, model, &format!("naturalist:entity/hedgehog/dye/{dye}"), tint);
            }
            rendered
        })
    }
}
