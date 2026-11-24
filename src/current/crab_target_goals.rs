//! Scratch prototype of Java Crab's weapon-gated target goals.
//! Owner goal structure derives from Horizon's pinned tame.rs (LGPL-3.0).
use horizon_client::mobs::{
    Mob, MobWorld, Species, Who,
    goals::{Cx, Goal, HurtByTarget, TARGET},
};

fn owner<'a>(m: &Mob, world: &'a MobWorld<'_>) -> Option<&'a horizon_client::mobs::PlayerView> {
    let uuid = m.extra.owner?;
    if !m.extra.tame || !world.player_identity.is_some_and(|(player, _)| player == uuid) {
        return None;
    }
    world.selected_player()
}
fn armed(m: &Mob) -> bool {
    m.extra.main_hand_id().is_some()
}
fn permitted(m: &Mob, world: &MobWorld<'_>, target: Who) -> bool {
    match target {
        Who::Player => {
            // Who represents the player category for the selected/redirected
            // target view. The owner-hurt event itself supplies the attacker;
            // rejecting this category was the checkpoint's PvP gap.
            let Some(player) = world.selected_player() else {
                return false;
            };
            player.alive && !player.invulnerable && !player.creative && !player.spectator
        }
        Who::Mob(id) => {
            if id == m.id {
                return false;
            }
            let Some(seen) = world.seen(id) else {
                return false;
            };
            seen.alive && !seen.tame && !matches!(seen.species, Species::Creeper | Species::Ghast)
        }
    }
}
#[derive(Clone, Debug)]
struct OwnerTarget {
    time: i64,
    hurt_by: bool,
}
impl Goal for OwnerTarget {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        if self.hurt_by {
            "NaturalistCrabOwnerHurtBy"
        } else {
            "NaturalistCrabOwnerHurt"
        }
    }
    fn flags(&self) -> u8 {
        TARGET
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if !armed(m) || m.extra.sitting {
            return false;
        }
        let Some(p) = owner(m, cx.w) else {
            return false;
        };
        let event = if self.hurt_by { p.hurt_by } else { p.last_hurt };
        event.is_some_and(|(who, at)| at != self.time && permitted(m, cx.w, who))
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        m.ai.target
            .and_then(|t| cx.w.target_info(t))
            .is_some_and(|t| t.alive)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some(p) = owner(m, cx.w) {
            if let Some((who, time)) = if self.hurt_by { p.hurt_by } else { p.last_hurt } {
                m.set_ai_target(cx.w, Some(who));
                self.time = time;
            }
        }
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        m.set_ai_target(cx.w, None);
    }
}
#[derive(Clone, Debug)]
struct Retaliate {
    inner: HurtByTarget,
}
impl Goal for Retaliate {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        "NaturalistCrabRetaliate"
    }
    fn flags(&self) -> u8 {
        self.inner.flags()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if !armed(m) {
            return false;
        }
        if m.extra.tame && m.ai.last_hurt_by == Some(Who::Player) && owner(m, cx.w).is_some() {
            return false;
        }
        self.inner.can_use(m, cx)
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.inner.can_continue(m, cx)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.inner.start(m, cx);
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.inner.stop(m, cx);
    }
}
pub(super) fn install(m: &mut Mob) {
    use horizon_client::mobs::goals::{MeleeAttack, Panic};
    if m.ai.targets.has("NaturalistCrabRetaliate") {
        return;
    }
    m.ai.targets.clear();
    m.ai.targets.add(
        1,
        OwnerTarget {
            time: 0,
            hurt_by: true,
        },
    );
    m.ai.targets.add(
        2,
        OwnerTarget {
            time: 0,
            hurt_by: false,
        },
    );
    m.ai.targets.add(
        3,
        Retaliate {
            inner: HurtByTarget::new(),
        },
    );
    m.ai.goals.add(2, MeleeAttack::new(1.65, true));
    m.ai.goals.add(3, Panic::new(1.5));
}
