use super::{
    Mob, MobEvent, MobWorld, Seen, Who,
    goals::{Cx, Float, Goal, JUMP, LOOK, MOVE, MoveToBlock, Tempt, within_home},
    nav::Navigation,
};
use horizon_core::{geometry::Vec3, random::RandomSource};

pub const BEAR: &str = "naturalist:bear";
pub const BLACK_BEAR: &str = "naturalist:black_bear";
pub const PREY: &str = "naturalist:bear_hostiles";

pub fn is_bear(kind: &str) -> bool {
    matches!(kind, BEAR | BLACK_BEAR)
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub sleeping: bool,
    pub sniffing: bool,
    pub eating_sit: bool,
    pub sheared: bool,
    pub eat_counter: u16,
    pub hunting: i32,
    pub wake: i32,
    pub prey: bool,
    pub loaded_anger_target: Option<[i32; 4]>,
    pub food_position: Option<Vec3>,
    pub pending_spit: Option<crate::items::ItemStack>,
    pub harvest: Option<[i32; 3]>,
}

pub fn state(nav: &Navigation) -> &State {
    nav.__horizon_mixins_naturalist_bears_state()
}

pub fn state_mut(nav: &mut Navigation) -> &mut State {
    nav.__horizon_mixins_naturalist_bears_state_mut()
}

pub fn can_shear(mob: &Mob) -> bool {
    mob.alive() && !mob.is_baby() && !state(&mob.ai.nav).sheared
}

pub(super) fn no_food(mob: &Mob) -> bool {
    mob.extra.main_hand.is_none()
}

pub(super) fn on_land(mob: &Mob, world: &MobWorld) -> bool {
    let [x, y, z] = mob.position.components().map(|v| v.floor() as i32);
    world.fluid(x, y, z) != crate::blocks::Fluid::Water
}

pub(super) fn protects_cubs(mob: &Mob, world: &MobWorld) -> bool {
    if mob.is_baby() || state(&mob.ai.nav).sleeping {
        return false;
    }
    let bounds = mob.bounds().inflate(Vec3::new(8.0, 4.0, 8.0));
    world.others_near(bounds).any(|seen| {
        seen.baby && seen.alive && bounds.intersects(seen.bounds)
            && (seen.effects.contains(&BEAR) || seen.effects.contains(&BLACK_BEAR))
    })
}

pub(super) fn natural_prey(seen: &Seen) -> bool {
    seen.effects.contains(&PREY)
}

fn sniff_sound(mob: &Mob) -> &'static str {
    if mob.kind.id == BLACK_BEAR { "naturalist:entity.black_bear.sniff" }
    else { "naturalist:entity.bear.sniff" }
}

#[derive(Clone, Debug)]
pub(super) struct BearFloat;

impl Goal for BearFloat {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "Float" }
    fn flags(&self) -> u8 { JUMP }
    fn every_tick(&self) -> bool { true }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        let [x, y, z] = mob.position.components().map(|v| v.floor() as i32);
        let beside_water = [y - 1, y + 1].into_iter().any(|y| cx.w.fluid(x, y, z) == crate::blocks::Fluid::Water);
        (mob.is_baby() || beside_water) && Float.can_use(mob, cx)
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) { Float.tick(mob, cx); }
}

#[derive(Clone, Debug)]
pub(super) struct Sleep;

impl Sleep {
    fn allowed(mob: &Mob, world: &MobWorld) -> bool {
        let daytime = world.day_time;
        state(&mob.ai.nav).wake <= 0 && (daytime < 12_000 || daytime > 18_000)
            && (6001..23_000).contains(&daytime) && mob.extra.anger <= 0 && on_land(mob, world)
    }
}

impl Goal for Sleep {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { MOVE | LOOK | JUMP }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.ai.xxa == 0.0 && mob.ai.yya == 0.0 && mob.ai.zza == 0.0
            && (Self::allowed(mob, cx.w) || state(&mob.ai.nav).sleeping)
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool { Self::allowed(mob, cx.w) }

    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        let status = state_mut(&mut mob.ai.nav);
        status.sleeping = true;
        status.sniffing = false;
        mob.ai.jump = false;
        mob.nav_stop();
        let p = mob.position;
        mob.ai.mov.set_wanted(p.x, p.y, p.z, 0.0);
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(&mut mob.ai.nav).sleeping = false;
    }
}

#[derive(Clone, Debug)]
pub(super) struct BearTempt(Tempt);

impl BearTempt {
    pub(super) fn new(food: std::sync::Arc<[String]>) -> Self {
        Self(Tempt::new(1.0, food, false))
    }
}

