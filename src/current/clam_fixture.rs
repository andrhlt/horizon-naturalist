fn clam_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_core::nbt::Tag;
    use std::ops::ControlFlow;
    let Some(query) = command.strip_prefix("naturalist_fixture_clam ") else {
        return ControlFlow::Continue(());
    };
    let Some(mob) = game.mobs.iter().find(|mob| mob.kind.id == "naturalist:clam" && mob.alive()) else {
        game.feedback(Text::literal("Clam fixture missing Clam"));
        return ControlFlow::Break(true);
    };
    let raw = game.persist.raw.get(&mob.id);
    let number = |name: &str| -> i32 {
        raw.and_then(|tag| tag.get(name)).and_then(|tag| match tag {
            Tag::Int(value) => Some(*value),
            Tag::Short(value) => Some(i32::from(*value)),
            Tag::Byte(value) => Some(i32::from(*value)),
            _ => None,
        }).unwrap_or(0)
    };
    let boolean = |name: &str| -> bool { number(name) != 0 };
    let status = serde_json::json!({
        "open": boolean("NaturalistClamOpen"),
        "treasure": mob.extra.main_hand_stack().is_some(),
        "timer": number("NaturalistClamTimer"),
        "cooldown": number("NaturalistClamCooldown"),
        "delay": number("NaturalistClamLaunchDelay"),
        "pushable": mob.pushable(),
        "stored_persistent": mob.persistent || horizon_client::mobs::ai::requires_custom_persistence(mob),
        "state_fields_implemented": raw.is_some_and(|tag| tag.get("NaturalistClamOpen").is_some()),
        "query": query,
    });
    game.feedback(Text::literal(format!("Clam fixture {status}")));
    ControlFlow::Break(true)
}
