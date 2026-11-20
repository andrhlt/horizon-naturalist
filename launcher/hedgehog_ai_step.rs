fn hedgehog_ai_step(
    mob: &mut Mob,
    world: &MobWorld<'_>,
    events: &mut Vec<MobEvent>,
) -> std::ops::ControlFlow<()> {
    use super::naturalist_hedgehog as hedgehog;
    use std::ops::ControlFlow;
    if mob.kind.id != hedgehog::KIND {
        return ControlFlow::Continue(());
    }
    let hiding = hedgehog::can_hide(mob, world);
    let previous = hedgehog::state(mob).hiding;
    if hiding != previous && !mob.silent {
        let sound = if hiding { "naturalist:entity.hedgehog.hide" }
                    else { "naturalist:entity.hedgehog.unhide" };
        events.push(MobEvent::Sound(sound, mob.position.components(), 0.6, 1.0));
    }
    let pose = hedgehog::state_mut(mob);
    pose.hiding = hiding;
    pose.following_cooldown = pose.following_cooldown.saturating_sub(1);
    if !pose.rolling {
        mob.no_gravity = false;
    }
    ControlFlow::Break(())
}
