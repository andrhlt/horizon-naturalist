use super::{Breed, Cx, Float, Goal, JUMP, LOOK, LookAtPlayer, MOVE, Mob, MobEvent, MobWorld,
    MoveToBlock, Stroll, Tempt, goal_clone, reduced};
use crate::mobs::{PlayerView, finalize::{Attr, set_base}, naturalist_ostrich as ostrich, tame::naturalist_duck_pet_goals};
use horizon_core::{geometry::Vec3, random::RandomSource};
use std::sync::Arc;

pub const KIND: &str = "naturalist:tortoise";
pub const EGG: &str = "naturalist:tortoise_egg";
pub const COLORS: [&str; 3] = ["naturalist:brown", "naturalist:green", "naturalist:black"];
const FOODS: [&str; 4] = ["minecraft:bamboo", "minecraft:dandelion", "minecraft:brown_mushroom", "minecraft:cactus"];
const SUBSTRATE: &str = "naturalist:tortoise_egg_layable_on";

pub fn variant_index(name: &str) -> u8 {
    COLORS.iter().position(|id| *id == name).unwrap_or(0) as u8
}

pub fn hidden(mob: &Mob) -> bool { *mob.ai.nav.__horizon_mixins_naturalist_tortoise_hidden() }

pub fn set_hidden(mob: &mut Mob, value: bool) {
    *mob.ai.nav.__horizon_mixins_naturalist_tortoise_hidden_mut() = value;
}

pub fn render_pose(mob: &Mob) -> u8 {
    u8::from(hidden(mob)) | (u8::from(mob.extra.flag2) << 1) | (u8::from(mob.extra.sitting_pose) << 2)
}

pub fn can_hide(mob: &Mob, world: &MobWorld<'_>, player: &PlayerView) -> bool {
    if mob.extra.tame || !player.alive || player.creative || player.spectator || player.sneaking
        || [player.main_hand.as_deref(), player.off_hand.as_deref()].into_iter().flatten()
            .any(|item| FOODS.contains(&item))
        || !mob.bounds().inflate(Vec3::new(5.0, 3.0, 5.0)).intersects(player.bounds)
    {
        return false;
    }
    let range = (5.0 * player.visibility_percent(Some(KIND), world.items)).max(2.0);
    mob.distance_sqr(player.position) <= range * range
        && world.line_of_sight_for(Some(mob.position.y), Vec3::new(mob.position.x, mob.eye_y(), mob.position.z),
            Vec3::new(player.eye[0], player.eye[1], player.eye[2]))
}

pub fn tame_attributes(mob: &mut Mob, healed: bool) {
    set_base(mob, Attr::MaxHealth, if mob.extra.tame { 30.0 } else { 20.0 });
    set_base(mob, Attr::AttackDamage, 4.0);
    if mob.extra.tame && healed {
        mob.health = mob.attrs.max_health;
    }
}

pub fn register(mob: &mut Mob) {
    let food: Arc<[String]> = FOODS.map(str::to_owned).into();
    let kind = Arc::make_mut(&mut mob.kind);
    kind.food = food.clone();
    kind.tempt = food.clone();
    kind.water_slowdown = 0.96;
    kind.max_head_y_rot = 75.0;
    mob.ai.goals.clear();
    mob.ai.goals.add(0, Float);
    mob.ai.goals.add(1, EggBreed(Breed::new(1.0)));
    mob.ai.goals.add(1, LayEgg(MoveToBlock::new(1.0, 16, 1)));
    naturalist_duck_pet_goals::add_tortoise_goals(mob);
    mob.ai.goals.add(1, Hide);
    mob.ai.goals.add(2, Tempt::new(1.0, food, false));
    mob.ai.goals.add(4, Breed::new(1.0));
    mob.ai.goals.add(5, Stroll::avoiding_water(1.0));
    mob.ai.goals.add(6, LookAtPlayer::new(10.0));
}

#[derive(Clone, Debug)]
struct Hide;
impl Goal for Hide {
    goal_clone!();
    fn flags(&self) -> u8 { MOVE | LOOK | JUMP }
    fn can_use(&mut self, mob: &mut Mob, _: &mut Cx) -> bool { hidden(mob) }
    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.ai.jumping = false;
        mob.ai.jump = false;
        mob.nav_stop();
        mob.ai.mov.set_wanted(mob.position.x, mob.position.y, mob.position.z, 0.0);
    }
}

#[derive(Clone, Debug)]
struct EggBreed(Breed);
impl Goal for EggBreed {
    goal_clone!();
    fn flags(&self) -> u8 { self.0.flags() }
    fn every_tick(&self) -> bool { self.0.every_tick() }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !has_egg(mob) && self.0.can_use(mob, cx)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool { self.0.can_continue(mob, cx) }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.start(mob, cx); }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.stop(mob, cx); }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.tick(mob, cx); }
}

