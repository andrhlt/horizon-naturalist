use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{animal_rest_pose, appearance_texture},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, goals::naturalist_snake_goals as snake, model_layers::ModelPart};
use horizon_core::random::{LegacyRandom, RandomSource};
use snake_animations::{
    ADULT_ATTACK, ADULT_CLIMB, ADULT_MOVE, ADULT_RATTLE, ADULT_SLEEP, ADULT_TONGUE,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(snake_models::adult());
    static POSES: RefCell<HashMap<u32, SnakePose>> = RefCell::new(HashMap::new());
}

struct SnakePose {
    age: i32,
    seen: Instant,
    clocks: [AnimationClock; 6],
    attack: u8,
    tongue: u8,
    random: LegacyRandom,
}

impl SnakePose {
    fn new(id: u32, seen: Instant) -> Self {
        Self {
            age: -1,
            seen,
            clocks: Default::default(),
            attack: 0,
            tongue: 0,
            random: LegacyRandom::new(i64::from(id)),
        }
    }
    fn update(&mut self, mob: &Mob, now: Instant, flags: u8, walk: f32) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(mob.id, now);
        }
        self.seen = now;
        if self.age == age {
            return;
        }
        self.age = age;
        let resting = flags & 1 != 0;
        let climbing = flags & 4 != 0;
        let sleeping = flags & 16 != 0;
        if self.attack == 0 && mob.ai.swinging {
            self.attack = 10;
        } else {
            self.attack = self.attack.saturating_sub(1);
        }
        if self.tongue > 0 {
            self.tongue -= 1;
        } else if !sleeping && self.random.next_int_bound(1000).unwrap_or(999) < mob.ambient_timer {
            self.tongue = 15;
        }
        let active = [
            self.attack != 0,
            self.tongue != 0,
            flags & 2 != 0,
            resting,
            !resting && climbing,
            !resting && !climbing && walk > 0.04,
        ];
        for (index, (clock, active)) in self.clocks.iter_mut().zip(active).enumerate() {
            clock.advance_with_rate(active, age, [1.0, 1.0, 0.5, 0.25, 0.5, 0.5][index]);
        }
    }
}

impl Game<'_> {
    pub(super) fn draw_naturalist_snake(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let texture = appearance_texture(self, mob.id, false);
        let texture = texture
            .as_deref()
            .unwrap_or("naturalist:entity/snake/green_snake");
        let flags = if self.remote.is_some() {
            animal_rest_pose(self, mob.id)
        } else {
            snake::render_pose(mob)
        };
        let sleeping = flags & 16 != 0;
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            for (name, visible) in [
                ("awake", !sleeping),
                ("asleep", sleeping),
                ("tail4", texture.ends_with("/rattle_snake")),
            ] {
                if let Some(part) = bone(model, name) {
                    part.visible = visible;
                }
            }
            if flags & 8 != 0
                && let Some(belly) = bone(model, "tail2")
            {
                belly.x_scale = 1.5;
                belly.y_scale = 1.5;
            }
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                if !poses.contains_key(&mob.id) {
                    poses.retain(|_, pose| now.duration_since(pose.seen) < Duration::from_secs(60));
                }
                let pose = poses
                    .entry(mob.id)
                    .or_insert_with(|| SnakePose::new(mob.id, now));
                pose.update(mob, now, flags, render.walk_speed);
                let walk = ((render.walk_speed / 0.65) * 2.5).clamp(0.1, 5.0);
                let clips = [
                    &ADULT_ATTACK,
                    &ADULT_TONGUE,
                    &ADULT_RATTLE,
                    &ADULT_SLEEP,
                    &ADULT_CLIMB,
                    &ADULT_MOVE,
                ];
                for (index, (clock, clip)) in pose.clocks.iter_mut().zip(clips).enumerate() {
                    clock.draw(
                        clip,
                        model,
                        render.age,
                        render.partial,
                        if index == 5 { walk } else { 1.0 },
                        1.0,
                    );
                }
            });
            if !sleeping && let Some(head) = bone(model, "skull") {
                head.x_rot += render.pitch.to_radians();
                head.y_rot += render.head_yaw.to_radians();
            }
            let scale = if render.baby { 0.5 } else { 1.0 };
            draw.scale([scale, -scale, -scale]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(
                draw,
                model,
                texture,
                if render.hurt_time > 0 {
                    0xFF7070
                } else {
                    0xFFFFFF
                },
            )
        })
    }
}
