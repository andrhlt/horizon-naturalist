use super::super::{
    naturalist_snail::{snail, snail_mut},
    naturalist_snail_control::crawl_to,
};
use super::{
    Breed, Cx, Goal, JUMP, LookAtPlayer, MOVE, Mob, MobEvent, MobWorld, MoveToBlock,
    RandomLookAround, Stroll, goal_clone, reduced,
};
use horizon_core::{geometry::Direction, random::RandomSource};

pub fn register(mob: &mut Mob) {
    mob.ai.mov.kind = super::super::control::MoveKind::Custom;
    mob.ai.goals.clear();
    mob.ai.goals.add(1, SnailBreed(Breed::new(1.0)));
    mob.ai
        .goals
        .add(1, LayEggs(MoveToBlock::new(1.0, 16, 1), 0));
    let mut stroll = Stroll::avoiding_water(0.9).interval(1);
    stroll.water_avoiding = Some(0.0);
    stroll.force = true;
    mob.ai.goals.add(
        2,
        SnailStroll {
            stroll,
            target: None,
        },
    );
    mob.ai.goals.add(3, LookAtPlayer::new(6.0));
    mob.ai.goals.add(4, RandomLookAround::default());
}

#[derive(Clone, Debug)]
struct SnailStroll {
    stroll: Stroll,
    target: Option<[i32; 3]>,
}

fn surface_position(mob: &mut Mob, world: &MobWorld) -> Option<[i32; 3]> {
    let [x, y, z] = [mob.position.x, mob.position.y, mob.position.z].map(|v| v.floor() as i32);
    for _ in 0..10 {
        let mut draw = |width| mob.random.next_int_bound(width).unwrap_or(0);
        let pos = [x + draw(13) - 6, y + draw(9) - 4, z + draw(13) - 6];
        if world
            .block_collision_boxes(pos[0], pos[1], pos[2])
            .is_empty()
            && Direction::ALL.into_iter().any(|direction| {
                let [dx, dy, dz] = direction.normal();
                let state = world.world.block(pos[0] + dx, pos[1] + dy, pos[2] + dz);
                world.fluid_tables.sturdy(state, direction.opposite())
            })
        {
            return Some(pos);
        }
    }
    None
}

impl Goal for SnailStroll {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE
    }

    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        let force = self.stroll.force;
        if !force && (m.ai.no_action_time >= 100 || m.random.next_int_bound(1) != Ok(0)) {
            return false;
        }
        self.target = if m.random.next_float() < 0.3 {
            surface_position(m, cx.w)
        } else {
            None
        };
        if self.target.is_some() {
            self.stroll.force = false;
            return true;
        }

        self.stroll.force = true;
        let selected = self.stroll.can_use(m, cx);
        if !selected {
            self.stroll.force = force;
        }
        selected
    }

    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.stroll.can_continue(m, cx)
    }

    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some(target) = self.target {
            m.nav_stop();
            crawl_to(m, target, self.stroll.speed);
        } else {
            self.stroll.start(m, cx);
        }
    }

    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.stroll.stop(m, cx);
    }
}

#[derive(Clone, Debug)]
struct SnailBreed(Breed);

impl Goal for SnailBreed {
    goal_clone!();
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        !snail(m).has_egg && self.0.can_use(m, cx)
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(m, cx)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx);
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.stop(m, cx);
    }
    fn every_tick(&self) -> bool {
        self.0.every_tick()
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.tick(m, cx);
    }
}

fn valid_site(w: &MobWorld, [x, y, z]: [i32; 3]) -> bool {
    w.facts.is_air(w.world.block(x, y + 1, z))
        && w.has_tag("naturalist:alligator_egg_layable_on", w.block_name(x, y, z))
}

#[derive(Clone, Debug)]
struct LayEggs(MoveToBlock, i32);

impl Goal for LayEggs {
    goal_clone!();
    fn flags(&self) -> u8 {
        MOVE | JUMP
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if !snail(m).has_egg {
            return false;
        }
        if self.0.next_start > 0 {
            self.0.next_start -= 1;
            return false;
        }
        self.0.next_start = reduced(200 + m.random.next_int_bound(200).unwrap_or(0));
        self.0.find_nearest(m, cx.w, valid_site)
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        snail(m).has_egg && self.0.can_continue(cx.w, valid_site)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx.w);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.tick(m, cx.w);
        if m.fluids.in_water || !self.0.reached {
            return;
        }
        if self.1 > reduced(60) {
            let at = [m.position.x, m.position.y, m.position.z];
            let pitch = 0.9 + cx.lr.next_float() * 0.2;
            cx.events
                .push(MobEvent::Sound("block.turtle_egg.lay", at, 0.3, pitch));
            let _eggs = m.random.next_int_bound(4).unwrap_or(0) + 1;
            let [x, y, z] = self.0.block;
            if let Some(state) = cx.w.registry.default_state("naturalist:snail_eggs") {
                cx.events.push(MobEvent::SetBlock {
                    pos: [x, y + 1, z],
                    state,
                });
            }
            snail_mut(m).has_egg = false;
            m.extra.in_love = 600;
            self.1 = 0;
        } else {
            self.1 += if self.1 == 0 { 2 } else { 1 };
        }
    }
}
