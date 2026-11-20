use super::{
    Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{animal_rest_pose, appearance_texture, lizard_appearance},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

const ZEBRA: &str = "naturalist:zebra";
const RHINO: &str = "naturalist:rhino";
const KOMODO: &str = "naturalist:komodo_dragon";
const LIZARD: &str = "naturalist:lizard";
const LIZARD_TAIL: &str = "naturalist:lizard_tail";
const DESERT_SCORPION: &str = "naturalist:desert_scorpion";
const JUNGLE_SCORPION: &str = "naturalist:jungle_scorpion";

thread_local! {
    static RHINO_MODEL: RefCell<ModelPart> = RefCell::new(rhino_models::adult());
    static KOMODO_MODEL: RefCell<ModelPart> = RefCell::new(komodo_dragon_models::adult());
    static GECKO: RefCell<ModelPart> = RefCell::new(lizard_models::adult());
    static DETACHED_TAIL: RefCell<ModelPart> = RefCell::new(lizard_tail_models::adult());
    static DESERT: RefCell<ModelPart> = RefCell::new(desert_scorpion_models::adult());
    static JUNGLE: RefCell<ModelPart> = RefCell::new(jungle_scorpion_models::adult());
    static BOAR: RefCell<ModelPart> = RefCell::new(boar_models::adult());
    static PIGLET: RefCell<ModelPart> = RefCell::new(boar_models::baby());
    static ZEBRA_MODELS: RefCell<[ModelPart; 2]> = RefCell::new([zebra_models::adult(), zebra_models::baby()]);
    static TURKEY: RefCell<ModelPart> = RefCell::new(turkey_models::adult());
    static POSES: RefCell<HashMap<u32, LandPose>> = RefCell::new(HashMap::new());
}

struct LandPose {
    clocks: [AnimationClock; 6],
    age: i32,
    action_end: i32,
    swinging: bool,
    last_seen: Instant,
}

impl LandPose {
    fn new(now: Instant) -> Self {
        Self { clocks: Default::default(), age: -1, action_end: -1, swinging: false, last_seen: now }
    }

    fn tick(&mut self, mob: &Mob, now: Instant, rest_pose: u8) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;
        let kind = mob.kind.id.as_str();
        let scorpion = matches!(kind, DESERT_SCORPION | JUNGLE_SCORPION);
        let swinging = mob.ai.swinging;
        if swinging && !self.swinging {
            self.action_end = age + match kind {
                "naturalist:turkey" => 25,
                DESERT_SCORPION | JUNGLE_SCORPION => 15,
                RHINO => 10,
                _ => 7,
            };
        }
        self.swinging = swinging;
        let horizontal_speed_sq = mob.velocity.x.powi(2) + mob.velocity.z.powi(2);
        let moving = horizontal_speed_sq > 1.0e-6;
        let running = moving && if scorpion { mob.ai.aggressive } else {
            mob.extra.sprinting || kind == ZEBRA && horizontal_speed_sq > 0.01
        };
        let resting = rest_pose != 0;
        let active = match kind {
            LIZARD => [resting, moving && !resting, false, false, false, false],
            LIZARD_TAIL => [true, false, false, false, false, false],
            KOMODO => [!resting && !moving, !resting && moving, false,
                age < self.action_end, resting, mob.is_baby()],
            RHINO => {
                let stunned = rest_pose == 2;
                let stomping = !stunned && !moving && rest_pose == 1;
                [
                    !stunned && !moving && !stomping,
                    !stunned && moving && !running,
                    !stunned && running,
                    age < self.action_end, stomping, stunned,
                ]
            }
            _ => [!moving, moving && !running, running, age < self.action_end, false, false],
        };
        for (clock, active) in self.clocks.iter_mut().zip(active) {
            clock.advance_with_rate(active, age, 0.5);
        }
    }
}

