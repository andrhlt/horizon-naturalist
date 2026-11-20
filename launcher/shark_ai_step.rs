fn shark_ai_step(
    kept: bool,
    mob: &mut Mob,
    world: &MobWorld<'_>,
    events: &mut Vec<MobEvent>,
) -> bool {
    use super::nav::{naturalist_catfish_state as hunt, naturalist_shark_state as shark};
    use super::nav::naturalist_whale_state as whale;
    if mob.species() == super::Species::Pufferfish {
        events.retain(|event| !matches!(event, MobEvent::PufferfishSting { mob: fish, target, .. }
            if *fish == mob.id && world.seen(*target).is_some_and(|seen| seen.effects.contains(&whale::KIND))));
    }
    if kept && mob.alive() && mob.kind.id == whale::KIND {
        whale::tick_body(mob, world, events);
        whale::tick_spray(mob, world, events);
        let cooldown = whale::motion_mut(&mut mob.ai.nav).flop_cooldown;
        let (cooldown, flopped) = shark::tick_flop(mob, world, cooldown);
        whale::motion_mut(&mut mob.ai.nav).flop_cooldown = cooldown;
        if flopped && !mob.silent {
            events.push(MobEvent::Sound("entity.guardian.flop", mob.position.components(), 2.0, 0.6));
        }
        return kept;
    }
    if !kept || !mob.alive() || mob.kind.id != shark::KIND {
        return kept;
    }
    let wet = mob.fluids.in_water;
    shark::motion_mut(&mut mob.ai.nav).tick(mob.body_yaw, mob.velocity, wet);
    let cooldown = hunt::cooldown(&mob.ai.nav);
    if cooldown > 0 {
        hunt::set_cooldown(&mut mob.ai.nav, cooldown - 1);
    }
    shark::move_parts(mob, world, events);
    let cooldown = shark::motion_mut(&mut mob.ai.nav).flop_cooldown;
    let (cooldown, _) = shark::tick_flop(mob, world, cooldown);
    shark::motion_mut(&mut mob.ai.nav).flop_cooldown = cooldown;
    kept
}
