use super::{
    Mob, MobEvent, MobWorld, PlayerView, Who,
    goals::{Cx, Float, FollowParent, Goal, HurtByTarget, JUMP, LOOK, LookAtPlayer, MOVE, MeleeAttack, Panic,
        RandomLookAround, Stroll, Tempt, goal_clone, naturalist_tortoise_goals},
    tame::naturalist_duck_pet_goals,
};
use horizon_core::geometry::Vec3;
use horizon_core::random::RandomSource;
use std::sync::Arc;

pub const KIND: &str = "naturalist:ostrich";
pub const EGG: &str = "naturalist:ostrich_egg";
pub const EGG_TRAMPLE_SOUND: i32 = 0x4e415453;
pub const SUBSTRATE: &str = "naturalist:ostrich_egg_layable_on";
pub const FOOD: [&str; 6] = [
    "minecraft:wheat_seeds", "minecraft:beetroot_seeds", "minecraft:melon_seeds",
    "minecraft:pumpkin_seeds", "minecraft:pitcher_pod", "minecraft:torchflower_seeds",
];

#[derive(Clone, Debug, Default)]
pub struct State {
    pub hiding: bool,
    pub has_egg: bool,
    pub lay_ticks: u8,
    pub all_players_visibility: Option<bool>,
    pub owned_eggs: Vec<[i32; 3]>,
    pub anger_actor: Option<[i32; 4]>,
    pub anger_end: Option<i64>,
    pub attack_pose_ticks: u8,
    jump_factor: Option<f32>,
}

pub fn state(mob: &Mob) -> &State {
    mob.ai.nav.__horizon_mixins_naturalist_ostrich_state()
}

pub fn state_mut(mob: &mut Mob) -> &mut State {
    mob.ai.nav.__horizon_mixins_naturalist_ostrich_state_mut()
}

pub fn register(mob: &mut Mob) {
    let seeds: Arc<[String]> = FOOD.map(str::to_owned).into();
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = seeds.clone();
    kind.tempt = seeds.clone();
    mob.extra.color = u8::MAX;
    mob.ai.goals.add(0, Float);
    naturalist_duck_pet_goals::add_ostrich_goals(mob);
    mob.ai.goals.add(1, MeleeAttack::new(1.8, true));
    mob.ai.goals.add(2, BabyPanic(Panic::new(1.5)));
    naturalist_tortoise_goals::add_ostrich_egg_goals(mob);
    mob.ai.goals.add(4, Tempt::new(1.25, seeds, false));
    mob.ai.goals.add(5, Hide);
    mob.ai.goals.add(7, FollowParent::new(1.1));
    mob.ai.goals.add(8, Stroll::avoiding_water(1.0));
    mob.ai.goals.add(9, LookAtPlayer::new(6.0));
    mob.ai.goals.add(10, RandomLookAround::default());
    mob.ai.targets.add(1, Retaliate(HurtByTarget::new().polar_bear()
        .alert_if(|seen| !seen.baby && seen.effects.contains(&KIND))));

}

pub fn visible_player(mob: &Mob, world: &MobWorld<'_>, player: &PlayerView) -> bool {
    let range = (16.0 * player.visibility_percent(Some(KIND), world.items)).max(2.0);
    player.alive && !player.creative && !player.spectator && !player.sneaking
        && ![player.main_hand.as_deref(), player.off_hand.as_deref()].into_iter()
            .flatten().any(|item| FOOD.contains(&item))
        && mob.bounds().inflate(Vec3::new(16.0, 8.0, 16.0)).contains(player.position)
        && mob.distance_sqr(player.position) <= range * range
        && world.line_of_sight(
            Vec3::new(mob.position.x, mob.eye_y(), mob.position.z),
            Vec3::new(player.eye[0], player.eye[1], player.eye[2]),
        )
}

pub fn can_hide(mob: &Mob, world: &MobWorld<'_>) -> bool {
    !mob.extra.tame && !mob.is_baby() && mob.ai.target.is_none()
        && mob.rider.is_none() && mob.extra.rider.is_none()
        && state(mob).all_players_visibility.unwrap_or_else(|| {
            world.player.as_ref().is_some_and(|player| visible_player(mob, world, player))
        })
}

pub fn remember_egg(mob: &mut Mob, position: [i32; 3]) {
    if mob.extra.tame {
        return;
    }
    let eggs = &mut state_mut(mob).owned_eggs;
    if eggs.len() >= 16 {
        eggs.remove(0);
    }
    if !eggs.contains(&position) {
        eggs.push(position);
    }
}

pub fn is_near_egg(mob: &Mob, world: &MobWorld<'_>, position: Vec3) -> bool {
    state(mob).owned_eggs.iter().any(|&[x, y, z]| {
        let center = Vec3::new(f64::from(x) + 0.5, f64::from(y) + 0.5, f64::from(z) + 0.5);
        center.vector_to(position).length_squared() <= 100.0
            && world.world.loaded(x, z) && world.block_name(x, y, z) == EGG
    })
}

