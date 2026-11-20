fn deer_social_breed(goal: &mut Breed, mob: &Mob, context: &Cx<'_, '_>) -> std::ops::ControlFlow<bool> {
    use super::naturalist_deer_social::{ZEBRA_MARKER, same_farm_template_animal};
    use std::ops::ControlFlow;

    if mob.species() == Species::Wolf {
        let hedgehog = mob.kind.id == super::naturalist_hedgehog::KIND;
        let ordinary_wolf = mob.kind.id == "minecraft:wolf";
        if !hedgehog && (!ordinary_wolf || !context.w.nearby(mob.position, 8.0, |other| {
            other.effects.contains(&super::naturalist_hedgehog::KIND)
        }).any(|other| mob.distance_sqr(other.position) <= 64.0)) {
            return ControlFlow::Continue(());
        }
        if !mob.extra.tame || mob.extra.in_love <= 0 {
            return ControlFlow::Break(false);
        }
        goal.partner = context.w.nearby(mob.position, 8.0, |other| {
            let other_hedgehog = other.effects.contains(&super::naturalist_hedgehog::KIND);
            other.id != mob.id && other.species == Species::Wolf
                && other_hedgehog == hedgehog && !other.effects.contains(&super::naturalist_ant::ANT)
                && other.tame && other.in_love && !other.baby && !other.panicking
                && (hedgehog || !other.sitting)
        })
        .find(|other| mob.distance_sqr(other.position) <= 64.0)
        .map(|other| other.id);
        return ControlFlow::Break(goal.partner.is_some());
    }
    if matches!(mob.species(), Species::Horse | Species::Donkey) {
        let ostrich = mob.kind.id == super::naturalist_ostrich::KIND;
        if ostrich {
            if mob.extra.flag || mob.extra.in_love <= 0 {
                return ControlFlow::Break(false);
            }
            goal.partner = context.w.nearby(mob.position, 8.0, |other| {
                other.id != mob.id && other.in_love && !other.baby && !other.panicking
                    && other.effects.contains(&super::naturalist_ostrich::KIND)
            }).find(|other| mob.distance_sqr(other.position) <= 64.0).map(|other| other.id);
            return ControlFlow::Break(goal.partner.is_some());
        }
        let zebra = mob.kind.id == super::naturalist_zebra::KIND;
        if !zebra && !context.w.nearby(mob.position, 8.0, |other| {
            other.species == Species::Donkey && other.variant == ZEBRA_MARKER
        }).any(|other| mob.distance_sqr(other.position) <= 64.0) {
            return ControlFlow::Continue(());
        }
        if !mob.extra.tame || mob.is_baby() || mob.extra.in_love <= 0
            || mob.rider.is_some() || mob.extra.rider.is_some() || mob.extra.mount.is_some()
            || mob.health < mob.attrs.max_health
        {
            return ControlFlow::Break(false);
        }
        goal.partner = context.w.nearby(mob.position, 8.0, |other| {
            other.id != mob.id && matches!(other.species, Species::Donkey | Species::Horse)
                && zebra == (other.species == Species::Donkey && other.variant == ZEBRA_MARKER)
                && other.tame && !other.baby && other.in_love && !other.ridden
                && other.health >= other.stats[0] as f32 && !other.panicking
        }).find(|other| mob.distance_sqr(other.position) <= 64.0).map(|other| other.id);
        return ControlFlow::Break(goal.partner.is_some());
    }
    if !matches!(mob.species(), Species::Cow | Species::Chicken | Species::Pig) {
        return ControlFlow::Continue(());
    }
    let giraffe = mob.kind.id == "naturalist:giraffe";
    if giraffe && (mob.is_baby() || mob.rider.is_some() || mob.extra.rider.is_some() || mob.extra.mount.is_some()) {
        return ControlFlow::Break(false);
    }
    let requires_tame = matches!(mob.kind.id.as_str(), "naturalist:capybara" | "naturalist:tortoise");
    if (requires_tame && !mob.extra.tame) || mob.extra.in_love <= 0 {
        return ControlFlow::Break(false);
    }
    goal.partner = context
        .w
        .nearby(mob.position, 8.0, |other| {
            other.id != mob.id
                && same_farm_template_animal(mob, other)
                && (!giraffe || !other.ridden && !other.passenger)
                && (!requires_tame || other.tame)
                && other.in_love
                && !other.baby
                && !other.panicking
        })
        .find(|other| mob.distance_sqr(other.position) <= 64.0)
        .map(|other| other.id);
    ControlFlow::Break(goal.partner.is_some())
}