fn pose(poses: &mut HashMap<u32, LandPose>, id: u32, now: Instant) -> &mut LandPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| LandPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_land_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        POSES.with_borrow_mut(|poses| {
            for mob in self.mobs.iter().filter(|mob| !mob.removed
                && matches!(mob.kind.id.as_str(), "naturalist:boar" | "naturalist:turkey" | ZEBRA | DESERT_SCORPION | JUNGLE_SCORPION | LIZARD | LIZARD_TAIL | KOMODO | RHINO))
            {
                let resting = match mob.kind.id.as_str() {
                    LIZARD => u8::from(lizard_appearance(self, mob.id).1),
                    KOMODO | RHINO => animal_rest_pose(self, mob.id),
                    _ => 0,
                };
                pose(poses, mob.id, now).tick(mob, now, resting);
            }
        });
    }

    pub(super) fn draw_naturalist_land_animal(
        &self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw,
    ) -> bool {
        if mob.kind.id == ZEBRA {
            return self.draw_naturalist_zebra(mob, render, draw);
        }
        if matches!(mob.kind.id.as_str(), LIZARD | LIZARD_TAIL) {
            return self.draw_naturalist_lizard(mob, render, draw);
        }
        if mob.kind.id == RHINO {
            return self.draw_naturalist_rhino(mob, render, draw);
        }
        if mob.kind.id == KOMODO {
            return self.draw_naturalist_komodo(mob, render, draw);
        }
        use boar_animations as boar;
        use desert_scorpion_animations as desert;
        use jungle_scorpion_animations as jungle;
        use turkey_animations as turkey;
        let (model, default_texture, clips, action, walk_rate, run_rate, head) =
            match (mob.kind.id.as_str(), render.baby) {
                (DESERT_SCORPION, _) => (&DESERT, "naturalist:entity/scorpion/desert_scorpion",
                    [&desert::ADULT_IDLE, &desert::ADULT_WALK, &desert::ADULT_RUN],
                    Some(&desert::ADULT_ATTACK), 2.0, 1.6, None),
                (JUNGLE_SCORPION, _) => (&JUNGLE, "naturalist:entity/scorpion/black_jungle_scorpion",
                    [&jungle::ADULT_IDLE, &jungle::ADULT_WALK, &jungle::ADULT_RUN],
                    Some(&jungle::ADULT_ATTACK), 2.0, 1.6, None),
                ("naturalist:turkey", _) => (&TURKEY, "naturalist:entity/turkey",
                    [&turkey::ADULT_IDLE, &turkey::ADULT_WALK, &turkey::ADULT_RUN],
                    Some(&turkey::ADULT_PICKING_ON_GROUND), 1.0, 1.0, Some("skull")),
                ("naturalist:boar", true) => (&PIGLET, "naturalist:entity/boar_baby",
                    [&boar::BABY_IDLE, &boar::BABY_WALK, &boar::BABY_RUN], None, 3.5, 2.0, Some("neck")),
                _ => (&BOAR, "naturalist:entity/boar", [&boar::ADULT_IDLE, &boar::ADULT_WALK, &boar::ADULT_RUN],
                    Some(&boar::ADULT_ATTACK), 2.5, 2.0, Some("neck")),
            };
        let texture = appearance_texture(self, mob.id, render.baby);
        let texture = texture.as_deref().unwrap_or(default_texture);
        model.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now, 0);
                }
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let movement_rate = (render.walk_speed / gait).clamp(0.4, 2.0);
                let idle_weight = 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0);
                if let Some(action) = action {
                    pose.clocks[3].draw(action, model, render.age, render.partial, 1.0, 1.0);
                }
                for (clock, (clip, (speed, weight))) in pose.clocks.iter_mut()
                    .zip(clips.into_iter().zip([
                        (1.0, idle_weight), (movement_rate * walk_rate, 1.0),
                        (movement_rate * run_rate, 1.0),
                    ]))
                {
                    clock.draw(clip, model, render.age, render.partial, speed, weight);
                }
                if let Some(head) = head.and_then(|name| bone(model, name)) {
                    head.x_rot += render.pitch.to_radians();
                    head.y_rot += render.head_yaw.to_radians();
                }
            });
            if render.baby && mob.kind.id == "naturalist:turkey" {
                draw.scale([0.5; 3]);
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            self.native_draw_model_part(draw, model, texture, tint)
        })
    }

    fn draw_naturalist_zebra(
        &self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw,
    ) -> bool {
        use zebra_animations as clips;
        let (default_texture, clips) = if render.baby {
            ("naturalist:entity/zebra_baby", [&clips::BABY_IDLE, &clips::BABY_WALK, &clips::BABY_RUN])
        } else {
            ("naturalist:entity/zebra", [&clips::ADULT_IDLE, &clips::ADULT_WALK, &clips::ADULT_RUN])
        };
        let texture = appearance_texture(self, mob.id, render.baby);
        ZEBRA_MODELS.with_borrow_mut(|models| {
            let model = &mut models[usize::from(render.baby)];
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now, 0);
                }
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let speed = (render.walk_speed / gait).clamp(0.4, 2.0);
                let idle = 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0);
                let walk_rate = if render.baby { 3.0 } else { 2.0 };
                for (clock, (clip, (rate, weight))) in pose.clocks.iter_mut().zip(
                    clips.into_iter().zip([(1.0, idle), (speed * walk_rate, 1.0), (speed * 1.5, 1.0)]),
                ) {
                    clock.draw(clip, model, render.age, render.partial, rate, weight);
                }
                if let Some(neck) = bone(model, "neck") {
                    neck.x_rot += render.pitch.to_radians();
                    neck.y_rot += render.head_yaw.to_radians();
                }
            });
            let saddled = mob.extra.saddled;
            let bridled = mob.extra.tame && saddled;
            let ridden = mob.rider.is_some() || mob.extra.rider.is_some();
            for (name, visible) in [
                ("saddle", saddled), ("bridle", bridled), ("reinsR", bridled && ridden),
                ("reinsL", bridled && ridden), ("chest", mob.extra.chested),
            ] {
                if let Some(part) = bone(model, name) {
                    part.visible = visible;
                }
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            self.native_draw_model_part(draw, model, texture.as_deref().unwrap_or(default_texture), tint)
        })
    }

    fn draw_naturalist_lizard(
        &self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw,
    ) -> bool {
        use horizon_client::mobs::{naturalist_lizard, render::DYE_NAMES};
        let tail = mob.kind.id == LIZARD_TAIL;
        let model = if tail { &DETACHED_TAIL } else { &GECKO };
        let resolved = appearance_texture(self, mob.id, false);
        let texture = resolved.as_deref().unwrap_or(if tail {
            "naturalist:entity/lizard/green_tail"
        } else {
            "naturalist:entity/lizard/green"
        });
        let (dye, sitting, missing_tail) = if self.remote.is_some() {
            lizard_appearance(self, mob.id)
        } else {
            (Some(mob.extra.color), mob.extra.sitting_pose, naturalist_lizard::missing_tail(&mob.ai.nav))
        };
        model.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now, u8::from(sitting));
                }
                if tail {
                    pose.clocks[0].draw(&lizard_tail_animations::ADULT_FLOP, model,
                        render.age, render.partial, 1.0, 1.0);
                } else {
                    let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                    let speed = (render.walk_speed / gait).clamp(0.4, 2.0);
                    pose.clocks[0].draw(&lizard_animations::ADULT_SIT, model,
                        render.age, render.partial, 1.0, 1.0);
                    pose.clocks[1].draw(&lizard_animations::ADULT_WALK, model,
                        render.age, render.partial, 4.5 * speed, 1.0);
                    if let Some(part) = bone(model, "tail") {
                        part.visible = !missing_tail;
                    }
                    if let Some(neck) = bone(model, "neck") {
                        neck.x_rot += render.pitch.to_radians();
                        neck.y_rot += render.head_yaw.to_radians();
                    }
                }
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            let drawn = self.native_draw_model_part(draw, model, texture, tint);
            if !tail && let Some(dye) = dye.and_then(|id| DYE_NAMES.get(usize::from(id))) {
                self.native_draw_model_part(draw, model, &format!("naturalist:entity/lizard/dye/{dye}"), tint);
            }
            drawn
        })
    }

    fn draw_naturalist_komodo(
        &self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw,
    ) -> bool {
        use komodo_dragon_animations::*;
        let resolved = appearance_texture(self, mob.id, render.baby);
        let texture = resolved.as_deref().unwrap_or("naturalist:entity/komodo_dragon");
        let basking = if self.remote.is_some() {
            animal_rest_pose(self, mob.id) != 0
        } else {
            horizon_client::mobs::naturalist_komodo::basking(&mob.ai.nav)
        };
        KOMODO_MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now, u8::from(basking));
                }
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let speed = (render.walk_speed / gait).clamp(0.4, 2.0);
                let walk_rate = if mob.extra.sprinting { 2.5 } else { 1.5 };
                let idle_weight = 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0);
                let clips = [
                    (3, &ADULT_BITE, 1.0, 1.0),
                    (5, &ADULT_BABY_TRANSFORM, 1.0, 1.0),
                    (4, &ADULT_BASK, 1.0, 1.0),
                    (0, &ADULT_IDLE, 1.0, idle_weight),
                    (1, &ADULT_WALK, walk_rate * speed, 1.0),
                ];
                for (clock, clip, rate, weight) in clips {
                    pose.clocks[clock].draw(clip, model, render.age, render.partial, rate, weight);
                }
                if let Some(skull) = bone(model, "skull") {
                    skull.x_rot += render.pitch.to_radians();
                    skull.y_rot += render.head_yaw.to_radians();
                }
            });
            if render.baby {
                draw.scale([0.45; 3]);
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            self.native_draw_model_part(draw, model, texture, tint)
        })
    }

    fn draw_naturalist_rhino(
        &self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw,
    ) -> bool {
        use horizon_client::mobs::naturalist_rhino;
        use rhino_animations::*;
        let resolved = appearance_texture(self, mob.id, render.baby);
        let texture = resolved.as_deref().unwrap_or("naturalist:entity/rhino");
        let rest_pose = if self.remote.is_some() {
            animal_rest_pose(self, mob.id)
        } else if naturalist_rhino::stun_ticks(&mob.ai.nav) > 0 {
            2
        } else {
            u8::from(naturalist_rhino::preparing(&mob.ai.nav))
        };
        RHINO_MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            let baby = render.baby;
            for (name, visible) in [("big_horn", !baby), ("small_horn", !baby), ("baby_horn", baby)] {
                if let Some(part) = bone(model, name) {
                    part.visible = visible;
                }
            }
            if baby {
                for (name, scale) in [("neck", 0.4), ("left_ear", 0.1), ("right_ear", 0.1)] {
                    if let Some(part) = bone(model, name) {
                        part.offset_scale([scale; 3]);
                    }
                }
            }
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let pose = pose(poses, mob.id, now);
                if self.remote.is_none() {
                    pose.tick(mob, now, rest_pose);
                }
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let speed = (render.walk_speed / gait).clamp(0.4, 2.0);
                let idle_weight = 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0);
                for (index, clip, rate, weight) in [
                    (3, &ADULT_ATTACK, 1.3, 1.0),
                    (5, &ADULT_STUNNED, 1.0, 1.0),
                    (4, &ADULT_FOOT, 1.0, 1.0),
                    (0, &ADULT_IDLE, 1.0, idle_weight),
                    (1, &ADULT_WALK, 2.2 * speed, 1.0),
                    (2, &ADULT_RUN, 3.6 * speed, 1.0),
                ] {
                    pose.clocks[index].draw(clip, model, render.age, render.partial, rate, weight);
                }
                if !mob.extra.sprinting && let Some(neck) = bone(model, "neck") {
                    neck.y_rot += render.head_yaw.to_radians();
                }
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF };
            self.native_draw_model_part(draw, model, texture, tint)
        })
    }
}