pub fn tick(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    if !mob.on_ground && mob.velocity.y < 0.0 {
        mob.velocity.x *= 1.05;
        mob.velocity.y *= 0.6;
        mob.velocity.z *= 1.05;
    }
    let hiding = can_hide(mob, world);
    let sprinting = mob.on_ground && mob.ai.nav.speed_modifier >= 1.4 && !mob.ai.nav.is_done();
    mob.extra.sprinting = sprinting;
    let swing = mob.ai.swinging && mob.ai.swing_time <= 1;
    let jump_factor = mob.block_jump_factor(world);
    let pose = state_mut(mob);
    pose.hiding = hiding;
    pose.jump_factor = Some(jump_factor);
    pose.attack_pose_ticks = if swing { 9 } else { pose.attack_pose_ticks.saturating_sub(1) };

    if mob.age % 60 == 0 {
        state_mut(mob).owned_eggs.retain(|&[x, y, z]| {
            !world.world.loaded(x, z) || world.block_name(x, y, z) == EGG
        });
    }
    if !mob.is_baby() && !mob.extra.tame {
        let mut engaged = false;
        let player = world.player.as_ref().filter(|player| player.alive && !player.creative && !player.spectator);
        if let Some(player) = player {
            let anger_end = state(mob).anger_end;
            let is_culprit = state(mob).anger_actor == Some(player.uuid);
            let in_range = mob.distance_sqr(player.position) < mob.attrs.follow_range.powi(2);
            engaged = is_culprit && in_range && mob.ai.target == Some(Who::Player);
            let guarding = is_near_egg(mob, world, player.position);
            if engaged {
                state_mut(mob).anger_end = anger_end.map(|end| end + 1);
            } else if (is_culprit && in_range && anger_end.is_some_and(|end| end > world.game_time)
                || guarding) && mob.random.next_int_bound(10) == Ok(0)
                && world.line_of_sight(Vec3::new(mob.position.x, mob.eye_y(), mob.position.z),
                    Vec3::new(player.eye[0], player.eye[1], player.eye[2]))
            {
                mob.ai.target = Some(Who::Player);
            } else if !guarding && mob.ai.target == Some(Who::Player) {
                mob.ai.target = None;
            }
        }
        if !engaged && state(mob).anger_end.is_some_and(|end| end <= world.game_time) {
            let state = state_mut(mob);
            state.anger_actor = None;
            state.anger_end = None;
        }
    }
    let laying = state(mob).lay_ticks;
    if mob.alive() && laying > 0 && laying % 5 == 0 {
        let [x, y, z] = mob.position.components().map(|axis| axis.floor() as i32);
        if world.has_tag(SUBSTRATE, world.block_name(x, y - 1, z)) {
            events.push(MobEvent::BlockBreakEffect { pos: [x, y, z], block: [x, y - 1, z] });
        }
    }
}

#[derive(Clone, Debug)]
struct Hide;

impl Goal for Hide {
    goal_clone!();
    fn flags(&self) -> u8 { MOVE | LOOK | JUMP }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool { can_hide(mob, cx.w) }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.ai.jump = false;
        mob.ai.jumping = false;
        mob.nav_stop();
        mob.ai.mov.set_wanted(mob.position.x, mob.position.y, mob.position.z, 0.0);
    }
}

#[derive(Clone, Debug)]
struct BabyPanic(Panic);

impl Goal for BabyPanic {
    goal_clone!();
    fn flags(&self) -> u8 { self.0.flags() }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.is_baby() && self.0.can_use(mob, cx)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.is_baby() && self.0.can_continue(mob, cx)
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.start(mob, cx); }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.stop(mob, cx); }
}

pub(super) fn ride(mob: &mut Mob) {
    use horizon_core::geometry::trig::Mth;
    let Some(rider) = mob.rider else { return };
    mob.yaw = rider.yaw;
    mob.head_pitch = rider.pitch * 0.5;
    mob.head_yaw = mob.yaw;
    mob.body_yaw += 0.35 * ((mob.yaw - mob.body_yaw + 180.0).rem_euclid(360.0) - 180.0);
    mob.ai.nav.stop();
    mob.set_speed(mob.movement_speed() as f32 * 0.4);
    mob.ai.xxa = rider.strafe * 0.5;
    mob.ai.zza = rider.forward;
    if mob.on_ground && rider.jump > 0.0 && mob.extra.saddled {
        let factor = f64::from(state(mob).jump_factor.unwrap_or(1.0));
        mob.velocity.y = mob.attrs.jump_strength * f64::from(rider.jump) * factor
            + f64::from(mob.jump_boost_power());
        if rider.forward > 0.0 {
            let yaw = mob.yaw.to_radians();
            mob.velocity.x += f64::from(-0.4 * yaw.msin() * rider.jump);
            mob.velocity.z += f64::from(0.4 * yaw.mcos() * rider.jump);
        }
        crate::motion::native_living_jump(mob.id, &mob.kind.id);
        if let Some(rider) = mob.rider.as_mut() {
            rider.jump = 0.0;
        }
    }
}

pub fn attachment(mob: &Mob) -> [f64; 3] {
    let scale = f64::from(mob.height()) / 2.5;
    [0.0, 1.55 * scale, -0.17 * scale]
}

#[derive(Clone, Debug)]
struct Retaliate(HurtByTarget);
impl Goal for Retaliate {
    goal_clone!();
    fn flags(&self) -> u8 { self.0.flags() }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !mob.extra.tame && self.0.can_use(mob, cx)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !mob.extra.tame && self.0.can_continue(mob, cx)
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.start(mob, cx); }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.stop(mob, cx); }
}
