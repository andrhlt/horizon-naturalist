use super::{
    Mob,
    control::{Operation, rotlerp},
    naturalist_snail::{snail, snail_mut},
    naturalist_snail_surface::tangent,
};
use horizon_core::geometry::{Vec3, trig};

const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

#[derive(Clone, Debug, Default)]
pub struct SurfaceControl {
    last_pos: Vec3,
    last_wanted: Vec3,
    stuck_ticks: i32,
    climb_ticks: i32,
    stuck_cycles: i32,
    progress_anchor: Vec3,
    progress_ticks: i32,
}

impl SurfaceControl {
    fn reset_progress(&mut self, at: Vec3) {
        self.stuck_ticks = 0;
        self.climb_ticks = 0;
        self.stuck_cycles = 0;
        self.progress_anchor = at;
        self.progress_ticks = 0;
    }

    fn give_up(&mut self, mob: &mut Mob) {
        mob.ai.mov.operation = Operation::Wait;
        self.reset_progress(mob.position);
        mob.nav_stop();
        snail_mut(mob).surface.halt();
    }

    pub fn tick(&mut self, mob: &mut Mob) {
        if !mob.ai.mov.has_wanted() || !snail(mob).surface.attached {
            self.stuck_ticks = 0;
            snail_mut(mob).surface.halt();
            return;
        }

        let [x, y, z] = mob.ai.mov.wanted;
        let wanted = Vec3::new(x, y, z);
        if wanted != self.last_wanted {
            self.last_wanted = wanted;
            self.reset_progress(mob.position);
        }
        let delta = wanted.subtract(mob.position);
        let distance_sqr = delta.length_squared();
        if distance_sqr < 0.01 {
            mob.ai.mov.operation = Operation::Wait;
            self.stuck_ticks = 0;
            snail_mut(mob).surface.halt();
            return;
        }

        self.track_progress(mob);
        let normal = snail(mob).surface.normal;
        let on_wall = normal.y.abs() < 0.5;
        let mut steer = if on_wall && delta.dot(normal) < 0.0 {
            UP
        } else {
            let mut result = tangent(delta, normal);
            if on_wall && result.y > 0.0 {
                result.y = 0.0;
            }
            if result.length_squared() < distance_sqr * 0.04 {
                result = if on_wall {
                    UP.reverse()
                } else {
                    tangent(UP, normal)
                };
                if result.length_squared() < 0.01 {
                    result = Vec3::new(delta.x, 0.0, delta.z);
                }
            }
            result
        };

        if self.climb_ticks > 0 {
            self.climb_ticks -= 1;
            let climb = tangent(UP, normal);
            if climb.length_squared() > 0.01 {
                steer = climb;
            } else {
                let slide = normal.cross(steer);
                if slide.length_squared() > 1.0e-6 {
                    steer = slide;
                }
            }
        }
        if steer.length_squared() < 1.0e-7 {
            snail_mut(mob).surface.halt();
            return;
        }

        let steer = steer.normalize();
        let speed = mob.ai.mov.speed_modifier * mob.movement_speed();
        mob.set_speed(speed as f32);
        let surface = &mut snail_mut(mob).surface;
        surface.desired = steer;
        surface.speed = speed;
        surface.goal = delta;

        if steer.x * steer.x + steer.z * steer.z > 1.0e-4 {
            let yaw = trig::atan2_degrees(steer.z, steer.x) - 90.0;
            mob.yaw = rotlerp(mob.yaw, yaw, 20.0);
            mob.body_yaw = mob.yaw;
        }
    }

    fn track_progress(&mut self, mob: &mut Mob) {
        let pos = mob.position;
        self.progress_ticks += 1;
        if self.progress_ticks >= 20 {
            if pos.subtract(self.progress_anchor).length_squared() < 0.01 {
                snail_mut(mob).surface.ground_cooldown = 40;
                self.give_up(mob);
                return;
            }
            self.progress_anchor = pos;
            self.progress_ticks = 0;
        }

        if pos.subtract(self.last_pos).length_squared() < 1.0e-4 {
            self.stuck_ticks += 1;
            if self.stuck_ticks > 8 && self.climb_ticks <= 0 {
                self.stuck_ticks = 0;
                if snail(mob).surface.on_side() {
                    self.stuck_cycles += 1;
                    if self.stuck_cycles >= 2 {
                        snail_mut(mob).surface.ground_cooldown = 40;
                        self.give_up(mob);
                    } else {
                        self.climb_ticks = 24;
                    }
                } else {
                    snail_mut(mob).surface.ground_cooldown = 40;
                    self.give_up(mob);
                }
            }
        } else {
            self.stuck_ticks = 0;
        }
        self.last_pos = pos;
    }
}

pub fn crawl_to(mob: &mut Mob, target: [i32; 3], speed: f64) -> bool {
    if !snail(mob).surface.attached {
        return false;
    }
    snail_mut(mob).crawl_target = Some(target);
    mob.ai.nav.speed_modifier = speed;
    true
}

pub fn tick_navigation(mob: &mut Mob) -> bool {
    if snail(mob).surface.on_side()
        && let Some(target) = mob.ai.nav.path.as_ref().map(|path| path.target)
    {
        mob.nav_stop();
        snail_mut(mob).crawl_target = Some(target);
    }
    let Some([x, y, z]) = snail(mob).crawl_target else {
        return false;
    };
    if snail(mob).surface.attached {
        let target = Vec3::new(f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5);
        if mob.distance_sqr(target) < f64::from(mob.width().max(1.0)).powi(2) {
            let mut control = std::mem::take(&mut snail_mut(mob).control);
            control.give_up(mob);
            snail_mut(mob).control = control;
        } else {
            let speed = mob.ai.nav.speed_modifier.max(0.7);
            mob.ai.mov.set_wanted(target.x, target.y, target.z, speed);
        }
    }
    true
}
