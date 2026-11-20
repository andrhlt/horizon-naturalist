use super::super::{Mob, MobEvent, MobWorld, Species, goals::{Cx, Goal, JUMP, MOVE, MoveToBlock, goal_clone, reduced}};
use horizon_core::random::RandomSource;

const NESTS: [(&str, &str); 3] = [
    ("naturalist:alligator_egg", "naturalist:entity.alligator.egg_break"),
    ("naturalist:tortoise_egg", "naturalist:entity.tortoise.egg_break"),
    ("naturalist:ostrich_egg", "naturalist:entity.ostrich.egg_break"),
];

pub(super) fn register(mob: &mut Mob) {
    if !matches!(mob.species(), Species::Zombie | Species::Husk | Species::ZombieVillager | Species::Drowned | Species::ZombifiedPiglin) {
        return;
    }
    for (block, sound) in NESTS {
        mob.ai.goals.add(2, AttackEgg { move_to: MoveToBlock::new(1.0, 24, 3), reached_ticks: 0, block, sound });
    }
}

#[derive(Clone, Debug)]
struct AttackEgg {
    move_to: MoveToBlock,
    reached_ticks: i32,
    block: &'static str,
    sound: &'static str,
}

fn valid(world: &MobWorld<'_>, [x, y, z]: [i32; 3], block: &str) -> bool {
    world.world.loaded(x, z) && world.block_name(x, y, z) == block
        && world.facts.is_air(world.world.block(x, y + 1, z))
        && world.facts.is_air(world.world.block(x, y + 2, z))
}

impl Goal for AttackEgg {
    goal_clone!();
    fn flags(&self) -> u8 { MOVE | JUMP }
    fn every_tick(&self) -> bool { true }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if !cx.w.mob_griefing { return false; }
        if self.move_to.next_start > 0 {
            self.move_to.next_start -= 1;
            return false;
        }
        let block = self.block;
        let found = self.move_to.may_find(mob, cx.w, |state| &*cx.w.facts.get(state).name == block)
            && self.move_to.find_nearest(mob, cx.w, |world, pos| valid(world, pos, block));
        self.move_to.next_start = reduced(if found { 20 } else { 200 + mob.random.next_int_bound(200).unwrap_or(0) });
        found
    }
    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        let block = self.block;
        self.move_to.can_continue(cx.w, |world, pos| valid(world, pos, block))
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.move_to.start(mob, cx.w);
        self.reached_ticks = 0;
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.fall_distance = 1.0;
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.move_to.tick_within(mob, cx.w, 1.14);
        if !self.move_to.reached { return; }
        let [x, y, z] = mob.position.components().map(|axis| axis.floor() as i32);
        let Some(pos) = [[x,y,z], [x,y-1,z], [x-1,y,z], [x+1,y,z], [x,y,z-1], [x,y,z+1], [x,y-2,z]]
            .into_iter().find(|&[x,y,z]| cx.w.block_name(x,y,z) == self.block)
        else { return };
        let [a, b, c] = pos.map(|axis| f64::from(axis) + 0.5);
        if self.reached_ticks > 0 {
            mob.velocity.y = 0.3;
            for _ in 0..3 { mob.random.next_float(); }
            cx.events.push(MobEvent::Particles { kind: "item_egg", at: [a, b + 0.2, c], count: 3 });
        }
        if self.reached_ticks % 2 == 0 {
            mob.velocity.y = -0.3;
            if self.reached_ticks % 6 == 0 {
                let center = self.move_to.block.map(|axis| f64::from(axis) + 0.5);
                cx.events.push(MobEvent::Sound("entity.zombie.destroy_egg", center, 0.5, 0.9 + cx.lr.next_float() * 0.2));
            }
        }
        if self.reached_ticks > 60 {
            cx.events.push(MobEvent::BreakBlock { pos, drop: false });
            for _ in 0..20 {
                let spread = [(); 3].map(|_| mob.random.next_gaussian() * 0.02);
                cx.events.push(MobEvent::LevelParticles {
                    kind: "poof", at: [a, b - 0.5, c], spread, speed: 0.15, count: 1,
                });
            }
            cx.events.push(MobEvent::Sound(self.sound, [a,b,c], 0.7, 0.9 + cx.lr.next_float() * 0.2));
        }
        self.reached_ticks += 1;
    }
}
