pub const GEAR_META: u8 = 246;
pub const BANNER_META: u8 = 247;
use crate::mobs::{Mob, MobEvent, MobWorld, Who, equine::Rider, neutral_anger, tame};
use horizon_core::geometry::Vec3;

pub const ELEPHANT: &str = "naturalist:elephant";
pub const MAMMOTH: &str = "naturalist:mammoth";
pub const CHEST_SLOTS: usize = 25;
pub const SADDLE_SLOT: usize = 25;
pub const BANNER_SLOT: usize = 26;
pub const INVENTORY_SLOTS: usize = 27;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub cakes: u32,
    pub swing_until: u64,
    pub mob_rider: Option<Rider>,
    pub anger_uuid: Option<[i32; 4]>,
    pub anger_target: Option<Who>,
    pub hurt_player_uuid: Option<[i32; 4]>,
    pub delegated_player_uuid: Option<[i32; 4]>,
    pub player_alerts: Vec<(u32, [i32; 4])>,
    pub anger_end: Option<i64>,
    pub restore_anger: bool,
    pub legacy_anger_ticks: Option<i64>,
}

pub fn includes(id: &str) -> bool {
    matches!(id, ELEPHANT | MAMMOTH)
}

pub fn state(mob: &Mob) -> &State {
    mob.ai.nav.__horizon_mixins_naturalist_elephants()
}

pub fn state_mut(mob: &mut Mob) -> &mut State {
    mob.ai.nav.__horizon_mixins_naturalist_elephants_mut()
}

pub fn seat(mob: &Mob) -> [f64; 3] {
    let ratio = if mob.kind.id == MAMMOTH { 1.05 } else { 0.92 };
    [0.0, f64::from(mob.height()) * ratio, -0.1]
}

pub fn angry_player(mob: &Mob, world: &MobWorld<'_>) -> bool {
    let status = state(mob);
    status.anger_end.is_some_and(|end| end > world.game_time)
        && (status.anger_target == Some(Who::Player)
            && world.player.as_ref().is_some_and(|player| Some(player.uuid) == status.anger_uuid)
            || (world.universal_anger && status.anger_uuid.is_none() && status.anger_target.is_none()))
}

pub fn anger_player_to_view(mob: &Mob) -> Option<[i32; 4]> {
    let status = state(mob);
    status.delegated_player_uuid.or_else(|| {
        if status.anger_target == Some(Who::Player) {
            status.anger_uuid
        } else {
            status.hurt_player_uuid
        }
    })
}

pub fn prepare_anger(mob: &mut Mob, game_time: i64) {
    let status = state_mut(mob);
    if let Some(ticks) = status.legacy_anger_ticks.take() {
        status.anger_end = Some((game_time - 1).saturating_add(ticks));
    }
    let (end, target) = (status.anger_end, status.anger_target);
    // Core NBT readback projects other mobs' fields from their live target.
    // Keep the absolute expiry and unresolved reference across those reads.
    mob.extra.anger_end = end.unwrap_or(-1);
    mob.extra.angry_at = target;
    neutral_anger::adopt_anger_end(mob, game_time - 1);
}

pub fn step_anger(mob: &mut Mob, world: &MobWorld<'_>) {
    if !mob.alive() {
        return;
    }
    let previous = state(mob).anger_target;
    if let Some(Who::Mob(id)) = previous
        && world.seen(id).is_some_and(|actor| !actor.alive)
    {
        mob.ai.forget_hurt_by(mob.age as i32);
        mob.ai.target = None;
        mob.extra.angry_at = None;
        neutral_anger::stop(mob);
    } else {
        if let Some(Who::Mob(id)) = mob.ai.target
            && world.seen(id).is_none()
        {
            mob.ai.target = None;
        }
        tame::update_persistent_anger(mob, world);
    }
    let end = neutral_anger::settle_anger_end(mob, world.game_time);
    let target = mob.extra.angry_at;
    let player_uuid = world.player.as_ref().map(|player| player.uuid);
    let anger = mob.extra.anger;
    let status = state_mut(mob);
    status.delegated_player_uuid = None;
    if anger <= 0 && status.anger_uuid.is_some() {
        status.anger_end = Some(-1);
        status.anger_uuid = None;
    } else {
        status.anger_end = Some(end);
        if target == Some(Who::Player) {
            status.anger_uuid = player_uuid;
        } else if target != previous {
            status.anger_uuid = None;
        }
    }
    status.anger_target = target;
}

pub fn travel_with_mob_rider(
    mob: &mut Mob,
    world: &MobWorld<'_>,
    input: Vec3,
    events: &mut Vec<MobEvent>,
) -> bool {
    // Temporarily take the inputs so the reentrant call uses stock travel.
    let Some(rider) = state_mut(mob).mob_rider.take() else {
        return false;
    };
    mob.yaw = rider.yaw;
    mob.head_yaw = rider.yaw;
    mob.head_pitch = rider.pitch * 0.5;
    let turn = (rider.yaw - mob.body_yaw + 180.0).rem_euclid(360.0) - 180.0;
    mob.body_yaw += turn * 0.35;
    mob.set_speed(mob.movement_speed() as f32 * 0.3);
    mob.travel(
        world,
        Vec3::new(f64::from(rider.strafe * 0.5), input.y, f64::from(rider.forward)),
        events,
    );
    state_mut(mob).mob_rider = Some(rider);
    true
}

pub fn add_spawn_bonus(mob: &mut Mob, random: &mut impl horizon_core::random::RandomSource) {
    use crate::mobs::finalize::{self, Attr, Op};
    if !includes(&mob.kind.id) {
        return;
    }
    let amount = 0.11485000000000001 * (random.next_double() - random.next_double());
    finalize::add_modifier(
        mob,
        Attr::FollowRange,
        "naturalist:random_spawn_bonus",
        amount,
        Op::AddMultipliedBase,
    );
}
