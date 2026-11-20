fn ray_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    use horizon_client_lib::mobs::Who;
    let Some(tag) = command.strip_prefix("naturalist_fixture_ray ") else {
        return ControlFlow::Continue(());
    };
    let mob = game.mobs.iter().find(|mob| {
        game.tags_of(Target::Mob(mob.id)).iter().any(|candidate| candidate == tag)
    }).unwrap();
    let text = match mob.kind.id.as_str() {
        "naturalist:ray" => {
            if let Some(Who::Mob(id)) = mob.ai.target {
                game.tags_of(Target::Mob(id)).into_iter().next().unwrap_or_default()
            } else {
                "no target".to_owned()
            }
        }
        _ => {
            let poison = mob.extra.effects.iter().find(|effect| effect.0 == "minecraft:poison");
            let (duration, amplifier) = poison.map(|(_, ticks, amp)| (*ticks, *amp)).unwrap_or((-1, 0));
            format!("{} {duration} {amplifier}", mob.health)
        }
    };
    game.feedback(Text::literal(format!("Ray fixture {text}")));
    ControlFlow::Break(true)
}
