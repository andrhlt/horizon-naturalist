use horizon_client::mobs::goals::{Cx, Goal};
use horizon_client::mobs::{Mob, MobEvent};

#[derive(Clone, Debug, Default)]
struct Cocoon {
    facing: Option<&'static str>,
}

impl Goal for Cocoon {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { 0 }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.age < 0 { return false; }
        let [x, y, z] = block_pos(m);
        self.facing = [
            ("north", [0, 0, -1]),
            ("south", [0, 0, 1]),
            ("west", [-1, 0, 0]),
            ("east", [1, 0, 0]),
        ].into_iter().find_map(|(facing, [dx, dy, dz])| {
            let log = [x + dx, y + dy, z + dz];
            let below = [log[0], log[1] - 1, log[2]];
            (matches!(cx.w.block_name(x, y, z), "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air")
                && is_log(cx.w.block_name(log[0], log[1], log[2]))
                && is_log(cx.w.block_name(below[0], below[1], below[2])))
                .then_some(facing)
        });
        self.facing.is_some()
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool { self.can_use(m, cx) }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(facing) = self.facing.take() else { return; };
        let [x, y, z] = block_pos(m);
        let Some(mut state) = cx.w.registry.default_state("naturalist:chrysalis")
            .and_then(|id| cx.w.registry.state(id).cloned()) else { return; };
        state.properties.insert("facing".into(), facing.into());
        let Some(state) = cx.w.registry.id(&state) else { return; };
        cx.events.push(MobEvent::SetBlock { pos: [x, y, z], state });
        // Caterpillar.discard() in CocoonGoal is a silent removal: it must not
        // run the killed/death path (drops, XP, or death hooks). The Mob is
        // the live row passed to this goal, so mark it removed directly.
        m.removed = true;
    }
}

fn block_pos(m: &Mob) -> [i32; 3] {
    [m.position.x.floor() as i32, m.position.y.floor() as i32, m.position.z.floor() as i32]
}

fn is_log(name: &str) -> bool {
    name.ends_with("_log") || name.ends_with("_wood") || name.ends_with("_stem")
        || name.ends_with("_hyphae")
}

pub fn install(m: &mut Mob) {
    if m.kind.id == "naturalist:caterpillar"
        && !m.ai.goals.has("Cocoon") {
        m.ai.goals.add(1, Cocoon::default());
    }
}
