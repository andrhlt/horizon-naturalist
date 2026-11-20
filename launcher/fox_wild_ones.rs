use crate::mobs::{Mob, finalize};
use horizon_core::random::RandomSource;

pub const DISC: &str = "naturalist:music_disc_wild_ones";

pub fn equip(mob: &mut Mob) -> bool {
    if mob.kind.id != "minecraft:fox" || mob.is_baby() || mob.random.next_float() >= 0.02 {
        return false;
    }
    finalize::set_item(mob, finalize::MAINHAND, DISC);
    finalize::set_drop_chance(mob, finalize::MAINHAND, 2.0);
    mob.extra.pick_up_loot = false;
    true
}

thread_local! {
    static PLAYER_EGG: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub struct PlayerEgg;
impl PlayerEgg {
    pub fn enter() -> Option<Self> {
        (!PLAYER_EGG.replace(true)).then(|| Self)
    }

    pub fn active() -> bool {
        PLAYER_EGG.get()
    }
}
impl Drop for PlayerEgg {
    fn drop(&mut self) {
        PLAYER_EGG.set(false);
    }
}
