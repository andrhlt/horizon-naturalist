use super::super::{
    control::{LookKind, MoveKind},
    nav::{NavKind, naturalist_catfish_state as hunt, naturalist_shark_state as shark},
    path::PathType,
    world::TargetInfo,
};
use super::{Cx, Goal, HurtByTarget, LOOK, MOVE, Mob, NearestTarget, Prey, Stroll, goal_clone};
use horizon_core::geometry::Vec3;

pub(in crate::mobs) fn register(mob: &mut Mob) {
    let kind = std::sync::Arc::make_mut(&mut mob.kind);
    kind.flags.needs_water = true;
    kind.water_slowdown = 0.9;
    kind.food = Default::default();
    kind.tempt = Default::default();
    mob.set_malus(PathType::Water, 0.0);
    mob.ai.nav.kind = NavKind::Water { breach: false };
    mob.ai.mov.kind = MoveKind::SmoothSwimming {
        max_turn_x: 85.0,
        max_turn_y: 10.0,
        in_water: 0.1,
        outside: 0.5,
        gravity: false,
    };
    mob.ai.look.kind = LookKind::SmoothSwimming(10.0);
    mob.ai.goals.clear();
    mob.ai.targets.clear();
    mob.ai.goals.add(1, SharkAttack::default());
    mob.ai.goals.add(2, Stroll::swimming(1.0, 10));
    mob.ai.targets.add(1, HurtByTarget::new());
    mob.ai.targets.add(
        2,
        NearestTarget::new(
            Prey::Filter(|prey| prey.in_water && prey.effects.contains(&shark::PREY)),
            false,
        )
        .gate(|mob, _| hunt::cooldown(&mob.ai.nav) <= 0)
        .named("NaturalistSharkHunt"),
    );
    mob.ai.targets.add(
        3,
        NearestTarget::player(false)
            .when(|mob, world| {
                let x = mob.position.x.floor() as i32;
                let y = mob.eye_y().floor() as i32;
                let z = mob.position.z.floor() as i32;
                hunt::cooldown(&mob.ai.nav) <= 0
                    && world.brightness(x, y, z) < 0.5
                    && world
                        .player
                        .as_ref()
                        .is_some_and(|player| world.fluids(player.bounds, player.eye[1]).in_water)
            })
            .named("NaturalistSharkWaterThreat"),
    );
}

#[derive(Clone, Debug, Default)]
struct SharkAttack {
    charging: bool,
    retreat_ticks: i32,
    fumble_ticks: i32,
    path_recalc: i32,
    cooldown: i32,
}

const CHARGE_DISTANCE: f64 = 25.0;
const FUMBLE_DISTANCE: f64 = 6.25;

impl SharkAttack {
    fn charge(&mut self) {
        self.charging = true;
        self.fumble_ticks = 0;
        self.path_recalc = 0;
    }

    fn retreat(&mut self, mob: &mut Mob, context: &Cx<'_, '_>, target: Vec3) {
        let Some(at) = mob.default_random_pos_away(context.w, 10, 4, target.components()) else {
            self.charge();
            return;
        };
        self.charging = false;
        self.retreat_ticks = 0;
        mob.nav_move_to(context.w, at, 1.2);
    }
}

fn facing(mob: &Mob, target: &TargetInfo) -> bool {
    let motion = shark::motion(&mob.ai.nav);
    let yaw = motion.map_or(mob.body_yaw, |part| part.yaw);
    let pitch = motion.map_or(0.0, |part| part.pitch);
    let center = Vec3::new(
        mob.position.x,
        mob.position.y + f64::from(mob.height()) * 0.5,
        mob.position.z,
    );
    let to_target = target.bounds.center().subtract(center);
    if to_target.length_squared() < 1.0e-4 {
        return true;
    }
    let (sy, cy) = f64::from(yaw.to_radians()).sin_cos();
    let (sp, cp) = f64::from(pitch.to_radians()).sin_cos();
    Vec3::new(-sy * cp, -sp, cy * cp).dot(to_target.normalize())
        >= f64::from(60_f32.to_radians().cos())
}

impl Goal for SharkAttack {
    goal_clone!();

    fn name(&self) -> &'static str {
        "NaturalistSharkAttack"
    }
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }
    fn every_tick(&self) -> bool {
        true
    }

    fn can_use(&mut self, mob: &mut Mob, context: &mut Cx) -> bool {
        mob.ai
            .target
            .and_then(|who| context.w.target_info(who))
            .is_some_and(|target| target.alive)
    }

    fn start(&mut self, mob: &mut Mob, context: &mut Cx) {
        if let Some(target) = mob.ai.target.and_then(|who| context.w.target_info(who))
            && mob.distance_sqr(target.position) < CHARGE_DISTANCE
        {
            self.retreat(mob, context, target.position);
        } else {
            self.charge();
        }
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        self.charging = false;
        mob.nav_stop();
    }

    fn tick(&mut self, mob: &mut Mob, context: &mut Cx) {
        let Some(who) = mob.ai.target else { return };
        let Some(target) = context.w.target_info(who) else {
            return;
        };
        self.cooldown = (self.cooldown - 1).max(0);
        let distance = mob.distance_sqr(target.position);
        if !self.charging {
            self.retreat_ticks += 1;
            if distance >= CHARGE_DISTANCE || mob.ai.nav.is_done() || self.retreat_ticks > 60 {
                self.charge();
            }
            return;
        }

        mob.look_at_speed(
            [target.position.x, target.eye_y, target.position.z],
            30.0,
            30.0,
        );
        self.path_recalc -= 1;
        if self.path_recalc <= 0 {
            self.path_recalc = 4;
            mob.nav_move_to(context.w, target.position.components(), 2.25);
        }
        if self.cooldown <= 0
            && mob
                .attack_box(f64::from(2.04_f32.sqrt()) - 0.6, None)
                .inflate(Vec3::new(0.9, 0.5, 0.9))
                .intersects(target.hitbox)
            && mob.sees(context.w, who)
            && facing(mob, &target)
        {
            mob.swing();
            super::super::ai::do_hurt_target(mob, who, context);
            self.cooldown = 20;
            self.retreat(mob, context, target.position);
        } else if distance < FUMBLE_DISTANCE {
            self.fumble_ticks += 1;
            if self.fumble_ticks > 15 {
                self.retreat(mob, context, target.position);
            }
        } else {
            self.fumble_ticks = 0;
        }
    }
}
