use super::naturalist_preview::bone;
use horizon_client::mobs::{Mob, MobWorld, model_layers::ModelPart};
use horizon_core::geometry::Vec3;

fn distance(world: &MobWorld<'_>, x: f64, y: f64, z: f64, block_y: i32) -> f32 {
    if block_y < world.world.bottom_y() {
        return 0.0;
    }
    let block_x = x.floor() as i32;
    let block_z = z.floor() as i32;
    let offset = Vec3::new(f64::from(block_x), f64::from(block_y), f64::from(block_z));
    let point = Vec3::new(x % 1.0, y, z % 1.0);
    let nearest = world
        .block_collision_boxes(block_x, block_y, block_z)
        .iter()
        .map(|bounds| {
            let min = bounds.min_position().subtract(offset);
            let max = bounds.max_position().subtract(offset);
            let spot = Vec3::new(
                point.x.clamp(min.x, max.x),
                point.y.clamp(min.y, max.y),
                point.z.clamp(min.z, max.z),
            );
            (spot.subtract(point).length_squared(), spot.y)
        })
        .min_by(|left, right| left.0.total_cmp(&right.0));
    nearest.map_or(1.0, |(_, height)| {
        let height = (height as f32).min(1.0);
        if y < 0.0 { height } else { 1.0 - height }
    })
}

pub(super) struct TerrainLegs {
    previous: [f32; 4],
    feet: [f32; 4],
    forward: f64,
    side: f64,
    range: f32,
}

impl TerrainLegs {
    pub(super) const fn new(forward: f64, side: f64, range: f32) -> Self {
        Self {
            previous: [0.0; 4],
            feet: [0.0; 4],
            forward,
            side,
            range,
        }
    }

    pub(super) fn heights(&self, partial: f32) -> [f32; 4] {
        std::array::from_fn(|i| self.previous[i] + (self.feet[i] - self.previous[i]) * partial)
    }

    pub(super) fn update(&mut self, mob: &Mob, world: &MobWorld<'_>) {
        let scale = mob.model_scale() * if mob.is_baby() { 0.5 } else { 1.0 };
        let side = f64::from(mob.body_yaw.to_radians());
        let forward = side + f64::from(std::f32::consts::FRAC_PI_2);
        let (side_z, side_x) = side.sin_cos();
        let (forward_z, forward_x) = forward.sin_cos();
        self.previous = self.feet;
        let y = mob.position.y - 1.0;
        let positions = [
            (-self.forward, self.side),
            (-self.forward, -self.side),
            (self.forward, self.side),
            (self.forward, -self.side),
        ];
        for ((forward, side), height) in positions.into_iter().zip(&mut self.feet) {
            let x = mob.position.x + f64::from(scale) * (side_x * side + forward_x * forward);
            let z = mob.position.z + f64::from(scale) * (side_z * side + forward_z * forward);
            let mut block_y = (y + 0.001).floor() as i32;
            let mut piece = distance(world, x, y, z, block_y);
            let mut target = piece;
            while piece == 1.0 {
                block_y -= 1;
                piece = distance(world, x, y, z, block_y);
                target += piece;
            }
            if mob.on_ground && *height <= target || *height > 0.0 {
                *height += 0.2 * (target - *height);
            }
            *height = height.clamp(-self.range * scale, self.range * scale);
        }
    }

    pub(super) fn articulate(
        &self,
        model: &mut ModelPart,
        head: Option<&str>,
        legs: [&str; 4],
        partial: f32,
    ) {
        let [bl, br, fl, fr] = self.heights(partial);
        let rear = (bl + br) * 0.5;
        let front = (fl + fr) * 0.5;
        let left = (bl + fl) * 0.5;
        let right = (br + fr) * 0.5;
        let pitch = (front - rear).atan2((self.forward * 2.0) as f32);
        let roll = (left - right).atan2((self.side * 2.0) as f32) * 0.5;
        if let Some(body) = bone(model, "body") {
            body.y += (rear + front) * 8.0;
            body.x_rot += pitch;
            body.z_rot += roll;
        }
        if let Some(head) = head.and_then(|name| bone(model, name)) {
            head.x_rot -= pitch;
            head.z_rot -= roll;
        }
        for (name, height) in legs.into_iter().zip([bl, br, fl, fr]) {
            if let Some(foot) = bone(model, name) {
                foot.y += height * 16.0;
            }
        }
    }
}
