//! Alligator breeding and egg laying goals.
use super::Game;
use horizon_client::mobs::{
    BreedExperience, Mob, MobEvent, MobWorld,
    goals::{self, Cx, Goal, JUMP, LOOK, MOVE, MoveToBlock},
};
use horizon_core::random::RandomSource;

#[derive(Clone, Debug, Default)]
struct AlligatorBreed {
    partner: Option<u32>,
    love_time: i32,
}

impl Goal for AlligatorBreed {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        "NaturalistAlligatorBreed"
    }
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.is_baby() || mob.extra.in_love <= 0 || mob.extra.flag {
            return false;
        }
        let id = mob.id;
        self.partner =
            cx.w.nearby(mob.position, 8.0, move |seen| {
                seen.id != id
                    && seen.kind.id == "naturalist:alligator"
                    && seen.in_love
                    && !seen.baby
                    && !seen.panicking
            })
            .find(|seen| mob.distance_sqr(seen.position) <= 64.0)
            .map(|seen| seen.id);
        self.partner.is_some()
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !mob.is_baby()
            && mob.extra.in_love > 0
            && self
                .partner
                .and_then(|id| cx.w.seen(id))
                .is_some_and(|seen| seen.alive && seen.in_love && !seen.panicking)
            && self.love_time < 60
    }

    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.partner = None;
        self.love_time = 0;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(partner) = self.partner.and_then(|id| cx.w.seen(id)).cloned() else {
            return;
        };
        mob.look_at_speed(
            [partner.position.x, partner.eye_y, partner.position.z],
            10.0,
            mob.max_head_x_rot(),
        );
        mob.nav_move_to_entity(
            cx.w,
            [partner.position.x, partner.position.y, partner.position.z],
            1,
            1.0,
        );
        self.love_time += 1;
        if self.love_time >= goals::reduced(60) && mob.distance_sqr(partner.position) < 9.0 {
            let experience = if cx.w.asks_baby_spawn {
                BreedExperience::Deferred
            } else {
                BreedExperience::Drawn(horizon_client::mobs::breeding_experience(
                    mob,
                    cx.w.mob_drops,
                ))
            };
            mob.extra.in_love = 0;
            mob.extra.age = 6000;
            mob.extra.flag = true;
            cx.events.push(MobEvent::Breed {
                parent: mob.id,
                partner: partner.id,
                equine: None,
                parent_variant: None,
                wolf_voice: None,
                experience,
            });
            self.love_time = 60;
        }
    }
}

#[derive(Clone, Debug)]
struct LayAlligatorEgg(MoveToBlock);

impl LayAlligatorEgg {
    fn valid(world: &MobWorld, [x, y, z]: [i32; 3]) -> bool {
        if !world.facts.is_air(world.world.block(x, y + 1, z)) {
            return false;
        }
        let Some(state) = world.registry.state(world.world.block(x, y, z)) else {
            return false;
        };
        matches!(
            state.name.as_str(),
            "minecraft:grass_block" | "minecraft:dirt" | "minecraft:sand" | "minecraft:mud"
        )
    }
}

impl Goal for LayAlligatorEgg {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        "NaturalistLayAlligatorEgg"
    }
    fn flags(&self) -> u8 {
        MOVE | JUMP
    }
    fn every_tick(&self) -> bool {
        true
    }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.is_baby() || !mob.extra.flag {
            return false;
        }
        if self.0.next_start > 0 {
            self.0.next_start -= 1;
            return false;
        }
        self.0.next_start = goals::reduced(200 + mob.random.next_int_bound(200).unwrap_or(0));
        self.0.find_nearest(mob, cx.w, Self::valid)
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(cx.w, Self::valid) && mob.extra.flag && !mob.is_baby()
    }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.start(mob, cx.w);
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.extra.flag2 = false;
        mob.extra.counter = 0;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.tick(mob, cx.w);
        if mob.fluids.in_water || !self.0.reached {
            return;
        }
        if mob.extra.counter < 1 {
            mob.extra.flag2 = true;
        }
        mob.extra.counter += 1;
        if mob.extra.counter > 60 {
            let registry = cx.w.registry;
            let Some(mut egg_state) = registry
                .default_state("naturalist:alligator_egg")
                .and_then(|default| registry.state(default).cloned())
            else {
                return;
            };

            let at = [mob.position.x, mob.position.y, mob.position.z];
            let pitch = 0.9 + cx.lr.next_float() * 0.2;
            let eggs = mob.random.next_int_bound(4).unwrap_or(0) + 1;
            egg_state.properties.insert("eggs".into(), eggs.to_string());
            egg_state.properties.insert("hatch".into(), "0".into());
            let Ok(state) = registry.resolve(&egg_state) else {
                return;
            };
            cx.events
                .push(MobEvent::Sound("entity.turtle.lay_egg", at, 0.3, pitch));
            let pos = [self.0.block[0], self.0.block[1] + 1, self.0.block[2]];
            cx.events.push(MobEvent::SetBlock { pos, state });
            cx.events.push(MobEvent::GameEvent {
                event: "minecraft:block_place",
                at: [
                    f64::from(pos[0]) + 0.5,
                    f64::from(pos[1]) + 0.5,
                    f64::from(pos[2]) + 0.5,
                ],
                source: mob.id,
                state,
            });
            mob.extra.flag = false;
            mob.extra.flag2 = false;
            mob.extra.counter = 0;
            mob.extra.in_love = 600;
        }
    }
}

pub(super) fn install(game: &mut Game<'_>, id: u32) {
    use mod_api::Host;
    if let Some(mob) = game.entity(id).and_then(|entity| game.mob(entity)) {
        mob.remove_goal(game, "BreedGoal");
    }
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
        if !mob.ai.goals.has("NaturalistAlligatorBreed") {
            mob.ai.goals.add(6, AlligatorBreed::default());
        }
        if !mob.ai.goals.has("NaturalistLayAlligatorEgg") {
            mob.ai
                .goals
                .add(7, LayAlligatorEgg(MoveToBlock::new(1.0, 16, 4)));
        }
    }
}
