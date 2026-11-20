use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture,
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
    naturalist_terrain_legs::TerrainLegs,
};
use horizon_client::mobs::{Mob, MobWorld, model_layers::ModelPart, nav::naturalist_elephants};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static ELEPHANT_ADULT: RefCell<ModelPart> = RefCell::new(elephant_models::adult());
    static ELEPHANT_BABY: RefCell<ModelPart> = RefCell::new(elephant_models::baby());
    static MAMMOTH_ADULT: RefCell<ModelPart> = RefCell::new(mammoth_models::adult());
    static MAMMOTH_BABY: RefCell<ModelPart> = RefCell::new(mammoth_models::baby());
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

fn pachyderm_clips(baby: bool) -> [&'static Clip; 4] {
    use elephant_animations::*;
    if baby {
        [&BABY_IDLE, &BABY_WALK, &BABY_RUN, &ADULT_SWING]
    } else {
        [&ADULT_IDLE, &ADULT_WALK, &ADULT_RUN, &ADULT_SWING]
    }
}

struct Pose {
    terrain: TerrainLegs,
    clocks: [AnimationClock; 4],
    age: i32,
    banner: BannerMotion,
    last_seen: Instant,
}

impl Pose {
    fn new(now: Instant) -> Self {
        Self {
            terrain: TerrainLegs::new(1.0, 0.5, 0.9),
            clocks: Default::default(),
            age: -1,
            banner: BannerMotion::default(),
            last_seen: now,
        }
    }

    fn tick(&mut self, mob: &Mob, world: &MobWorld<'_>, now: Instant) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;
        let moving = mob.velocity.x.powi(2) + mob.velocity.z.powi(2) > 1.0e-6;
        let running = moving && mob.extra.sprinting;
        let swinging = (mob.age as u64) < naturalist_elephants::state(mob).swing_until;
        let states = [!moving, moving && !running, running, swinging];
        for (index, (clock, active)) in self.clocks.iter_mut().zip(states).enumerate() {
            clock.advance_with_rate(active, age, if index == 3 { 1.0 } else { 0.5 });
        }
        self.terrain.update(mob, world);
        let [bl, br, fl, fr] = self.terrain.heights(1.0);
        let pitch = ((fl + fr - bl - br) * 0.5).atan2(2.0);
        self.banner.tick(mob, pitch);
    }
}

#[derive(Default)]
struct BannerMotion {
    yaw: Option<f32>,
    previous: [f32; 2],
    angles: [f32; 2],
}

impl BannerMotion {
    fn tick(&mut self, mob: &Mob, pitch: f32) {
        let (sin, cos) = f64::from(mob.body_yaw.to_radians()).sin_cos();
        let side = mob.velocity.x * cos + mob.velocity.z * sin;
        let forward = mob.velocity.z * cos - mob.velocity.x * sin;
        let yaw = self.yaw.replace(mob.body_yaw).unwrap_or(mob.body_yaw);
        let turn = (mob.body_yaw - yaw + 180.0).rem_euclid(360.0) - 180.0;
        self.previous = self.angles;
        let lift = (forward as f32 * 120.0 + pitch.to_degrees().max(0.0)).clamp(-35.0, 35.0);
        let swing = (turn * 1.5 - side as f32 * 160.0).clamp(-20.0, 20.0);
        for (angle, target) in self.angles.iter_mut().zip([lift, swing]) {
            *angle += (target - *angle) * 0.15;
        }
    }

    fn lerp(&self, partial: f32) -> [f32; 2] {
        std::array::from_fn(|i| self.previous[i] + (self.angles[i] - self.previous[i]) * partial)
    }
}

struct BannerParts {
    bar: ModelPart,
    flag: ModelPart,
}

thread_local! {
    static BANNER_PARTS: RefCell<[BannerParts; 2]> = RefCell::new([
        BannerParts::new(false), BannerParts::new(true),
    ]);
}

impl BannerParts {
    fn new(mirror: bool) -> Self {
        use horizon_client::mobs::model_layers::{
            CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, PartPose,
        };
        let mut mesh = MeshDefinition::new();
        let root = mesh.root();
        mesh.add_or_replace_child(root, "bar",
            CubeListBuilder::create().tex_offs(0, 42).set_mirror(mirror)
                .add_box_with([-10.0, -32.0, -1.0], [20.0, 2.0, 2.0], CubeDeformation::new(0.0)),
            PartPose::offset(0.0, 32.0, 0.0));
        mesh.add_or_replace_child(root, "flag",
            CubeListBuilder::create().tex_offs(0, 0).set_mirror(mirror)
                .add_box_with([-10.0, 0.0, -2.0], [20.0, 40.0, 1.0], CubeDeformation::new(0.0)),
            PartPose::offset(0.0, 0.0, 0.0));
        let root = LayerDefinition::create(mesh, 64, 64).bake_root();
        Self {
            bar: root.get_child("bar").expect("banner bar").clone(),
            flag: root.get_child("flag").expect("banner flag").clone(),
        }
    }
}

fn transform_part(draw: &mut BlockEntityDraw, part: &ModelPart) {
    draw.translate([part.x / 16.0, part.y / 16.0, part.z / 16.0]);
    draw.rotate_about([0.0, 0.0, 1.0], part.z_rot.to_degrees(), [0.0; 3]);
    draw.rotate_about([0.0, 1.0, 0.0], part.y_rot.to_degrees(), [0.0; 3]);
    draw.rotate_about([1.0, 0.0, 0.0], part.x_rot.to_degrees(), [0.0; 3]);
    draw.scale([part.x_scale, part.y_scale, part.z_scale]);
}

