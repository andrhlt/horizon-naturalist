use super::{
    Breed, Cx, Float, FollowParent, Goal, HurtByTarget, JUMP, LOOK, LookAtPlayer, MOVE,
    MeleeAttack, Mob, MobWorld, NearestTarget, Panic, Prey, RandomLookAround, Stroll, Tempt,
    goal_clone, naturalist_lion_goals as lion, reduced,
};
use crate::mobs::{
    nav::{Navigation, naturalist_catfish_state as hunt},
    tame::naturalist_duck_pet_goals,
};
use std::sync::Arc;

pub const KIND: &str = "naturalist:tiger";

pub fn is_big_cat(id: &str) -> bool {
    matches!(id, KIND | lion::KIND)
}
pub const PREY: &str = "naturalist:tiger_hostiles";
pub const COLORS: [&str; 4] = [
    "naturalist:black_panther",
    "naturalist:leopard",
    "naturalist:tiger",
    "naturalist:white_tiger",
];
pub const FOOD: [&str; 27] = [
    "minecraft:beef",
    "minecraft:chicken",
    "minecraft:cooked_beef",
    "minecraft:cooked_chicken",
    "minecraft:cooked_mutton",
    "minecraft:cooked_porkchop",
    "minecraft:cooked_rabbit",
    "minecraft:mutton",
    "minecraft:porkchop",
    "minecraft:rabbit",
    "naturalist:bushmeat",
    "naturalist:clam_meat",
    "naturalist:cooked_bushmeat",
    "naturalist:cooked_clam_meat",
    "naturalist:cooked_crab_meat",
    "naturalist:cooked_drumstick",
    "naturalist:cooked_duck",
    "naturalist:cooked_lizard_tail",
    "naturalist:cooked_mammoth_meat",
    "naturalist:cooked_venison",
    "naturalist:crab_meat",
    "naturalist:drumstick",
    "naturalist:duck",
    "naturalist:lizard_tail",
    "naturalist:mammoth_meat",
    "naturalist:morsel",
    "naturalist:venison",
];

const SLEEPING: u8 = 1;
const STALKING: u8 = 2;
const TIGER_PREY: u8 = 4;
const LION_PREY: u8 = 8;

fn flag(nav: &Navigation, flag: u8) -> bool {
    *nav.__horizon_mixins_naturalist_tiger_pose() & flag != 0
}
fn set_flag(nav: &mut Navigation, flag: u8, enabled: bool) {
    let value = nav.__horizon_mixins_naturalist_tiger_pose_mut();
    *value = (*value & !flag) | if enabled { flag } else { 0 };
}
pub fn sleeping(mob: &Mob) -> bool {
    flag(&mob.ai.nav, SLEEPING)
}
pub fn stalking(mob: &Mob) -> bool {
    flag(&mob.ai.nav, STALKING)
}
pub fn is_tiger_prey(mob: &Mob) -> bool {
    flag(&mob.ai.nav, TIGER_PREY)
}
pub fn is_lion_prey(mob: &Mob) -> bool {
    flag(&mob.ai.nav, LION_PREY)
}

pub fn mark_prey(mob: &mut Mob, tiger: bool, lion: bool) {
    set_flag(&mut mob.ai.nav, TIGER_PREY, tiger);
    set_flag(&mut mob.ai.nav, LION_PREY, lion);
}

pub fn render_pose(mob: &Mob, uuid: Option<[i32; 4]>) -> u8 {
    let lion = mob.kind.id == lion::KIND;
    let cub_sitting = lion && mob.is_baby() && mob.extra.sitting_pose;
    let naturally_asleep = sleeping(mob);
    let sleep = if naturally_asleep || mob.extra.sitting_pose && !cub_sitting {
        if !(lion && mob.is_baby())
            && uuid
                .unwrap_or_default()
                .into_iter()
                .fold(0, |lhs, rhs| lhs ^ rhs)
                & 1
                == 0
        {
            2
        } else {
            1
        }
    } else {
        0
    };
    sleep
        | (u8::from(stalking(mob)) << 2)
        | (u8::from(mob.extra.sprinting) << 3)
        | (u8::from(cub_sitting) << 4)
        | (u8::from(lion && mob.extra.flag) << 5)
        | (u8::from(lion && mob.ai.goals.is_running("MeleeAttack")) << 6)
        | (u8::from(naturally_asleep) << 7)
}

