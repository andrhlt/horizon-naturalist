use crate::mobs::{Mob, MobWorld, control::MoveKind, goals::Cx};
use horizon_core::{geometry::Vec3, random::RandomSource};

#[derive(Clone, Debug)]
pub struct Flight {
    hover: u8,
    target: Option<[i32; 3]>,
}

impl Default for Flight {
    fn default() -> Self {
        Self { hover: 30, target: None }
    }
}

pub fn register(m: &mut Mob) {
    m.ai.goals.clear();
    m.ai.targets.clear();
    m.ai.mov.kind = MoveKind::Ground;
    m.no_gravity = true;
}

pub fn step(m: &mut Mob, cx: &mut Cx) {
    let position = m.position;
    let flight = m.ai.nav.__horizon_mixins_naturalist_dragonfly_flight_mut();
    if let Some([x, y, z]) = flight.target
        && (y <= cx.w.world.bottom_y() || !cx.w.facts.is_air(cx.w.world.block(x, y, z)))
    {
        flight.target = None;
    }
    if flight.hover > 0 {
        flight.hover -= 1;
    } else if flight.target.is_none_or(|[x, y, z]| {
        position.distance_squared(Vec3::new(f64::from(x) + 0.5, f64::from(y) + 0.5, f64::from(z) + 0.5)) < 4.0
    }) {
        let target = choose_waypoint(m, cx.w);
        let flight = m.ai.nav.__horizon_mixins_naturalist_dragonfly_flight_mut();
        flight.target = Some(target);
        flight.hover = 15;
        return;
    }

    let flight = m.ai.nav.__horizon_mixins_naturalist_dragonfly_flight();
    if let Some([x, y, z]) = flight.target.filter(|_| flight.hover == 0) {
        let isgn = |v: f64| if v == 0.0 { 0.0 } else { v.signum() };
        let desired = Vec3::new(
            isgn(f64::from(x) + 0.5 - position.x) * 0.5,
            isgn(f64::from(y) + 0.1 - position.y) * 0.7,
            isgn(f64::from(z) + 0.5 - position.z) * 0.5,
        );
        m.velocity = m.velocity.add_vector(desired.subtract(m.velocity).scale(f64::from(0.1_f32)));
        m.ai.zza = 5.0;
        let yaw = horizon_core::geometry::trig::atan2_degrees(m.velocity.z, m.velocity.x) - 90.0;
        let delta = (yaw - m.yaw + 180.0).rem_euclid(360.0) - 180.0;
        m.yaw += delta;
    }
}

fn choose_waypoint(m: &mut Mob, w: &MobWorld) -> [i32; 3] {
    use crate::blocks::Fluid;
    let mut best = [0; 3];
    let mut best_score = -1;
    for _ in 0..10 {
        let mut draw = |count| f64::from(m.random.next_int_bound(count).unwrap_or(0));
        let [x, y, z] = [
            (m.position.x + draw(7) - draw(7)) as i32,
            (m.position.y + draw(6) - 2.0) as i32,
            (m.position.z + draw(7) - draw(7)) as i32,
        ];
        let score = if w.facts.is_air(w.world.block(x, y, z)) && w.fluid(x, y - 2, z) == Fluid::Water {
            10
        } else {
            0
        };
        if score > best_score {
            best_score = score;
            best = [x, y, z];
        }
    }
    best
}