impl Game<'_> {
    fn draw_elephant_banners(
        &self,
        draw: &mut BlockEntityDraw,
        model: &ModelPart,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        banner_angles: [f32; 2],
    ) {
        use horizon_client::banner::{banner_color, dye_rgb, layers};
        if render.baby {
            return;
        }
        let Some(Some(stack)) = mob.extra.items.get(naturalist_elephants::BANNER_SLOT) else { return };
        let Some(color) = banner_color(stack.item.as_ref()) else { return };
        let Some(root) = model.get_child("root") else { return };
        let Some(body) = root.get_child("body") else { return };
        let (x, y, z, scale) = if mob.kind.id == naturalist_elephants::MAMMOTH {
            (16.5, -17.5, -4.0, 0.8)
        } else {
            (15.0, -15.5, -6.0, 0.7)
        };
        let phase = ((mob.age as i64 + i64::from(mob.id) * 13).rem_euclid(100) as f32
            + render.partial) / 100.0;
        let sway = (-0.0125 + 0.01 * (std::f32::consts::TAU * phase).cos()) * std::f32::consts::PI;
        let [lift, swing] = banner_angles.map(f32::to_radians);
        let angle = swing + body.z_rot;
        let layers = super::banners::layer_textures(&self.stations().banners, layers(stack));
        draw.push();
        transform_part(draw, root);
        transform_part(draw, body);
        BANNER_PARTS.with_borrow_mut(|parts| {
            for (part, side) in parts.iter_mut().zip([1.0, -1.0]) {
                draw.push();
                draw.translate([side * x / 16.0, y / 16.0, z / 16.0]);
                draw.rotate_about([0.0, 1.0, 0.0], -90.0 * side, [0.0; 3]);
                draw.scale([scale; 3]);
                part.flag.x_rot = (sway - side * angle).min(0.0);
                part.flag.z_rot = side * lift;
                self.native_draw_model_part(draw, &part.bar, "entity/banner/banner_base", 0xffffff);
                self.native_draw_model_part(draw, &part.flag, "entity/banner/banner_base", 0xffffff);
                self.native_draw_model_part(draw, &part.flag, "entity/banner/base", dye_rgb(color));
                for (sprite, tint) in &layers {
                    draw.translate([0.0, 0.0, -0.0001]);
                    self.native_draw_model_part(draw, &part.flag, sprite, *tint);
                }
                draw.pop();
            }
        });
        draw.pop();
    }

    pub(super) fn draw_naturalist_elephant(
        &self,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let mammoth = mob.kind.id == naturalist_elephants::MAMMOTH;
        let (model, fallback) = match (mammoth, render.baby) {
            (false, false) => (&ELEPHANT_ADULT, "naturalist:entity/elephant/elephant"),
            (false, true) => (&ELEPHANT_BABY, "naturalist:entity/elephant/elephant_baby"),
            (true, false) => (&MAMMOTH_ADULT, "naturalist:entity/mammoth/mammoth"),
            (true, true) => (&MAMMOTH_BABY, "naturalist:entity/mammoth/mammoth_baby"),
        };
        let clips = pachyderm_clips(render.baby);
        let texture = appearance_texture(self, mob.id, render.baby);
        let world = self.mob_world_base(&[]);
        let ridden = self.riding == Some(mob.id)
            || self.ridden_by_guest(mob.id)
            || mob.extra.rider.is_some();
        let fast = mob.ai.target.is_some() || ridden;
        let base = match (mammoth, render.baby, fast) {
            (_, true, _) | (false, false, true) => 4.4,
            (false, false, false) => 2.9,
            (true, false, false) => 2.5,
            (true, false, true) => 3.0,
        };
        model.with_borrow_mut(|model| {
            model.reset_all_poses();
            for (names, showing) in [
                (
                    &["saddle", "saddleFront"][..],
                    mob.extra.saddled && !render.baby,
                ),
                (
                    &["chests", "leftChest", "rightChest"][..],
                    mob.extra.chested && !render.baby,
                ),
                (&["asleep"][..], false),
                (&["awake"][..], true),
            ] {
                for name in names {
                    if let Some(part) = bone(model, name) {
                        part.visible = showing;
                    }
                }
            }
            let banner_angles = POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                if poses.len() > 2048 {
                    poses.retain(|_, pose| {
                        now.duration_since(pose.last_seen) < Duration::from_secs(30)
                    });
                }
                let pose = poses.entry(mob.id).or_insert_with(|| Pose::new(now));
                pose.tick(mob, &world, now);
                let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
                let rate = base * (render.walk_speed / gait).clamp(0.4, 2.0);
                if !render.baby {
                    pose.clocks[3].draw(clips[3], model, render.age, render.partial, 1.0, 1.0);
                }
                let idle_weight = 1.0 - (render.walk_speed * 2.5).clamp(0.0, 1.0);
                for (index, (speed, weight)) in [(1.0, idle_weight), (rate, 1.0), (rate, 1.0)]
                    .into_iter()
                    .enumerate()
                {
                    pose.clocks[index].draw(
                        clips[index],
                        model,
                        render.age,
                        render.partial,
                        speed,
                        weight,
                    );
                }
                pose.terrain.articulate(
                    model,
                    Some("neck"),
                    ["leftLeg", "rightLeg", "leftArm", "rightArm"],
                    render.partial,
                );
                if let Some(neck) = bone(model, "neck") {
                    neck.x_rot += render.pitch.to_radians();
                    neck.y_rot += render.head_yaw.to_radians();
                }
                pose.banner.lerp(render.partial)
            });
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 {
                0xFF7070
            } else {
                0xFFFFFF
            };
            let drawn = self.native_draw_model_part(draw, model, texture.as_deref().unwrap_or(fallback), tint);
            if drawn {
                self.draw_elephant_banners(draw, model, mob, render, banner_angles);
            }
            drawn
        })
    }
}
