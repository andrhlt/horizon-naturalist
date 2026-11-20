use super::{
    Game,
    native_block_entity_draw::BlockEntityDraw,
    native_mob_draw::NativeMobRender,
    naturalist_deer_variants::{appearance_texture, appearance_variant},
    naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target, bone},
};
use horizon_client::mobs::{
    Mob, MobWorld, living::wrap, model_layers::ModelPart, naturalist_snail::snail,
    render::DYE_NAMES,
};
use horizon_core::geometry::{Aabb, Axis, Vec3, trig};
use std::{
    cell::RefCell,
    collections::HashMap,
    time::{Duration, Instant},
};

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(snail_models::adult());
    static SNAIL_POSES: RefCell<HashMap<u32, SnailPose>> = RefCell::new(HashMap::new());
}

const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const FRONT: Vec3 = Vec3::new(0.0, 0.0, 1.0);

fn project(vector: Vec3, normal: Vec3) -> Vec3 {
    vector.subtract(normal.scale(vector.dot(normal)))
}

fn unit_lerp(from: Vec3, to: Vec3, partial: f32) -> Vec3 {
    let mixed = from.lerp(to, f64::from(partial));
    if mixed.length_squared() > 1.0e-8 {
        mixed.normalize()
    } else {
        to
    }
}

fn flatten(vector: Vec3, normal: Vec3) -> Vec3 {
    let tangent = project(vector, normal);
    if tangent.length_squared() > 1.0e-6 {
        tangent.normalize()
    } else {
        vector
    }
}

#[derive(Clone, Copy)]
struct OrientationFrame {
    normal: Vec3,
    forward: Vec3,
    tail: Vec3,
    anchor: Vec3,
}

impl Default for OrientationFrame {
    fn default() -> Self {
        Self {
            normal: UP,
            forward: FRONT,
            tail: FRONT,
            anchor: Vec3::ZERO,
        }
    }
}

#[derive(Default)]
struct SnailOrientation {
    previous: OrientationFrame,
    frame: OrientationFrame,
    velocity: Vec3,
    boxes: Vec<Aabb>,
}

struct RenderOrientation {
    normal: Vec3,
    forward: Vec3,
    anchor: Vec3,
    tail_lag: f32,
}

impl SnailOrientation {
    fn tick(&mut self, mob: &Mob, world: &MobWorld, attached_normal: Vec3) {
        self.previous = self.frame;
        self.boxes.clear();
        let bounds = mob.bounds();
        let range = bounds.inflate(Vec3::new(0.5, 0.5, 0.5));
        let block_pos = |axis| range.min(axis).floor() as i32..=range.max(axis).floor() as i32;
        for y in block_pos(Axis::Y) {
            for z in block_pos(Axis::Z) {
                for x in block_pos(Axis::X) {
                    self.boxes.extend(
                        world
                            .block_collision_boxes(x, y, z)
                            .into_iter()
                            .filter(|b| b.intersects(range)),
                    );
                }
            }
        }

        let center = bounds.center();
        let mut normal_target = attached_normal;
        let mut anchor_target = Vec3::ZERO;
        if !self.boxes.is_empty() {
            let gradient = |dir: Vec3| {
                self.distance(center.add_vector(dir)) - self.distance(center.subtract(dir))
            };
            let sample = Vec3::new(
                gradient(Vec3::new(0.2, 0.0, 0.0)),
                gradient(Vec3::new(0.0, 0.2, 0.0)),
                gradient(Vec3::new(0.0, 0.0, 0.2)),
            );
            if sample.length_squared() > 1.0e-6 {
                normal_target = sample
                    .normalize()
                    .add_vector(attached_normal.scale(0.6))
                    .normalize();
            }
        }
        let normal = unit_lerp(self.frame.normal, normal_target, 0.3);
        if !self.boxes.is_empty() {
            let distance = self.distance(center);
            if distance < 0.75 {
                anchor_target = center
                    .subtract(normal.scale(distance))
                    .subtract(mob.position);
            }
        }
        let anchor = self.frame.anchor.lerp(anchor_target, f64::from(0.4_f32));
        self.velocity = self.velocity.lerp(mob.position.subtract(mob.previous), 0.3);

        let mut forward = self.frame.forward;
        let moment = project(self.velocity, normal);
        if moment.length_squared() > 2.5e-5 {
            forward = unit_lerp(forward, moment.normalize(), 0.35);
        } else {
            let direction = project(Vec3::direction_from_rotation(0.0, mob.body_yaw), normal);
            if direction.length_squared() > 1.0e-4 {
                forward = unit_lerp(forward, direction.normalize(), 0.08);
            }
        }
        let forward = flatten(forward, normal);
        let tail = flatten(unit_lerp(self.frame.tail, forward, 0.5), normal);
        self.frame = OrientationFrame {
            normal,
            forward,
            tail,
            anchor,
        };
    }

