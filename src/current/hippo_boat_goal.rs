use horizon_client::mobs::{
    Mob,
    goals::{Cx, Goal, LOOK, MOVE, reduced},
};
use horizon_core::geometry::Vec3;
use std::sync::{LazyLock, Mutex, RwLock};

#[derive(Clone, Copy, Debug)]
struct BoatView {
    id: u32,
    position: Vec3,
}

static BOATS: LazyLock<RwLock<Vec<BoatView>>> = LazyLock::new(|| RwLock::new(Vec::new()));
static ATTACKS: LazyLock<Mutex<Vec<(u32, u32)>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub(crate) fn refresh(boats: Vec<(u32, Vec3)>) {
    if let Ok(mut current) = BOATS.write() {
        *current = boats
            .into_iter()
            .map(|(id, position)| BoatView { id, position })
            .collect();
    }
}

pub(crate) fn take_attacks(mob: u32) -> Vec<u32> {
    let Ok(mut attacks) = ATTACKS.lock() else {
        return Vec::new();
    };
    let mut selected = Vec::new();
    attacks.retain(|(attacker, boat)| {
        if *attacker == mob {
            selected.push(*boat);
            false
        } else {
            true
        }
    });
    selected
}

#[derive(Clone, Debug, Default)]
pub(crate) struct HippoBoatAttack {
    target: Option<u32>,
    attack_delay: i32,
}

impl Goal for HippoBoatAttack {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        "NaturalistHippoBoatAttack"
    }
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }

    fn can_use(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        if mob.is_baby() {
            return false;
        }
        let Ok(boats) = BOATS.read() else {
            return false;
        };
        self.target = boats
            .iter()
            .filter(|boat| mob.distance_sqr(boat.position) <= 256.0)
            .min_by(|a, b| {
                mob.distance_sqr(a.position)
                    .total_cmp(&mob.distance_sqr(b.position))
            })
            .map(|boat| boat.id);
        self.target.is_some()
    }

    fn can_continue(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        if mob.is_baby() {
            return false;
        }
        let Some(target) = self.target else {
            return false;
        };
        boat(target).is_some_and(|position| mob.distance_sqr(position) <= 256.0)
    }

    fn start(&mut self, _: &mut Mob, _: &mut Cx) {
        self.attack_delay = 0;
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        self.target = None;
        mob.nav_stop();
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(id) = self.target else {
            return;
        };
        let Some(position) = boat(id) else {
            self.target = None;
            return;
        };
        mob.look_at_speed(
            [position.x, position.y, position.z],
            10.0,
            mob.max_head_x_rot(),
        );
        if mob.distance_sqr(position) > 4.0 {
            mob.nav_move_to(cx.w, [position.x, position.y, position.z], 1.0);
        } else if self.attack_delay <= 0 {
            if let Ok(mut attacks) = ATTACKS.lock() {
                attacks.push((mob.id, id));
            }
            self.attack_delay = reduced(20);
        }
        self.attack_delay -= 1;
    }
}

fn boat(id: u32) -> Option<Vec3> {
    BOATS
        .read()
        .ok()?
        .iter()
        .find(|boat| boat.id == id)
        .map(|boat| boat.position)
}
