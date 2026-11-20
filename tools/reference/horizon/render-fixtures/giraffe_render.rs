use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
    naturalist_terrain_legs::TerrainLegs,
};
use giraffe_animations::*;
use horizon_client::mobs::{Mob, MobWorld, model_layers::ModelPart};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static ADULT: RefCell<ModelPart> = RefCell::new(giraffe_models::adult());
    static BABY: RefCell<ModelPart> = RefCell::new(giraffe_models::baby());
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

struct Pose {
    legs: TerrainLegs,
    clocks: [AnimationClock; 3],
    age: i32,
    last_seen: Instant,
}

impl Pose {
    fn new(now: Instant) -> Self {
        Self {
            legs: TerrainLegs::new(0.75, 0.4, 1.3),
            clocks: Default::default(),
            age: -1,
            last_seen: now,
        }
    }

    fn tick(&mut self, mob: &Mob, world: &MobWorld<'_>, ridden: bool, now: Instant) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;
        let motion = mob.velocity;
        let moving = motion.x * motion.x + motion.z * motion.z > 1.0e-6;
        let running = moving && (mob.extra.sprinting || ridden);
        for (clock, active) in self
            .clocks
            .iter_mut()
            .zip([!moving, moving && !running, running])
        {
            clock.advance_with_rate(active, age, 0.5);
        }

        self.legs.update(mob, world);
    }
}

impl Game<'_> {
    pub(super) fn draw_naturalist_giraffe(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let model = if render.baby { &BABY } else { &ADULT };
        let texture = appearance_texture(self, mob.id, render.baby);
        let texture = texture.as_deref().unwrap_or(if render.baby {
            "naturalist:entity/giraffe/giraffe_baby"
        } else {
            "naturalist:entity/giraffe/giraffe"
        });
        model.with_borrow_mut(|model| {
            model.reset_all_poses();
            let world = self.mob_world_base(&[]);
            let ridden = self.riding == Some(mob.id)
                || self.ridden_by_guest(mob.id)
                || mob.extra.rider.is_some();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                if poses.len() > 2048 {
                    poses.retain(|_, pose| {
                        now.duration_since(pose.last_seen) < Duration::from_secs(30)
                    });
                }
                let pose = poses.entry(mob.id).or_insert_with(|| Pose::new(now));
                pose.tick(mob, &world, ridden, now);
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let pace = (render.walk_speed / gait).clamp(0.4, 2.0);
                let (clips, rates) = if render.baby {
                    (
                        [&BABY_IDLE, &BABY_WALK, &BABY_RUN],
                        [1.4, 1.56 * pace, 1.56 * pace],
                    )
                } else {
                    (
                        [&ADULT_IDLE, &ADULT_WALK, &ADULT_RUN],
                        [1.0, 1.3 * pace, 1.43 * pace],
                    )
                };
                for (i, (clip, rate)) in clips.into_iter().zip(rates).enumerate() {
                    let weight = if i == 0 {
                        1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0)
                    } else {
                        1.0
                    };
                    pose.clocks[i].draw(clip, model, render.age, render.partial, rate, weight);
                }
                let (head, legs) = if render.baby {
                    (None, ["leftLeg", "rightLeg", "leftArm", "rightArm"])
                } else {
                    (
                        Some("head"),
                        ["leftFoot", "rightFoot", "leftHand", "rightHand"],
                    )
                };
                pose.legs.articulate(model, head, legs, render.partial);
                if !render.baby
                    && let Some(head) = bone(model, "head")
                {
                    head.x_rot += render.pitch.to_radians();
                    head.y_rot += render.head_yaw.to_radians();
                }
            });
            if let Some(saddle) = bone(model, "saddle") {
                saddle.visible = mob.extra.tame;
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 {
                0xFF7070
            } else {
                0xFFFFFF
            };
            self.native_draw_model_part(draw, model, texture, tint)
        })
    }
}

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_giraffe_terrain(&self) -> Option<serde_json::Value> {
        use horizon_core::geometry::Vec3;
        let world = self.mob_world_base(&[]);
        let kind = self.res.mob_types().get("naturalist:giraffe")?.clone();
        let mut rows = Vec::new();
        for age in [0, -24_000] {
            for yaw in [0.0, 45.0, 90.0] {
                let mut giraffe = Mob::new(0, kind.clone(), Vec3::new(8.15, 100.0, 8.15), yaw);
                giraffe.extra.age = age;
                giraffe.on_ground = true;
                giraffe.body_yaw = yaw;
                let now = Instant::now();
                let mut pose = Pose::new(now);
                for tick in 1..=8 {
                    giraffe.age = tick;
                    pose.tick(&giraffe, &world, false, now);
                    let feet = pose.legs.heights(0.5);
                    let [bl, br, fl, fr] = feet;
                    let rear = (bl + br) * 0.5;
                    let front = (fl + fr) * 0.5;
                    let left = (bl + fl) * 0.5;
                    let right = (br + fr) * 0.5;
                    rows.push(serde_json::json!({"age": age, "yaw": yaw, "tick": tick, "feet": feet,
                        "body": [(front + rear) * 8.0, (front - rear).atan2(1.5), (left - right).atan2(0.8) * 0.5]}));
                }
            }
        }
        Some(serde_json::json!(rows))
    }
}
