fn snail_register(mob: &mut Mob) {
    if mob.kind.id == "naturalist:snail" {
        // The Cow entity template has the right ageable mechanics. Replace its
        // food before the authoritative entity or its client proxy can use it.
        if mob.kind.food.as_ref() != ["minecraft:beetroot"] {
            std::sync::Arc::make_mut(&mut mob.kind).food =
                vec!["minecraft:beetroot".to_owned()].into();
        }
        super::goals::naturalist_snail_goals::register(mob);
    }
}
