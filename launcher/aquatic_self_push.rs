fn aquatic_self_push(ordinary: bool, mob: &Mob) -> bool {
    ordinary
        && !matches!(
            mob.kind.id.as_str(),
            "naturalist:great_white_shark" | "naturalist:whale" | "naturalist:starfish" | "naturalist:clam"
        )
}
