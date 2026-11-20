use super::{
    Cx, Float, Goal, HurtByTarget, JUMP, LOOK, LookAtPlayer, MOVE, MeleeAttack, Mob, MobEvent,
    MobWorld, NearestTarget, Prey, RandomLookAround, Stroll, goal_clone,
};
use crate::items::ItemStack;
use crate::mobs::{
    Species, Who,
    nav::{NavKind, Navigation, naturalist_catfish_state as hunt},
    neutral_anger,
    tame::{self, naturalist_duck_pet_goals},
};
use horizon_core::{geometry::Vec3, random::RandomSource};
use std::sync::Arc;

pub const KIND: &str = "naturalist:snake";
pub const PREY: &str = "naturalist:snake_hostiles";
pub const COLORS: [&str; 3] = [
    "naturalist:snake",
    "naturalist:coral_snake",
    "naturalist:rattlesnake",
];
pub const FOOD: [&str; 4] = [
    "minecraft:chicken",
    "minecraft:egg",
    "minecraft:rabbit",
    "minecraft:rabbit_foot",
];
pub const TAME_FOOD: [&str; 3] = [
    "minecraft:spider_eye",
    "minecraft:rabbit",
    "minecraft:cooked_rabbit",
];

#[derive(Clone, Debug, Default)]
pub struct SnakeState {
    pub sleeping: bool,
    pub rattling: bool,
    pub prey: bool,
    pub food_position: Option<Vec3>,
    pub pending_spit: Option<ItemStack>,
    pub anger_uuid: Option<[i32; 4]>,
    pub anger_end: Option<i64>,
    pub anger_target: Option<Who>,
    pub anger_load_pending: bool,
}

pub fn state(nav: &Navigation) -> &SnakeState {
    nav.__horizon_mixins_naturalist_snake_state()
}
pub fn state_mut(nav: &mut Navigation) -> &mut SnakeState {
    nav.__horizon_mixins_naturalist_snake_state_mut()
}

pub fn variant_index(name: &str) -> u8 {
    // The Java predicates compare the identifier path.
    match name.rsplit(':').next().unwrap_or(name) {
        "coral_snake" => 1,
        "rattlesnake" => 2,
        _ => 0,
    }
}

pub fn venomous(mob: &Mob) -> bool {
    mob.kind.id == KIND && matches!(mob.extra.variant, 1 | 2)
}

pub fn spawn_equipment(mob: &mut Mob) {
    if mob.random.next_float() >= 0.2 {
        return;
    }
    let roll = mob.random.next_float();
    let item = if roll < 0.05 {
        "minecraft:rabbit_foot"
    } else if roll < 0.1 {
        "minecraft:slime_ball"
    } else if roll < 0.15 {
        "minecraft:feather"
    } else if roll < 0.3 {
        "minecraft:rabbit"
    } else {
        "minecraft:chicken"
    };
    mob.extra.set_main_hand_stack(Some(ItemStack::new(item, 1)));
}

pub fn register(mob: &mut Mob) {
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = FOOD.map(str::to_owned).into();
    kind.flags.climber = true;
    mob.extra.pick_up_loot = true;
    mob.ai.nav.kind = NavKind::WallClimber;
    mob.ai.goals.clear();
    mob.ai.targets.clear();
    mob.ai.goals.add(0, Float);
    naturalist_duck_pet_goals::add_snake_goals(mob);
    mob.ai
        .goals
        .add(1, SnakeMelee(MeleeAttack::new(1.75, true)));
    mob.ai.goals.add(2, SearchFood);
    mob.ai.goals.add(3, Sleep);
    mob.ai.goals.add(5, Stroll::avoiding_water(1.0));
    mob.ai.goals.add(6, LookAtPlayer::new(6.0));
    mob.ai.goals.add(7, RandomLookAround::default());
    mob.ai.targets.add(3, HurtByTarget::new());
    mob.ai
        .targets
        .add(4, NearestTarget::player(true).when(angry_at_player));
    mob.ai.targets.add(
        5,
        NearestTarget::new(
            Prey::Filter(|seen| {
                seen.effects.contains(&PREY) || seen.species == Species::Slime && seen.size == 1
            }),
            true,
        )
        .interval(5)
        .when(can_hunt),
    );
    mob.ai
        .targets
        .add(6, super::super::golem::ResetUniversalAnger::default());
}

fn angry_at_player(mob: &Mob, world: &MobWorld<'_>) -> bool {
    let status = state(&mob.ai.nav);
    status.anger_end.is_some_and(|end| end > world.game_time)
        && (status.anger_target == Some(Who::Player)
            || world.universal_anger && status.anger_target.is_none())
}

fn can_hunt(mob: &Mob, _: &MobWorld<'_>) -> bool {
    hunt::cooldown(&mob.ai.nav) <= 0
}

pub fn stop_anger(mob: &mut Mob) {
    mob.ai.last_hurt_by = None;
    mob.ai.last_hurt_by_ref = None;
    mob.ai.last_hurt_by_time = mob.age as i32;
    mob.ai.target = None;
    mob.extra.angry_at = None;
    let status = state_mut(&mut mob.ai.nav);
    status.anger_uuid = None;
    status.anger_end = Some(-1);
    status.anger_target = None;
    status.anger_load_pending = false;
    neutral_anger::stop(mob);
}

pub fn can_sleep(mob: &Mob, world: &MobWorld<'_>) -> bool {
    if state(&mob.ai.nav)
        .anger_end
        .is_some_and(|end| end > world.game_time)
        || mob.extra.anger > 0
        || !world.dimension_type.has_timeline("minecraft:day")
    {
        return false;
    }
    let time = world.day_time;
    let [x, y, z] = mob.position.components().map(|axis| axis.floor() as i32);
    time > 12_000
        && time < 28_000
        && !(time > 18_000 && time < 23_000)
        && world.facts.get(world.world.block(x, y, z)).fluid != crate::blocks::Fluid::Water
}

