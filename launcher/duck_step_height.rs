fn duck_step_height(
    game: &Game<'_>,
    command: &str,
    context: &Context,
    target: Target,
    attribute: &str,
) -> std::ops::ControlFlow<R<Option<(i32, Text)>>> {
    use std::ops::ControlFlow;

    // This pinned Horizon version has the physical step height but does not
    // register attributes for native mob types in its generic command map.
    if !matches!(command, "attribute.get" | "attribute.base.get") {
        return ControlFlow::Continue(());
    }
    let Target::Mob(id) = target else { return ControlFlow::Continue(()) };
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == id) else {
        return ControlFlow::Continue(());
    };
    if !mob.kind.id.starts_with("naturalist:") {
        return ControlFlow::Continue(());
    }
    let value = match attribute {
        "minecraft:step_height" => mob.kind.step_height,
        "minecraft:attack_knockback" => mob.attrs.attack_knockback,
        "minecraft:movement_speed"
            if command == "attribute.get"
                && mob.extra.sprinting
                && matches!(mob.kind.id.as_str(), "naturalist:elephant" | "naturalist:mammoth") =>
        {
            // Horizon already syncs this transient modifier to tracking clients.
            finalize::value_with(
                mob,
                finalize::Attr::MovementSpeed,
                &[(f64::from(0.3f32), finalize::Op::AddMultipliedTotal)],
            )
        },
        _ => return ControlFlow::Continue(()),
    };
    let scale = match context.get("scale") {
        Some(Arg::Double(scale)) => *scale,
        _ => 1.0,
    };
    let message = if command == "attribute.base.get" {
        "commands.attribute.base_value.get.success"
    } else {
        "commands.attribute.value.get.success"
    };
    ControlFlow::Break(Ok(Some(((value * scale) as i32, Text::translate(message, &[
        Game::attribute_description(attribute), game.target_name(target), java_double_text(value),
    ])))))
}
