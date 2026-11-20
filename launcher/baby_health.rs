fn baby_health(mob: &mut Mob) {
    use super::finalize::{Attr, Op, add_modifier, modifier_amount, remove_modifier};
    use super::nav::naturalist_baby_health::{MODIFIER_ID, last_age};

    const EPSILON: f64 = 1.0e-4;
    let Some(species) = mob.kind.id.strip_prefix("naturalist:") else { return };
    if matches!(species, "carried_food" | "dirt_trail" | "duck_egg" | "lizard_tail") {
        return;
    }
    let baby = mob.is_baby();
    let previous = last_age(&mut mob.ai.nav);
    if *previous == Some(baby) {
        return;
    }
    *previous = Some(baby);
    let previous_modifier = modifier_amount(mob, Attr::MaxHealth, MODIFIER_ID);
    if baby {
        let adult = f64::from(mob.attrs.max_health) / (1.0 + previous_modifier.unwrap_or(0.0));
        if adult <= 0.0 {
            return;
        }
        let raw = (1.70 * adult.sqrt()).min(adult * 0.75);
        let stepped = (raw / 2.0).round() * 2.0;
        let ceiling = (((adult - EPSILON) / 2.0).floor() * 2.0).max(2.0);
        let amount = stepped.clamp(2.0, ceiling) / adult - 1.0;
        if previous_modifier.is_some_and(|old| (old - amount).abs() < EPSILON) {
            return;
        }
        add_modifier(mob, Attr::MaxHealth, MODIFIER_ID, amount, Op::AddMultipliedTotal);
        mob.health = mob.health.min(mob.attrs.max_health);
    } else if previous_modifier.is_some() {
        let fraction = if mob.attrs.max_health > 0.0 { mob.health / mob.attrs.max_health } else { 1.0 };
        remove_modifier(mob, Attr::MaxHealth, MODIFIER_ID);
        mob.health = mob.attrs.max_health * fraction;
    }
}
