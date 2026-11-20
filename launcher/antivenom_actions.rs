use super::Game;
use horizon_client::mobs::Mob;
use horizon_core::geometry::Vec3;
use horizon_world::attachments::AttachmentType;
use std::cell::Cell;

thread_local! {
    static CLOUD_ACTIVE: Cell<bool> = const { Cell::new(false) };
}

pub(super) struct CloudScope;

impl CloudScope {
    pub(super) fn enter() -> Option<Self> {
        (!CLOUD_ACTIVE.replace(true)).then(|| Self)
    }
}

impl Drop for CloudScope {
    fn drop(&mut self) {
        CLOUD_ACTIVE.set(false);
    }
}

pub(super) const ID: &str = "naturalist:antivenom";
const POISON: &str = "minecraft:poison";

// Splash hits from a guest's turn are applied to nearby players after the hub is restored.
pub(super) const PENDING_SPLASHES: AttachmentType<Vec<(Vec3, u32, u32)>> = AttachmentType {
    namespace: "naturalist",
    name: "pending_antivenom_splashes",
    default: Vec::new,
    codec: None,
    copy_on_death: false,
    sync: false,
};

pub(super) fn clear_player(game: &mut Game<'_>) {
    let list = &mut game.potions.effects.list;
    let before = list.len();
    list.retain(|effect| effect.id != "poison");
    if list.len() != before {
        game.apply_effect_status();
    }
}

pub(super) fn clear_mob(mob: &mut Mob) {
    clear_effect(mob, POISON);
}

pub(super) fn clear_effect(mob: &mut Mob, id: &str) {
    mob.extra.effects.retain(|effect| effect.0 != id);
    mob.extra.hidden_effects.retain(|effect| *effect != id);
    mob.extra.ambient_effects.retain(|effect| *effect != id);
    mob.extra.effect_fallbacks.retain(|effect| effect.id != id);
}
