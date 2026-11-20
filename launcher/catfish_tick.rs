fn catfish_tick(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    use super::nav::naturalist_catfish_state as catfish;
    if mob.kind.id != "naturalist:catfish" || mob.removed || !mob.alive() {
        return;
    }
    let countdown = catfish::cooldown(&mob.ai.nav);
    if countdown > 0 {
        catfish::set_cooldown(&mut mob.ai.nav, countdown - 1);
    }
    let Some((target, ticks)) = catfish::prey(&mob.ai.nav) else { return };
    if !world.seen(target).is_some_and(|prey| prey.alive) {
        catfish::set_prey(&mut mob.ai.nav, None);
        return;
    }
    if ticks <= 0 {
        return;
    }
    catfish::set_prey(&mut mob.ai.nav, Some((target, ticks - 1)));
    if ticks == 1 {
        events.push(MobEvent::AttackMob {
            target,
            amount: 1.0,
            source: Damage { kind: DamageKind::MobAttack, attacker: Some(Who::Mob(mob.id)),
                from: Some(mob.position.components()), attacker_species: Some(Species::Cod) },
        });
    }
}
