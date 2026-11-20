use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use bird_animations::*;
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static ADULT: RefCell<ModelPart> = RefCell::new(bird_models::adult());
    static BABY: RefCell<ModelPart> = RefCell::new(bird_models::baby());
    static POSES: RefCell<HashMap<u32, BirdPose>> = RefCell::new(HashMap::new());
}

struct BirdPose {
    clocks: [AnimationClock; 3],
    age: i32,
    pitch: [f32; 2],
    last_seen: Instant,
}

impl BirdPose {
    fn new(now: Instant) -> Self {
        Self {
            clocks: Default::default(),
            age: -1,
            pitch: [0.0; 2],
            last_seen: now,
        }
    }

    fn tick(&mut self, mob: &Mob, sitting: bool, now: Instant) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;
        let flying = !sitting && !mob.on_ground;
        let walking = !sitting
            && !flying
            && mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z > 1.0e-6;
        for (clock, active) in self
            .clocks
            .iter_mut()
            .zip([!flying && !walking, walking, flying])
        {
            clock.advance_with_rate(active, age, 0.5);
        }

        let [_, pitch] = self.pitch;
        let target = if mob.on_ground {
            0.0
        } else {
            mob.head_pitch * (std::f32::consts::PI / 360.0) - mob.velocity.y as f32 * 3.0
        };
        self.pitch = [pitch, pitch + (target - pitch) * 0.3];
    }

    fn apply(&mut self, model: &mut ModelPart, mob: &Mob, render: &NativeMobRender<'_>) {
        let [idle, walk, fly] = if render.baby {
            [&BABY_IDLE, &BABY_WALK, &ADULT_FLY]
        } else {
            [&ADULT_IDLE, &ADULT_WALK, &ADULT_FLY]
        };
        let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
        let speed = (render.walk_speed / gait).clamp(0.4, 2.0);
        let idle_weight = 1.0 - (render.walk_speed * 2.5).min(1.0);
        self.clocks[0].draw(idle, model, render.age, render.partial, 1.0, idle_weight);
        self.clocks[1].draw(walk, model, render.age, render.partial, speed, 1.0);
        if !render.baby {
            self.clocks[2].draw(fly, model, render.age, render.partial, 1.0, 1.0);
            if let Some(head) = bone(model, "head") {
                head.x_rot += render.pitch.to_radians();
                head.y_rot += render.head_yaw.to_radians();
            }
        }
        if !mob.on_ground
            && let Some(root) = bone(model, "root")
        {
            let [old, pitch] = self.pitch;
            root.x_rot += old + (pitch - old) * render.partial;
            root.z_rot = render.head_yaw * (std::f32::consts::PI / 600.0);
        }
    }
}

fn pose(poses: &mut HashMap<u32, BirdPose>, id: u32, now: Instant) -> &mut BirdPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| BirdPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_bird_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self
                .mobs
                .iter()
                .filter(|mob| !mob.removed && mob.kind.id == "naturalist:bird")
            {
                let (_, _, sitting) =
                    super::naturalist_deer_variants::duck_appearance(self, mob.id, mob.is_baby());
                pose(poses, mob.id, now).tick(mob, sitting, now);
            }
        });
    }

    pub(super) fn draw_naturalist_bird(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let model = if render.baby { &BABY } else { &ADULT };
        let texture = appearance_texture(self, mob.id, render.baby);
        let texture = texture
            .as_deref()
            .unwrap_or("naturalist:entity/bird/american_robin");
        let (_, guest_dye, _) =
            super::naturalist_deer_variants::duck_appearance(self, mob.id, render.baby);
        let dye = if self.remote.is_some() {
            guest_dye
        } else {
            Some(mob.extra.color)
        };
        model.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, mob.extra.sitting_pose, now);
                }
                pose.apply(model, mob, render);
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 {
                0xFF7070
            } else {
                0xFFFFFF
            };
            let drawn = self.native_draw_model_part(draw, model, texture, tint);
            if let Some(color) =
                dye.and_then(|id| horizon_client::mobs::render::DYE_NAMES.get(usize::from(id)))
            {
                self.native_draw_model_part(
                    draw,
                    model,
                    &format!("naturalist:entity/bird/dye/{color}"),
                    0xFFFFFF,
                );
            }
            drawn
        })
    }
}

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_bird_pose(&self) -> usize {
        let Some(kind) = self.res.mob_types().get("naturalist:bird").cloned() else {
            return 0;
        };
        let mut mob = Mob::new(0, kind, horizon_core::geometry::Vec3::ZERO, 0.0);
        let mut model = bird_models::adult();
        let now = Instant::now();
        let mut sample = BirdPose::new(now);
        let mut frames = 0;
        for tick in 1..=48 {
            let walking = (9..20).contains(&tick);
            let flying = (20..36).contains(&tick);
            let (speed, pitch, yaw, vertical) = (
                if walking { 0.26 } else { 0.0 },
                if flying { 22.0 } else { 6.0 },
                if flying { -16.0 } else { 8.0 },
                if flying { -0.22 } else { 0.0 },
            );
            mob.age = tick as _;
            mob.on_ground = !flying;
            mob.head_pitch = pitch;
            mob.previous_head_pitch = pitch;
            mob.head_yaw = yaw;
            mob.previous_head_yaw = yaw;
            mob.limb_speed = speed;
            mob.previous_limb_speed = speed;
            mob.velocity =
                horizon_core::geometry::Vec3::new(if walking { 0.1 } else { 0.0 }, vertical, 0.0);
            sample.tick(&mob, false, now);
            for partial in [0.0, 0.5, 0.9] {
                model.reset_all_poses();
                let frame = NativeMobRender::of(&mob, partial);
                sample.apply(&mut model, &mob, &frame);
                let weights = sample
                    .clocks
                    .each_ref()
                    .map(|clock| clock.factor_at(partial));
                let time_ms = sample
                    .clocks
                    .each_ref()
                    .map(AnimationClock::started_phase_ms);
                let bones = [
                    "root",
                    "head",
                    "leftWingRot",
                    "rightWingRot",
                    "left_leg",
                    "right_leg",
                ]
                .into_iter()
                .map(|name| {
                    let part = bone(&mut model, name).unwrap().store_pose();
                    let data = [part.x, part.y, part.z, part.x_rot, part.y_rot, part.z_rot];
                    (name, data)
                })
                .collect::<std::collections::BTreeMap<_, _>>();
                println!(
                    "naturalist_bird_pose {}",
                    serde_json::json!({
                        "tick": tick, "partial": partial, "weights": weights,
                        "time_ms": time_ms, "bones": bones,
                    })
                );
                frames += 1;
            }
        }
        frames
    }
}
