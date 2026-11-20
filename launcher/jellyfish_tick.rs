fn jellyfish_tick(kept: bool, mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) -> bool {
    use super::nav::naturalist_jellyfish_state as jellyfish;
    if !kept || !mob.alive() || mob.kind.id != "naturalist:jellyfish" {
        return kept;
    }
    let hitbox = mob.bounds().inflate(Vec3::new(0.2, 0.2, 0.2));
    let from = mob.position.components();
    if world.player.as_ref().is_some_and(|player| player.alive && player.bounds.intersects(hitbox)) {
        events.push(MobEvent::DamagePlayer { amount: 2.0, from, kind: "minecraft:mob_attack" });
    }
    for other in world.others_near(hitbox) {
        if other.id != mob.id && other.alive && other.bounds.intersects(hitbox)
            && !other.effects.contains(&"naturalist:whale")
            && !(other.species == Species::Cod && other.variant == jellyfish::SNAPSHOT)
        {
            events.push(MobEvent::DamageMob { target: other.id, amount: 2.0,
                source: Damage::mob(mob.id, from) });
        }
    }

    let cooldown = (jellyfish::pulse_cooldown(&mob.ai.nav) - 1).max(0);
    if cooldown == 0 && mob.fluids.in_water && mob.ai.mov.has_wanted() {
        let [x, y, z] = mob.ai.mov.wanted;
        let delta = Vec3::new(x, y, z).subtract(mob.position);
        if delta.length_squared() > 1e-4 {
            mob.velocity = mob.velocity.add_vector(delta.normalize().scale(0.18));
        }
        jellyfish::set_pulse_cooldown(&mut mob.ai.nav, jellyfish::PULSE_INTERVAL);
    } else {
        jellyfish::set_pulse_cooldown(&mut mob.ai.nav, cooldown);
    }
    kept
}