    fn distance(&self, point: Vec3) -> f64 {
        self.boxes
            .iter()
            .map(|bounds| bounds.distance_squared_to_point(point))
            .fold(f64::MAX, f64::min)
            .sqrt()
    }

    fn sample(&self, partial: f32) -> RenderOrientation {
        let normal = unit_lerp(self.previous.normal, self.frame.normal, partial);
        let heading = unit_lerp(self.previous.forward, self.frame.forward, partial);
        let tail = unit_lerp(self.previous.tail, self.frame.tail, partial);
        let mut flat = project(heading, normal);
        if flat.length_squared() < 1.0e-6 {
            flat = project(FRONT, normal);
            if flat.length_squared() < 1.0e-6 {
                flat = project(UP, normal);
            }
        }
        RenderOrientation {
            normal,
            forward: flat.normalize(),
            anchor: self
                .previous
                .anchor
                .lerp(self.frame.anchor, f64::from(partial)),
            tail_lag: trig::atan2(tail.cross(heading).dot(normal), tail.dot(heading)) as f32,
        }
    }
}

struct SnailPose {
    orientation: SnailOrientation,
    clocks: [AnimationClock; 4],
    age: i32,
    hidden: bool,
    show_end: u8,
    look: [[f32; 2]; 2],
    last_seen: Instant,
}

impl SnailPose {
    fn new(now: Instant) -> Self {
        Self {
            orientation: SnailOrientation::default(),
            clocks: Default::default(),
            age: -1,
            hidden: false,
            show_end: 0,
            look: [[0.0; 2]; 2],
            last_seen: now,
        }
    }

    fn tick(&mut self, mob: &Mob, world: &MobWorld, now: Instant) {
        let age = mob.age as i32;
        if age < self.age {
            *self = Self::new(now);
        }
        self.last_seen = now;
        if age == self.age {
            return;
        }
        self.age = age;

        let state = snail(mob);
        let surface = &state.surface;
        let synced = if surface.attached { surface.normal } else { UP };
        self.orientation.tick(mob, world, synced);

        let hidden = state.hiding || !mob.alive();
        if self.hidden && !hidden {
            self.show_end = 16;
        }
        self.hidden = hidden;
        let ending = !hidden && self.show_end > 0;
        self.show_end = self.show_end.saturating_sub(1);
        let locomotion = !hidden && !ending;
        let moving = mob.velocity.x * mob.velocity.x + mob.velocity.z * mob.velocity.z > 1.0e-6;
        let crawl = locomotion && (moving || surface.on_side());
        for (clock, (active, rate)) in self.clocks.iter_mut().zip([
            (hidden, 1.0),
            (ending, 1.0),
            (locomotion && !crawl, 0.5),
            (crawl, 0.5),
        ]) {
            clock.advance_with_rate(active, age, rate);
        }

        let target = if mob.limb_speed > 1.0e-5 {
            0.0
        } else {
            wrap(mob.head_yaw - mob.body_yaw) * -0.5
        };
        let [body, tail] = self.look[1];
        let body_next = body + 0.15 * wrap(target - body);
        let tail_next = tail + 0.15 * wrap(body_next - tail);
        self.look = [[body, tail], [body_next, tail_next]];
    }