impl Goal for BearTempt {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { self.0.flags() }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool { no_food(mob) && self.0.can_use(mob, cx) }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool { no_food(mob) && self.0.can_continue(mob, cx) }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.start(mob, cx);
        state_mut(&mut mob.ai.nav).sniffing = true;
    }

    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.stop(mob, cx);
        state_mut(&mut mob.ai.nav).sniffing = false;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.tick(mob, cx);
        if mob.random.next_float() < 0.05 && !mob.silent {
            cx.events.push(MobEvent::SoundId(sniff_sound(mob).to_owned(), mob.position.components(), 1.0, 1.0));
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct FindFood;

impl Goal for FindFood {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { MOVE }
    fn can_use(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        !mob.is_baby() && !state(&mob.ai.nav).sleeping && no_food(mob)
            && state(&mob.ai.nav).food_position.is_some()
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.tick(mob, cx); }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) { mob.nav_stop(); }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if let Some(position) = state(&mob.ai.nav).food_position {
            mob.nav_move_to(cx.w, position.components(), 1.2);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct SitAndEat {
    cooldown: u32,
}

impl Goal for SitAndEat {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { MOVE }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        let pose = state(&mob.ai.nav);
        mob.age >= self.cooldown && !mob.is_baby() && on_land(mob, cx.w)
            && !pose.sleeping && !pose.eating_sit && !no_food(mob)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool { on_land(mob, cx.w) }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.tick(mob, cx);
        self.cooldown = 0;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if !no_food(mob) && on_land(mob, cx.w) && !state(&mob.ai.nav).eating_sit {
            mob.ai.zza = 0.0;
            mob.nav_stop();
            state_mut(&mut mob.ai.nav).eating_sit = true;
        }
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        if let Some(stack) = mob.extra.main_hand_stack().cloned() {
            state_mut(&mut mob.ai.nav).pending_spit = Some(stack);
            mob.extra.set_main_hand_stack(None);
            self.cooldown = mob.age + (10 + mob.random.next_int_bound(150).unwrap_or(0) as u32) * 20;
        }
        state_mut(&mut mob.ai.nav).eating_sit = false;
    }
}

pub(super) fn server_tick(mob: &mut Mob, world: &MobWorld, events: &mut Vec<MobEvent>) {
    let holding = !no_food(mob);
    let pose = state_mut(&mut mob.ai.nav);
    pose.wake = (pose.wake - 1).max(0);
    if !mob.no_ai {
        pose.hunting = (pose.hunting - 1).max(0);
    }
    if pose.sleeping {
        mob.ai.jump = false;
        mob.ai.xxa = 0.0;
        mob.ai.zza = 0.0;
    }
    if holding {
        pose.sniffing = false;
        if mob.extra.anger > 0 {
            mob.ai.target = None;
            mob.extra.angry_at = None;
            super::neutral_anger::stop(mob);
        }
    } else {
        super::neutral_anger::resume_anger(mob, world.game_time);
        if let Some(uuid) = state(&mob.ai.nav).loaded_anger_target {
            if mob.ai.target.is_some() {
                state_mut(&mut mob.ai.nav).loaded_anger_target = None;
            } else if let Some(player) = world.player.as_ref().filter(|player| player.uuid == uuid) {
                state_mut(&mut mob.ai.nav).loaded_anger_target = None;
                if mob.extra.anger > 0 {
                    mob.extra.angry_at = Some(Who::Player);
                    if player.alive && !player.creative && !player.spectator && world.difficulty.id != 0
                        && !super::tame::owned_by(mob.extra.owner, player.uuid)
                    {
                        mob.ai.target = Some(Who::Player);
                    }
                }
            } else if mob.extra.anger > 0 && mob.extra.angry_at.is_none() {
                // An unresolved UUID still excludes universal anger.
                mob.extra.angry_at = Some(Who::Mob(0));
            }
        }
        super::tame::update_persistent_anger(mob, world);
        super::neutral_anger::settle_anger_end(mob, world.game_time);
    }
    let pose = state_mut(&mut mob.ai.nav);
    let seated = pose.eating_sit;
    if !holding || !seated {
        pose.eat_counter = 0;
        return;
    }
    if pose.eat_counter == 0 && !pose.sleeping && mob.random.next_int_bound(40).unwrap_or(0) == 1 {
        pose.eat_counter = 1;
    }
    if pose.eat_counter == 0 {
        return;
    }
    if pose.eat_counter > 40 {
        pose.eat_counter = 0;
        if let Some(item) = mob.extra.main_hand.as_deref()
            && mob.kind.food.iter().any(|food| food == item)
        {
            let salmon = item == "minecraft:salmon";
            mob.extra.set_main_hand_stack(None);
            pose.sheared = false;
            pose.eating_sit = false;
            let at = mob.position.components();
            events.push(MobEvent::GameEvent {
                event: "minecraft:eat", at, source: mob.id, state: horizon_world::registry::BlockStateId(0),
            });
            if salmon {
                let yaw = f64::from(mob.body_yaw.to_radians());
                events.push(MobEvent::DropItem {
                    mob: mob.id, item: std::sync::Arc::from("minecraft:bone_meal"), count: 1,
                    at: [at[0] - 1.5 * yaw.sin(), at[1] + 0.5, at[2] + 1.5 * yaw.cos()],
                });
            }
        }
        return;
    }
    if mob.kind.id == BEAR && (pose.eat_counter == 1 || pose.eat_counter % 5 == 0) {
        let volume = 0.5 + 0.5 * mob.random.next_int_bound(2).unwrap_or(0) as f32;
        let pitch = 1.0 + (mob.random.next_float() - mob.random.next_float()) * 0.2;
        if !mob.silent {
            events.push(MobEvent::SoundId("naturalist:entity.bear.eat".to_owned(), mob.position.components(), volume, pitch));
        }
    }
    pose.eat_counter += 1;
}

pub(super) fn angry_player(mob: &Mob, world: &MobWorld) -> bool {
    mob.extra.anger > 0
        && (mob.extra.angry_at == Some(Who::Player)
            || (world.universal_anger && mob.extra.angry_at.is_none()))
}

pub fn render_pose(mob: &Mob) -> u8 {
    let pose = state(&mob.ai.nav);
    u8::from(pose.sleeping)
        | (u8::from(pose.sniffing) << 1)
        | (u8::from(pose.eating_sit || mob.extra.sitting_pose) << 2)
        | (u8::from(pose.eat_counter > 0) << 3)
        | (u8::from(pose.sheared) << 4)
        | (u8::from(mob.extra.anger > 0 || mob.ai.target.is_some()) << 5)
        | match mob.extra.main_hand.as_deref() {
            Some("minecraft:sweet_berries") => 1 << 6,
            Some("minecraft:honeycomb") => 2 << 6,
            _ => 0,
        }
}

#[derive(Clone, Copy)]
pub enum FoodSource {
    Berries(u32),
    Hive,
    Campfire(usize),
}

pub fn food_source(world: &MobWorld, pos: [i32; 3], food: &[String]) -> Option<FoodSource> {
    use crate::containers::BlockEntity;
    let [x, y, z] = pos;
    let id = world.world.block(x, y, z);
    let block = world.registry.state(id)?;
    match block.name.as_str() {
        "minecraft:sweet_berry_bush" => {
            let age = block.properties.get("age")?.parse::<u32>().ok()?;
            (age >= 2).then_some(FoodSource::Berries(age))
        }
        "minecraft:beehive" | "minecraft:bee_nest" => {
            let honey = block.properties.get("honey_level")?.parse::<u32>().ok()?;
            (honey >= 5).then_some(FoodSource::Hive)
        }
        "minecraft:campfire" => {
            let BlockEntity::Campfire(campfire) = world.world.block_entities.get(&(x, y, z))? else {
                return None;
            };
            campfire.items.iter().position(|item| {
                item.as_ref().is_some_and(|item| food.iter().any(|name| name == item.item.as_ref()))
            }).map(FoodSource::Campfire)
        }
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub(super) struct HarvestFood {
    movement: MoveToBlock,
    waited: u16,
}

impl HarvestFood {
    pub(super) fn new() -> Self {
        Self { movement: MoveToBlock::new(1.2, 12, 3), waited: 0 }
    }
}

fn reach_block(movement: &MoveToBlock, world: &MobWorld, flower: bool) -> [f64; 3] {
    let [x, mut y, z] = movement.block;
    if !flower {
        while y > world.world.shape.column().min_y()
            && world.world.block(x, y - 1, z) == world.world.air
        {
            y -= 1;
        }
    }
    [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5]
}

fn start_block(movement: &mut MoveToBlock, mob: &mut Mob, world: &MobWorld, flower: bool) {
    movement.try_ticks = 0;
    movement.reached = false;
    let r = mob.random.next_int_bound(1200).unwrap_or(0) + 1200;
    movement.max_stay = mob.random.next_int_bound(r).unwrap_or(0) + 1200;
    mob.nav_move_to(world, reach_block(movement, world, flower), movement.speed);
}

fn tick_block(movement: &mut MoveToBlock, mob: &mut Mob, world: &MobWorld, distance: f64, flower: bool) {
    let at = reach_block(movement, world, flower);
    let center = Vec3::new(at[0], at[1] + 0.5, at[2]);
    movement.reached = mob.position.distance_squared(center) < distance * distance;
    if movement.reached {
        movement.try_ticks -= 1;
    } else {
        movement.try_ticks += 1;
        if movement.try_ticks % 100 == 0 {
            mob.nav_move_to(world, at, movement.speed);
        }
    }
}

fn sniff(mob: &Mob, cx: &mut Cx) {
    if !mob.silent {
        cx.events.push(MobEvent::SoundId(sniff_sound(mob).to_owned(), mob.position.components(), 1.0, 1.0));
    }
}

impl Goal for HarvestFood {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { MOVE | JUMP }
    fn every_tick(&self) -> bool { true }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.is_baby() || !no_food(mob) {
            return false;
        }
        let food = mob.kind.food.clone();
        if self.movement.next_start > 0 {
            self.movement.next_start -= 1;
            return false;
        }
        let delay = super::goals::reduced(200 + mob.random.next_int_bound(200).unwrap_or(0));
        self.movement.can_use(mob, cx.w, delay, |w, pos| food_source(w, pos, &food).is_some())
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        no_food(mob) && self.movement.can_continue(cx.w, |w, pos| food_source(w, pos, &mob.kind.food).is_some())
    }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.waited = 0;
        start_block(&mut self.movement, mob, cx.w, false);
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if self.movement.reached {
            if self.waited >= 40 {
                if cx.w.mob_griefing {
                    let status = state_mut(&mut mob.ai.nav);
                    status.sniffing = false;
                    status.harvest = Some(self.movement.block);
                }
            } else {
                self.waited += 1;
            }
        } else if mob.random.next_float() < 0.05 {
            sniff(mob, cx);
            state_mut(&mut mob.ai.nav).sniffing = true;
        }
        let [x, y, z] = self.movement.block;
        mob.look_at_speed([f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5], 10.0, 40.0);
        tick_block(&mut self.movement, mob, cx.w, 3.0, false);
    }
}

#[derive(Clone, Debug)]
pub(super) struct SniffFlowers {
    movement: MoveToBlock,
    waited: u16,
}

impl SniffFlowers {
    pub(super) fn new() -> Self {
        Self { movement: MoveToBlock::new(1.2, 12, 3), waited: 0 }
    }
}

fn flower(world: &MobWorld, [x, y, z]: [i32; 3]) -> bool {
    world.block_tags.get("minecraft:flowers").is_some_and(|tag| tag.contains(world.block_name(x, y, z)))
}

impl Goal for SniffFlowers {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { MOVE | JUMP | LOOK }
    fn every_tick(&self) -> bool { true }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if !mob.is_baby() || !no_food(mob) { return false }
        if self.movement.next_start > 0 {
            self.movement.next_start -= 1;
            return false;
        }
        self.movement.next_start = super::goals::reduced(200 + mob.random.next_int_bound(200).unwrap_or(0));
        if !self.movement.may_find(mob, cx.w, |id| {
            cx.w.block_tags.get("minecraft:flowers")
                .is_some_and(|tag| tag.contains(cx.w.facts.get(id).name.as_ref()))
        }) {
            return false;
        }
        let [x, y, z] = mob.position.components().map(|v| v.floor() as i32);
        let home = mob.ai.home;
        let range = self.movement.range;
        let mut choices = Vec::new();
        for dy in [0, 1, -1, 2, -2, 3, -3] {
            for dx in -range..=range {
                for dz in -range..=range {
                    let pos = [x + dx, y + dy - 1, z + dz];
                    if home.is_none_or(|(center, radius)| within_home(center, radius, pos)) && flower(cx.w, pos) {
                        choices.push(pos);
                    }
                }
            }
        }
        let Ok(index) = mob.random.next_int_bound(choices.len() as i32) else { return false };
        self.movement.block = choices[index as usize];
        true
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.waited < 100 && mob.is_baby() && self.movement.can_continue(cx.w, flower)
    }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.waited = 0;
        start_block(&mut self.movement, mob, cx.w, true);
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(&mut mob.ai.nav).sniffing = false;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if self.movement.reached {
            mob.nav_stop();
            if self.waited % 20 == 0 { sniff(mob, cx); }
            self.waited += 1;
        }
        let [x, y, z] = self.movement.block.map(|v| f64::from(v) + 0.5);
        mob.look_at_speed([x, y, z], 30.0, 40.0);
        tick_block(&mut self.movement, mob, cx.w, 2.0, true);
        state_mut(&mut mob.ai.nav).sniffing = self.movement.reached;
    }
}
