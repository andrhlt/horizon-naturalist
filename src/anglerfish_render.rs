use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_bass::BassPose, naturalist_deer_variants::angler_appearance,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone}};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

struct AnglerfishPose {
    swim: BassPose,
    clocks: [AnimationClock; 4],
    last_swim: [Option<i64>; 2],
    was_swinging: bool,
    attack: i32,
    last_seen: Instant,
}

impl AnglerfishPose {
    fn new(now: Instant) -> Self {
        Self { swim: BassPose::new(now), clocks: Default::default(), last_swim: [None; 2],
            was_swinging: false, attack: 0, last_seen: now }
    }

    fn tick(&mut self, mob: &Mob, has_target: bool, now: Instant) -> usize {
        let age = mob.age as i32;
        if age < self.swim.age { *self = Self::new(now); }
        self.last_seen = now;
        if age == self.swim.age { return 0; }
        let water = mob.fluids.in_water;
        if mob.ai.swinging && !self.was_swinging {
            self.attack = 7;
        }
        self.was_swinging = mob.ai.swinging;
        self.clocks[0].advance_with_rate(self.attack > 0, age, 1.0);
        self.attack = (self.attack - 1).max(0);
        self.clocks[1].advance_with_rate(!water, age, 0.5);
        self.clocks[2].advance_with_rate(water && !has_target, age, 0.5);
        self.clocks[3].advance_with_rate(water && has_target, age, 0.5);
        let mut cues = 0;
        for (index, period) in [1000, 500].into_iter().enumerate() {
            let current = self.clocks[index + 2].started_phase_ms();
            if current.is_some_and(|phase| (phase - 50).div_euclid(period)
                > (self.last_swim[index].filter(|old| *old <= phase).unwrap_or(-1) - 50).div_euclid(period))
            {
                cues += 1;
            }
            self.last_swim[index] = current;
        }
        self.swim.tick(mob, now);
        cues
    }
}

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(anglerfish_models::adult());
    static POSES: RefCell<HashMap<u32, AnglerfishPose>> = RefCell::new(HashMap::new());
}

fn pose(poses: &mut HashMap<u32, AnglerfishPose>, id: u32, now: Instant) -> &mut AnglerfishPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| AnglerfishPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_anglerfish_animations(&mut self) {
        if self.remote.is_none() { return; }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed && mob.kind.id == "naturalist:anglerfish") {
                let (_, _, has_target) = angler_appearance(self, mob);
                for _ in 0..pose(poses, mob.id, now).tick(mob, has_target, now) {
                    self.audio.play_unshared("naturalist:entity.anglerfish.swim", Some(mob.position.components()), 0.3, 1.0);
                }
            }
        });
    }

    pub(super) fn draw_naturalist_anglerfish(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        let (texture, glowing, has_target) = angler_appearance(self, mob);
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() { pose.tick(mob, has_target, now); }
                use anglerfish_animations::{ADULT_ATTACK, ADULT_FLOP, ADULT_SWIM, ADULT_SWIM_FAST};
                let gait = (render.walk_speed / 0.25).clamp(0.4, 2.0);
                pose.clocks[0].draw(&ADULT_ATTACK, model, render.age, render.partial, 1.0, 1.0);
                pose.clocks[1].draw(&ADULT_FLOP, model, render.age, render.partial, 1.0, 1.0);
                pose.clocks[2].draw(&ADULT_SWIM, model, render.age, render.partial, gait, 1.0);
                pose.clocks[3].draw(&ADULT_SWIM_FAST, model, render.age, render.partial, gait, 1.0);
                let lerp = |[old, new]: [f32; 2]| old + (new - old) * render.partial;
                if let Some(body) = bone(model, "body") {
                    body.x_rot += lerp(pose.swim.pitch).to_radians();
                }
                draw.rotate_about([0.0, 0.0, 1.0], -lerp(pose.swim.tilt), [0.0; 3]);
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            if glowing { draw.light = 0xF0F0; }
            self.native_draw_model_part(draw, model, &texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