    fn apply(
        &mut self,
        model: &mut ModelPart,
        mob: &Mob,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) {
        let orientation = self.orientation.sample(render.partial);
        let render_pos = mob.previous.lerp(mob.position, f64::from(render.partial));
        let view = -draw.pose().w_axis.truncate();
        let to_camera = Vec3::new(f64::from(view.x), f64::from(view.y), f64::from(view.z))
            .add_vector(render_pos.subtract(mob.position));
        let back = orientation.forward.reverse();
        let right = orientation.normal.cross(back);
        let glam_vec = |v: Vec3| glam::Vec3::new(v.x as f32, v.y as f32, v.z as f32);
        let basis = glam::Mat3::from_cols(
            glam_vec(right),
            glam_vec(orientation.normal),
            glam_vec(back),
        );
        let (axis, angle) = glam::Quat::from_mat3(&basis).to_axis_angle();
        draw.rotate_about([0.0, 1.0, 0.0], render.body_yaw, [0.0; 3]);
        draw.translate(glam_vec(orientation.anchor).to_array());
        draw.rotate_about(axis.to_array(), angle.to_degrees(), [0.0; 3]);

        if render.baby
            && let Some(eyes) = bone(model, "eyes")
        {
            eyes.x_scale = 1.5;
            eyes.y_scale = 1.5;
            eyes.z_scale = 1.5;
        }
        use snail_animations::*;
        let gait = ((8.64 * mob.movement_speed()).min(1.0) * 0.65).max(0.05) as f32;
        let crawl_speed = if snail(mob).surface.on_side() {
            1.0
        } else {
            5.0 * (render.walk_speed / gait).clamp(0.4, 2.0)
        };
        let idle = 1.0 - (render.walk_speed * 2.5).min(1.0);
        for (clock, (clip, speed, weight)) in self.clocks.iter_mut().zip([
            (&ADULT_HIDE_START, 1.0, 1.0),
            (&ADULT_HIDE_END, 1.0, 1.0),
            (&ADULT_IDLE, 1.0, idle),
            (&ADULT_CRAWL, crawl_speed, 1.0),
        ]) {
            clock.draw(clip, model, render.age, render.partial, speed, weight);
        }
        if !self.hidden {
            self.apply_look(model, to_camera, &orientation, render.partial);
        }
    }

    fn apply_look(
        &self,
        model: &mut ModelPart,
        camera: Vec3,
        orientation: &RenderOrientation,
        partial: f32,
    ) {
        let look: [f32; 2] = std::array::from_fn(|i| {
            self.look[0][i] + partial * wrap(self.look[1][i] - self.look[0][i])
        });
        let body_yaw = look[0].to_radians();
        for name in ["body", "shell"] {
            if let Some(part) = bone(model, name) {
                part.y_rot -= body_yaw;
            }
        }
        let plane = project(camera, orientation.normal);
        let plane_len = plane.length_squared().sqrt();
        let view = if plane_len < 1.0e-4 {
            orientation.forward
        } else {
            plane.normalize()
        };
        let yaw = (orientation
            .forward
            .cross(view)
            .dot(orientation.normal)
            .atan2(orientation.forward.dot(view)) as f32)
            - body_yaw;
        let yaw = wrap(yaw.to_degrees()).clamp(-90.0, 90.0).to_radians();
        let pitch = (camera.dot(orientation.normal).atan2(plane_len) as f32)
            .clamp(-22.5_f32.to_radians(), 22.5_f32.to_radians());
        for name in ["leftEye", "rightEye"] {
            if let Some(part) = bone(model, name) {
                part.x_rot -= pitch;
                part.y_rot -= yaw;
            }
        }
        if let Some(tail) = bone(model, "tail") {
            tail.x_rot -= (-1.4 * orientation.tail_lag).clamp(-0.6, 0.6);
            tail.y_rot -= (look[1] - look[0]).to_radians() * 3.0;
        }
    }
}

