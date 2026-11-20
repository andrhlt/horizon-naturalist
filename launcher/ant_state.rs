use super::Mob;

pub const ANT: &str = "naturalist:ant";
pub const HILL: &str = "naturalist:ant_hill";
pub const ANGER_TICKS: i32 = 500;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Enter([i32; 3]),
    Store([i32; 3]),
    PickUp(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct FoodTarget {
    pub id: u32,
    pub at: [f64; 3],
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub cooldown: i32,
    pub meal: Option<u32>,
    pub anger_actor: Option<[i32; 4]>,
    pub food: Option<FoodTarget>,
    pub pursuing_food: Option<u32>,
    pub action: Option<Action>,
}

pub fn state(mob: &Mob) -> &State {
    mob.ai.nav.__horizon_mixins_naturalist_ant_state()
}

pub fn state_mut(mob: &mut Mob) -> &mut State {
    mob.ai.nav.__horizon_mixins_naturalist_ant_state_mut()
}

pub fn set_cooldown(mob: &mut Mob, ticks: i32) {
    state_mut(mob).cooldown = ticks;
}

pub fn start_cooldown(mob: &mut Mob) {
    use horizon_core::random::RandomSource;
    let next = 400
        + mob
            .random
            .next_int_bound(800)
            .expect("positive duration range");
    set_cooldown(mob, next);
}
