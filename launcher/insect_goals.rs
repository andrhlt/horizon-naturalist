use super::{Breed, Cx, Float, FollowParent, Goal, JUMP, MOVE, Mob, MobWorld, MoveToBlock, Tempt, goal_clone};
use horizon_core::{geometry::Vec3, random::RandomSource};
use horizon_world::registry::{BlockRegistry, BlockStateId};
use std::sync::Arc;

pub(super) fn configure_small_flyer(m: &mut Mob) {
    use super::super::{control::MoveKind, path::PathType};
    m.ai.mov.kind = MoveKind::Flying { max_turn: 20.0, hovers: true };
    for (path, malus) in [
        (PathType::Fire, PathType::Fire.malus()),
        (PathType::Water, -1.0),
        (PathType::WaterBorder, 16.0),
        (PathType::Fence, -1.0),
    ] {
        m.set_malus(path, malus);
    }
    m.ai.goals.clear();
}

pub fn register_butterfly(m: &mut Mob) {
    configure_small_flyer(m);
    m.ai.goals.add(1, Breed::new(1.0));
    m.ai.goals.add(2, Tempt::new(1.25, Arc::from(["#minecraft:flowers".to_owned()]), false));
    m.ai.goals.add(3, FollowParent::new(1.25));
    m.ai.goals.add(4, Pollination::new(true));
    m.ai.goals.add(5, Pollination::new(false));
    m.ai.goals.add(6, FlyingWander);
    m.ai.goals.add(7, Float);
}

#[derive(Clone, Debug)]
pub(super) struct FlyingWander;

impl Goal for FlyingWander {
    goal_clone!();

    fn flags(&self) -> u8 { MOVE }

    fn can_use(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        m.ai.nav.is_done() && m.random.next_int_bound(10) == Ok(0)
    }

    fn can_continue(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        !m.ai.nav.is_done()
    }

    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some(target) = m.flying_stroll_pos(cx.w) {
            m.nav_move_to_entity(cx.w, target.map(f64::floor), 1, 1.0);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Nectar {
    present: bool,
    crops: u8,
}

pub fn finish_butterfly_step(m: &mut Mob) {
    if m.kind.id == "naturalist:butterfly" {
        let nectar = m.ai.nav.__horizon_mixins_naturalist_butterfly_nectar_mut();
        if nectar.crops >= 10 {
            *nectar = Nectar::default();
        }
    }
}

#[derive(Clone, Debug)]
struct Pollination {
    path: MoveToBlock,
    grow: bool,
    waited: i32,
}

impl Pollination {
    fn new(grow: bool) -> Self {
        Self { path: MoveToBlock::new(1.0, 16, 4), grow, waited: 0 }
    }
}

fn valid_pollination(grow: bool, w: &MobWorld, [x, y, z]: [i32; 3]) -> bool {
    if grow {
        return crop_age(w, [x, y, z]).is_some_and(|(age, max)| age < max);
    }
    w.has_tag("minecraft:flowers", w.block_name(x, y, z))
}

fn crop_age(w: &MobWorld, [x, y, z]: [i32; 3]) -> Option<(i32, i32)> {
    let max = match w.block_name(x, y, z) {
        "minecraft:wheat" | "minecraft:carrots" | "minecraft:potatoes" => 7,
        "minecraft:beetroots" => 3,
        "minecraft:torchflower_crop" => 2,
        _ => return None,
    };
    Some((w.prop(x, y, z, "age")?.parse().ok()?, max))
}

fn next_start(path: &mut MoveToBlock, m: &mut Mob) -> bool {
    if path.next_start > 0 {
        path.next_start -= 1;
        return false;
    }
    path.next_start = super::reduced(200 + m.random.next_int_bound(200).unwrap_or(0));
    true
}

impl Goal for Pollination {
    goal_clone!();

    fn flags(&self) -> u8 { MOVE | JUMP }

    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        m.ai.nav.__horizon_mixins_naturalist_butterfly_nectar_mut().present == self.grow
            && next_start(&mut self.path, m)
            && self.path.find_nearest(m, cx.w, |w, pos| valid_pollination(self.grow, w, pos))
    }

    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        m.ai.nav.__horizon_mixins_naturalist_butterfly_nectar_mut().present == self.grow
            && self.path.can_continue(cx.w, |w, pos| valid_pollination(self.grow, w, pos))
    }

    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.path.start(m, cx.w);
        self.waited = 0;
    }

    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        if self.grow {
            self.grow_crop(m, cx);
            return;
        }
        self.path.tick(m, cx.w);
        if self.path.reached {
            if self.waited >= 40 && valid_pollination(false, cx.w, self.path.block) {
                m.ai.nav.__horizon_mixins_naturalist_butterfly_nectar_mut().present = true;
            } else {
                self.waited += 1;
            }
        }
    }
}

