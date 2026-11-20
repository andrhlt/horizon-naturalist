fn aerial_living_drag(mob: &mut Mob) {
    if mob.kind.flags.omnidirectional
        && matches!(
            mob.kind.id.as_str(),
            "naturalist:bird"
                | "naturalist:butterfly"
                | "naturalist:dragonfly"
                | "naturalist:firefly"
                | "naturalist:vulture"
        )
    {
        std::sync::Arc::make_mut(&mut mob.kind)
            .flags
            .omnidirectional = false;
    }
}