pub fn step(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    let saved = state(&mob.ai.nav);
    if let Some(end) = saved.anger_end {
        // The base Cow save adapter derives its fields from a transient target.
        // Java Snake uses its absolute end and target even after an NBT read.
        mob.extra.anger_end = end;
        mob.extra.angry_at = saved.anger_target;
        neutral_anger::adopt_anger_end(mob, world.game_time - 1);
    }
    tame::update_persistent_anger(mob, world);
    let end = neutral_anger::settle_anger_end(mob, world.game_time);
    let status = state_mut(&mut mob.ai.nav);
    status.anger_end = Some(end);
    status.anger_target = mob.extra.angry_at;
    if status.anger_target.is_none() {
        status.anger_uuid = None;
    }
    if let cooldown @ 1.. = hunt::cooldown(&mob.ai.nav) {
        hunt::set_cooldown(&mut mob.ai.nav, cooldown - 1);
    }
    let sleeping = state(&mob.ai.nav).sleeping;
    if sleeping || mob.extra.sitting_pose {
        mob.ai.jumping = false;
        mob.ai.xxa = 0.0;
        mob.ai.zza = 0.0;
    }
    if mob.extra.main_hand.is_none() {
        mob.extra.counter = 0;
    } else if mob.extra.counter > 6_000 {
        mob.extra.counter = 0;
        mob.extra.set_main_hand_stack(None);
        events.push(MobEvent::GameEvent {
            event: "minecraft:eat",
            at: mob.position.components(),
            source: mob.id,
            state: horizon_world::registry::BlockStateId(0),
        });
    } else if mob.extra.counter > 0 {
        mob.extra.counter += 1;
    } else if !sleeping {
        mob.extra.counter = 2;
    }
    if mob.extra.main_hand.is_some() && mob.extra.anger > 0 {
        stop_anger(mob);
    }
    let player = (mob.extra.variant == 2)
        .then_some(world.player.as_ref())
        .flatten()
        .filter(|player| !player.spectator)
        .filter(|player| {
            mob.bounds()
                .inflate(Vec3::new(4.0, 2.0, 4.0))
                .contains(player.position)
        })
        .filter(|player| {
            let range = (4.0 * player.visibility_percent(Some(KIND), world.items)).max(2.0);
            mob.distance_sqr(player.position) <= range * range
                && world.line_of_sight_for(
                    Some(mob.position.y),
                    Vec3::new(mob.position.x, mob.eye_y(), mob.position.z),
                    Vec3::new(player.eye[0], player.eye[1], player.eye[2]),
                )
        });
    let rattling = player.is_some() && !sleeping;
    state_mut(&mut mob.ai.nav).rattling = rattling;
    mob.ai.target = player
        .filter(|player| !player.creative)
        .map(|_| Who::Player);
    if rattling && !mob.silent {
        events.push(MobEvent::EntitySound {
            mob: mob.id,
            name: "naturalist:entity.snake.rattle",
            at: mob.position.components(),
            volume: 0.15,
            pitch: 1.0,
            source: 6,
        });
    }
}

pub fn render_pose(mob: &Mob) -> u8 {
    let pose = state(&mob.ai.nav);
    u8::from(pose.sleeping || mob.extra.sitting_pose)
        | (u8::from(pose.rattling) << 1)
        | (u8::from(mob.climbing) << 2)
        | (u8::from(mob.extra.main_hand.is_some()) << 3)
        | (u8::from(pose.sleeping) << 4)
}

#[derive(Clone, Debug)]
struct SearchFood;
impl Goal for SearchFood {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE
    }
    fn can_use(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        mob.extra.main_hand.is_none() && state(&mob.ai.nav).food_position.is_some()
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.tick(mob, cx);
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if mob.extra.main_hand.is_none()
            && let Some(target) = state(&mob.ai.nav).food_position
        {
            mob.nav_move_to_entity(cx.w, target.components(), 0, 1.2);
        }
    }
}

#[derive(Clone, Debug)]
struct Sleep;
impl Goal for Sleep {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | LOOK | JUMP
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.ai.xxa == 0.0
            && mob.ai.yya == 0.0
            && mob.ai.zza == 0.0
            && (can_sleep(mob, cx.w) || state(&mob.ai.nav).sleeping)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        can_sleep(mob, cx.w)
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(&mut mob.ai.nav).sleeping = true;
        mob.ai.jumping = false;
        mob.ai.jump = false;
        mob.nav_stop();
        mob.ai
            .mov
            .set_wanted(mob.position.x, mob.position.y, mob.position.z, 0.0);
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(&mut mob.ai.nav).sleeping = false;
    }
}

#[derive(Clone, Debug)]
struct SnakeMelee(MeleeAttack);
impl Goal for SnakeMelee {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.extra.main_hand.is_some() || cx.w.game_time - self.0.last_check < 20 {
            return false;
        }
        self.0.last_check = cx.w.game_time;
        let Some(target) = mob.target_info(cx.w).filter(|target| target.alive) else {
            return false;
        };
        let path = mob.create_path_to_entity(cx.w, target.position.components(), 0);
        let reaches =
            path.is_some() || mob.distance_sqr(target.position) <= 4.0 + target.bounds.size().x;
        mob.ai.pending_path = path;
        reaches
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.extra.main_hand.is_none() && self.0.can_continue(mob, cx)
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.start(mob, cx);
    }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.stop(mob, cx);
    }
    fn every_tick(&self) -> bool {
        self.0.every_tick()
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.tick(mob, cx);
    }
}