impl Pollination {
    fn grow_crop(&mut self, m: &mut Mob, cx: &mut Cx) {
        let [x, y, z] = self.path.block;
        let target = [f64::from(x) + 0.5, f64::from(y) + 1.5, f64::from(z) + 0.5];
        if m.distance_sqr(Vec3::new(target[0], target[1], target[2])) >= 1.0 {
            self.path.try_ticks += 1;
            if self.path.try_ticks % 40 == 0 {
                m.nav_move_to(cx.w, [target[0], target[1] - 0.5, target[2]], self.path.speed);
            }
        } else if let Some((age, max)) = crop_age(cx.w, self.path.block) {
            let mut growth = 2 + cx.lr.next_int_bound(4).unwrap_or(0);
            if max == 3 || max == 2 {
                growth /= 3;
            }
            let age = (age + growth).min(max);
            let state = if max == 2 && age >= 2 {
                cx.w.registry.default_state("minecraft:torchflower")
            } else {
                let old = cx.w.world.block(x, y, z);
                state_property(cx.w.registry, old, "age", &age.to_string())
            };
            if let Some(state) = state {
                cx.events.push(super::super::MobEvent::SetBlock { pos: self.path.block, state });
            }
            m.ai.nav.__horizon_mixins_naturalist_butterfly_nectar_mut().crops += 1;
        }
    }
}

pub fn state_property(registry: &BlockRegistry, state: BlockStateId, key: &str, value: &str) -> Option<BlockStateId> {
    let mut row = registry.state(state)?.clone();
    row.properties.insert(key.to_owned(), value.to_owned());
    registry.id(&row)
}

const HORIZONTAL: [(&str, [i32; 2]); 4] = [
    ("north", [0, -1]), ("east", [1, 0]), ("south", [0, 1]), ("west", [-1, 0]),
];

fn logs(w: &MobWorld, [x, y, z]: [i32; 3]) -> Option<(&'static str, [i32; 3])> {
    if !w.facts.is_air(w.world.block(x, y, z)) {
        return None;
    }
    HORIZONTAL.into_iter().find_map(|(name, [dx, dz])| {
        let at = [x + dx, y, z + dz];
        (w.has_tag("minecraft:logs", w.block_name(at[0], at[1], at[2]))
            && w.has_tag("minecraft:logs", w.block_name(at[0], at[1] - 1, at[2])))
            .then_some((name, at))
    })
}

#[derive(Clone, Debug)]
pub struct Cocoon(MoveToBlock);

impl Default for Cocoon {
    fn default() -> Self {
        Self(MoveToBlock::new(1.0, 5, 2))
    }
}

impl Goal for Cocoon {
    goal_clone!();

    fn flags(&self) -> u8 { MOVE | JUMP }

    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        !m.is_baby() && next_start(&mut self.0, m) && self.0.find_nearest(m, cx.w, |w, p| logs(w, p).is_some())
    }

    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(cx.w, |w, p| logs(w, p).is_some())
    }

    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some((_, log)) = logs(cx.w, self.0.block) {
            let shell = std::mem::replace(&mut self.0.block, log);
            self.0.start(m, cx.w);
            self.0.block = shell;
        }
    }

    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some((_, log)) = logs(cx.w, self.0.block) {
            let shell = std::mem::replace(&mut self.0.block, log);
            self.0.tick(m, cx.w);
            self.0.block = shell;
            m.look_at_speed([f64::from(log[0]) + 0.5, f64::from(log[1] + 1), f64::from(log[2]) + 0.5],
                10.0, m.max_head_x_rot());
        }
        let p = block_of(m);
        let Some((facing, _)) = logs(cx.w, p) else { return };
        let Some(base) = cx.w.registry.default_state("naturalist:chrysalis") else { return };
        let Some(state) = state_property(cx.w.registry, base, "facing", facing) else { return };
        m.removed = true;
        cx.events.push(super::super::MobEvent::SetBlock { pos: p, state });
        cx.events.push(super::super::MobEvent::Sound(
            "minecraft:block.grass.place",
            p.map(|i| f64::from(i) + 0.5),
            0.7,
            0.9 + cx.lr.next_float() * 0.2,
        ));
    }
}

fn block_of(m: &Mob) -> [i32; 3] {
    [m.position.x, m.position.y, m.position.z].map(|v| v.floor() as i32)
}
