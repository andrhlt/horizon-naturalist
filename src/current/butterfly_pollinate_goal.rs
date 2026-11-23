use horizon_client::mobs::goals::{Cx, Goal};
use horizon_client::mobs::{Mob, MobEvent};
use horizon_world::registry::BlockStateId;

#[derive(Clone, Debug, Default)]
struct Pollinate {
    flower: Option<[i32; 3]>,
    nectar: bool,
    pollinate_ticks: i32,
    crops: i32,
    grow_cooldown: i32,
}

impl Goal for Pollinate {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { 0 }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if self.nectar { return self.crops < 10; }
        if self.flower.is_some_and(|p| is_flower(cx.w, p)) { return true; }
        self.flower = find_flower(m, cx);
        self.flower.is_some()
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.nectar && self.crops < 10 || self.flower.is_some_and(|p| is_flower(cx.w, p))
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        if !self.nectar {
            self.pollinate_ticks += 1;
            if self.pollinate_ticks >= 41 { self.nectar = true; }
            return;
        }
        if self.grow_cooldown > 0 { self.grow_cooldown -= 1; return; }
        self.grow_cooldown = 30;
        if let Some((pos, state)) = find_crop(m, cx) {
            cx.events.push(MobEvent::LevelEvent { id: 2011, pos, data: 15 });
            cx.events.push(MobEvent::SetBlock { pos, state });
            self.crops += 1;
            if self.crops >= 10 { self.nectar = false; self.pollinate_ticks = 0; }
        }
    }
}

fn find_flower(m: &Mob, cx: &Cx) -> Option<[i32; 3]> {
    let [x, y, z] = block_pos(m);
    for dy in -2..=2 { for dx in -6..=6 { for dz in -6..=6 {
        let p = [x + dx, y + dy, z + dz];
        if is_flower(cx.w, p) { return Some(p); }
    }}}
    None
}

fn is_flower(w: &horizon_client::mobs::MobWorld, p: [i32; 3]) -> bool {
    w.has_tag("minecraft:flowers", w.block_name(p[0], p[1], p[2]))
}

fn find_crop(m: &Mob, cx: &Cx) -> Option<([i32; 3], BlockStateId)> {
    let [x, y, z] = block_pos(m);
    for dy in -4..=0 { for dx in -16..=16 { for dz in -16..=16 {
        let p = [x + dx, y + dy, z + dz];
        let name = cx.w.block_name(p[0], p[1], p[2]);
        if !cx.w.has_tag("minecraft:bee_growables", name) { continue; }
        let Some(mut state) = cx.w.registry.state(cx.w.world.block(p[0], p[1], p[2])).cloned() else { continue; };
        let (key, max) = match name {
            "minecraft:beetroots" => ("age", 3),
            "minecraft:wheat" | "minecraft:carrots" | "minecraft:potatoes" => ("age", 7),
            "minecraft:melon_stem" | "minecraft:pumpkin_stem" => ("age", 7),
            "minecraft:sweet_berry_bush" => ("age", 3),
            _ => continue,
        };
        let age = state.properties.get(key)?.parse::<i32>().ok()?;
        if age >= max { continue; }
        state.properties.insert(key.into(), (age + 1).to_string());
        if let Some(id) = cx.w.registry.id(&state) { return Some((p, id)); }
    }}}
    None
}

fn block_pos(m: &Mob) -> [i32; 3] {
    [m.position.x.floor() as i32, m.position.y.floor() as i32, m.position.z.floor() as i32]
}

pub fn install(m: &mut Mob) {
    if m.kind.id == "naturalist:butterfly" && !m.ai.goals.has("Pollinate") {
        m.ai.goals.add(4, Pollinate::default());
    }
}