pub fn register(mob: &mut Mob) {
    let lion = mob.kind.id == lion::KIND;
    let food: Arc<[String]> = FOOD.map(str::to_owned).into();
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    kind.ambient_interval = 900;
    mob.ai.goals.clear();
    mob.ai.targets.clear();
    mob.ai.goals.add(0, Float);
    let alert = HurtByTarget::new().polar_bear();
    mob.ai.targets.add(
        1,
        if lion {
            alert.alert_if(|seen| seen.effects.contains(&lion::KIND) && !seen.baby)
        } else {
            alert.alert_if(|seen| seen.effects.contains(&KIND) && !seen.baby)
        },
    );
    naturalist_duck_pet_goals::add_big_cat_goals(mob);
    if !lion {
        mob.ai.goals.add(1, Stalk::default());
    }
    mob.ai.goals.add(1, MeleeAttack::new(1.6, true));
    mob.ai.goals.add(2, CubPanic(Panic::new(1.25)));
    mob.ai.goals.add(3, Sleep);
    mob.ai.goals.add(4, Tempt::new(1.0, food, false));
    mob.ai.goals.add(4, Breed::new(1.0));
    mob.ai.goals.add(4, FollowCub(FollowParent::new(1.1)));
    mob.ai.goals.add(6, Stroll::avoiding_water(0.6));
    if lion {
        mob.ai.goals.add(6, lion::FollowLeader::default());
    }
    mob.ai.goals.add(7, LookAtPlayer::new(6.0));
    mob.ai.goals.add(8, RandomLookAround::default());
    if lion {
        mob.ai.targets.add(
            4,
            NearestTarget::new(
                Prey::Filter(|seen| !seen.baby && seen.effects.contains(&lion::PREY)),
                true,
            )
            .reach()
            .when(can_select_prey),
        );
    } else {
        mob.ai.targets.add(4, HurtByTarget::new());
        mob.ai.targets.add(
            5,
            NearestTarget::new(
                Prey::Filter(|seen| !seen.baby && seen.effects.contains(&PREY)),
                true,
            )
            .reach()
            .when(can_select_prey),
        );
    }
}

pub fn can_sleep(mob: &Mob, world: &MobWorld<'_>) -> bool {
    !mob.extra.tame
        && mob.ai.target.is_none()
        && world.dimension_type.has_timeline("minecraft:day")
        && world.day_time > 6_000
        && world.day_time < 13_000
        && {
            let [x, y, z] = mob.position.components().map(|axis| axis.floor() as i32);
            world.facts.get(world.world.block(x, y, z)).fluid != crate::blocks::Fluid::Water
        }
}

pub fn can_select_prey(mob: &Mob, world: &MobWorld<'_>) -> bool {
    !mob.is_baby()
        && !sleeping(mob)
        && !stalking(mob)
        && !world.daylight
        && world.dimension_type.has_timeline("minecraft:day")
        && hunt::cooldown(&mob.ai.nav) <= 0
}

pub fn tick(mob: &mut Mob) {
    if mob.kind.id == lion::KIND {
        lion::update_cub_speed(mob);
    }
    if let cooldown @ 1.. = hunt::cooldown(&mob.ai.nav) {
        hunt::set_cooldown(&mut mob.ai.nav, cooldown - 1);
    }
    mob.extra.sprinting =
        mob.ai.mov.has_wanted() && mob.ai.mov.speed_modifier >= 1.05 && mob.on_ground;
    if sleeping(mob) {
        mob.ai.jumping = false;
        mob.ai.jump = false;
        mob.ai.xxa = 0.0;
        mob.ai.zza = 0.0;
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
            && (can_sleep(mob, cx.w) || sleeping(mob))
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        can_sleep(mob, cx.w)
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        set_flag(&mut mob.ai.nav, SLEEPING, true);
        mob.ai.jumping = false;
        mob.ai.jump = false;
        mob.nav_stop();
        mob.ai
            .mov
            .set_wanted(mob.position.x, mob.position.y, mob.position.z, 0.0);
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        set_flag(&mut mob.ai.nav, SLEEPING, false);
    }
}

#[derive(Clone, Debug, Default)]
struct Stalk {
    recalc: i32,
}
impl Goal for Stalk {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !mob.is_baby()
            && !mob.extra.tame
            && !sleeping(mob)
            && mob
                .target_info(cx.w)
                .is_some_and(|t| t.alive && mob.distance_sqr(t.position) > 144.0)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !sleeping(mob)
            && mob
                .target_info(cx.w)
                .is_some_and(|t| t.alive && mob.distance_sqr(t.position) > 36.0)
    }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        set_flag(&mut mob.ai.nav, STALKING, true);
        self.recalc = 0;
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        set_flag(&mut mob.ai.nav, STALKING, false);
        mob.nav_stop();
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(target) = mob.target_info(cx.w) else {
            return;
        };
        let (pos, eye_y) = (target.position.components(), target.eye_y);
        mob.look_at_speed([pos[0], eye_y, pos[2]], 10.0, mob.max_head_x_rot());
        self.recalc -= 1;
        if self.recalc <= 0 {
            self.recalc = reduced(10);
            mob.nav_move_to(cx.w, pos, 0.7);
        }
    }
}

#[derive(Clone, Debug)]
struct CubPanic(Panic);
impl Goal for CubPanic {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        (mob.fire_ticks > 0 || mob.is_baby() && mob.ai.last_hurt_by.is_some())
            && self.0.can_use(mob, cx)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(mob, cx)
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.start(mob, cx);
    }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.stop(mob, cx);
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.tick(mob, cx);
    }
}

#[derive(Clone, Debug)]
struct FollowCub(FollowParent);
impl Goal for FollowCub {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !sleeping(mob) && !mob.extra.tame && self.0.can_use(mob, cx)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        (mob.kind.id != lion::KIND || !sleeping(mob) && !mob.extra.tame)
            && self.0.can_continue(mob, cx)
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.start(mob, cx);
    }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.stop(mob, cx);
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.tick(mob, cx);
    }
}
