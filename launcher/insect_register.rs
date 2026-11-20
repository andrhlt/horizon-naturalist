fn insect_register(mob: &mut Mob) {
    use super::goals::{Float, Stroll, naturalist_insect_goals};
    match mob.kind.id.as_str() {
        "naturalist:butterfly" => naturalist_insect_goals::register_butterfly(mob),
        "naturalist:caterpillar" => {
            mob.ai.nav.kind = super::nav::NavKind::WallClimber;
            mob.ai.goals.clear();
            mob.ai.goals.add(0, Float);
            mob.ai
                .goals
                .add(1, naturalist_insect_goals::Cocoon::default());
            mob.ai.goals.add(2, Stroll::avoiding_water(1.0));
        }
        _ => {}
    }
}