fn pose(poses: &mut HashMap<u32, SnailPose>, id: u32, now: Instant) -> &mut SnailPose {
    if poses.len() > 2048 {
        poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
    }
    poses.entry(id).or_insert_with(|| SnailPose::new(now))
}

impl Game<'_> {
    pub(super) fn tick_naturalist_snail_animations(&self) {
        if self.remote.is_none() {
            return;
        }
        let now = Instant::now();
        let world = self.mob_world_base(&[]);
        SNAIL_POSES.with_borrow_mut(|poses| {
            for mob in self
                .mobs
                .iter()
                .filter(|mob| !mob.removed && mob.kind.id == "naturalist:snail")
            {
                pose(poses, mob.id, now).tick(mob, &world, now);
            }
        });
    }

    pub(super) fn draw_naturalist_snail(
        &self,
        render: &NativeMobRender<'_>,
        draw: &mut BlockEntityDraw,
    ) -> bool {
        let mob = self.mobs.iter().find(|mob| mob.id == render.id);
        let color = mob.map_or(12, |mob| snail(mob).color);
        let color_texture = format!("naturalist:entity/snail/{}", DYE_NAMES[usize::from(color)]);
        let variant = appearance_variant(self, render.id);
        let texture = variant
            .filter(|id| id != "naturalist:snail")
            .and_then(|_| appearance_texture(self, render.id, render.baby))
            .unwrap_or(color_texture);
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            if let Some(mob) = mob {
                SNAIL_POSES.with_borrow_mut(|poses| {
                    let now = Instant::now();
                    let pose = pose(poses, mob.id, now);
                    if self.remote.is_none() {
                        pose.tick(mob, &self.mob_world_base(&[]), now);
                    }
                    pose.apply(model, mob, render, draw);
                });
            }
            if render.baby {
                draw.scale([0.5; 3]);
            }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let tint = if render.hurt_time > 0 {
                0xFF7070
            } else {
                0xFFFFFF
            };
            self.native_draw_model_part(draw, model, &texture, tint)
        })
    }
}

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_snail_orientation(&self) -> usize {
        use horizon_client::mobs::naturalist_snail::snail_mut;
        let Some(kind) = self.res.mob_types().get("naturalist:snail").cloned() else {
            return 0;
        };
        let mut mob = Mob::new(0, kind, Vec3::new(7.2, 93.0, 8.5), 0.0);
        let world = self.mob_world_base(&[]);
        let mut surface = std::mem::take(&mut snail_mut(&mut mob).surface);
        let mut orientation = SnailOrientation::default();
        let mut events = Vec::new();
        let mut wall = false;
        let mut frames = 0;
        for tick in 1..=57 {
            mob.previous = mob.position;
            let direction = if wall { UP } else { Vec3::new(1.0, 0.0, 0.0) };
            surface.desired = direction;
            surface.speed = 0.22;
            surface.goal = direction;
            if !surface.travel(&mut mob, &world, &mut events) {
                break;
            }
            surface.tick(&mut mob);
            let normal = if surface.attached { surface.normal } else { UP };
            orientation.tick(&mob, &world, normal);
            wall |= normal.x == -1.0;
            if [1, 5, 7, 8, 10, 20, 46, 47, 48, 50, 57].contains(&tick) {
                for partial in [0.0, 0.5, 0.9] {
                    let pose = orientation.sample(partial);
                    let axes = |v: Vec3| [v.x, v.y, v.z];
                    println!(
                        "naturalist_snail_orientation {}",
                        serde_json::json!({
                            "tick": tick, "partial": partial, "normal": axes(pose.normal),
                            "forward": axes(pose.forward), "anchor": axes(pose.anchor),
                            "tail_lag": pose.tail_lag,
                        }),
                    );
                    frames += 1;
                }
            }
        }
        frames
    }
}