pub(in crate::mobs) fn add_ostrich_egg_goals(mob: &mut Mob) {
    mob.ai.goals.add(3, EggBreed(Breed::new(1.0)));
    mob.ai.goals.add(3, LayEgg(MoveToBlock::new(1.0, 16, 1)));
}

fn has_egg(mob: &Mob) -> bool {
    if mob.kind.id == ostrich::KIND { ostrich::state(mob).has_egg } else { mob.extra.flag }
}

fn substrate(mob: &Mob) -> &'static str {
    if mob.kind.id == ostrich::KIND { ostrich::SUBSTRATE } else { SUBSTRATE }
}

fn site(world: &MobWorld<'_>, [x, y, z]: [i32; 3], substrate: &str) -> bool {
    world.has_tag(substrate, world.block_name(x, y, z))
        && world.facts.is_air(world.world.block(x, y + 1, z))
}

#[derive(Clone, Debug)]
struct LayEgg(MoveToBlock);
impl Goal for LayEgg {
    goal_clone!();
    fn flags(&self) -> u8 { MOVE | JUMP }
    fn every_tick(&self) -> bool { true }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if !has_egg(mob) { return false; }
        if self.0.next_start > 0 {
            self.0.next_start -= 1;
            return false;
        }
        self.0.next_start = reduced(200 + mob.random.next_int_bound(200).unwrap_or(0));
        let substrate = substrate(mob);
        self.0.find_nearest(mob, cx.w, |world, pos| site(world, pos, substrate))
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        let substrate = substrate(mob);
        has_egg(mob) && self.0.can_continue(cx.w, |world, pos| site(world, pos, substrate))
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.start(mob, cx.w); }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        use super::naturalist_insect_goals::state_property;
        self.0.tick(mob, cx.w);
        let ostrich = mob.kind.id == ostrich::KIND;
        if mob.extra.sitting_pose {
            if ostrich { ostrich::state_mut(mob).lay_ticks = 0; }
            else { mob.extra.flag2 = false; }
            return;
        }
        if mob.fluids.in_water || !self.0.reached { return; }
        let counter = if ostrich { i32::from(ostrich::state(mob).lay_ticks) } else { mob.extra.counter };
        if counter > 60 {
            let [x, y, z] = self.0.block;
            let at = mob.position.components().map(|value| value.floor() + 0.5);
            cx.events.push(MobEvent::Sound("entity.turtle.lay_egg", at, 0.3, 0.9 + cx.lr.next_float() * 0.2));
            let eggs = 1 + mob.random.next_int_bound(4).unwrap_or(0);
            if ostrich {
                if let Some(state) = cx.w.registry.default_state(ostrich::EGG) {
                    let pos = [x, y + 1, z];
                    cx.events.push(MobEvent::SetBlock { pos, state });
                    cx.events.push(MobEvent::LevelEvent { id: 2005, pos, data: 0 });
                    ostrich::remember_egg(mob, pos);
                }
            } else if let Some(base) = cx.w.registry.default_state(EGG)
                && let Some(state) = state_property(cx.w.registry, base, "eggs", &eggs.to_string())
                && let Some(state) = state_property(cx.w.registry, state, "variant", &mob.extra.variant.min(2).to_string())
            {
                cx.events.push(MobEvent::SetBlock { pos: [x, y + 1, z], state });
                cx.events.push(MobEvent::LevelEvent { id: 2005, pos: [x, y + 1, z], data: 0 });
            }
            if ostrich {
                let state = ostrich::state_mut(mob);
                state.has_egg = false;
                state.lay_ticks = 0;
            } else {
                mob.extra.flag = false;
                mob.extra.flag2 = false;
                mob.extra.counter = 0;
            }
            mob.extra.in_love = 600;
        } else if ostrich {
            ostrich::state_mut(mob).lay_ticks += 1;
        } else {
            mob.extra.flag2 = true;
            mob.extra.counter += if mob.extra.counter == 0 { 2 } else { 1 };
        }
    }
}

pub fn dig_effect(mob: &Mob, cx: &mut Cx) {
    if !mob.alive() || !mob.extra.flag2 || mob.extra.counter < 1 || mob.extra.counter % 5 != 0 {
        return;
    }
    let pos = mob.position.components().map(|value| value.floor() as i32);
    let [x, y, z] = pos;
    if cx.w.has_tag(SUBSTRATE, cx.w.block_name(x, y - 1, z)) {
        cx.events.push(MobEvent::BlockBreakEffect { pos, block: [x, y - 1, z] });
    }
}
