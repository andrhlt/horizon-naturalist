fn piranha_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Damage, DamageKind, Species, Who};
    use std::ops::ControlFlow;
    let mut args = command.split_ascii_whitespace();
    if args.next() != Some("naturalist_fixture_piranha") {
        return ControlFlow::Continue(());
    }
    let by_tag = |game: &Game<'_>, tag: &str| game.mobs.iter()
        .find(|mob| game.tags_of(Target::Mob(mob.id)).iter().any(|value| value == tag))
        .map(|mob| mob.id).unwrap();
    let output = match args.next() {
        Some("start") => {
            let id = by_tag(game, args.next().unwrap());
            for mob in &mut game.mobs {
                if mob.species() == Species::Cod {
                    mob.ai.goals.set_start_delay("FollowFlockLeaderGoal", if mob.id == id { 0 } else { 1000 });
                }
            }
            "ready".to_owned()
        }
        Some("hurt") => {
            let victim = by_tag(game, args.next().unwrap());
            let attacker = by_tag(game, args.next().unwrap());
            let source = Damage { kind: DamageKind::MobAttack, attacker: Some(Who::Mob(attacker)),
                from: None, attacker_species: None };
            game.mob_attacks_mob(victim, 1.0, source).to_string()
        }
        Some("set_target") => {
            let id = by_tag(game, args.next().unwrap());
            let target = args.next().map(|tag| Who::Mob(by_tag(game, tag)));
            game.mobs.iter_mut().find(|mob| mob.id == id).unwrap().ai.target = target;
            "set".to_owned()
        }
        Some("inspect") => {
            let id = by_tag(game, args.next().unwrap());
            let mob = game.mobs.iter().find(|mob| mob.id == id).unwrap();
            let target = match mob.ai.target { Some(Who::Mob(id)) => id, Some(Who::Player) => u32::MAX, None => 0 };
            format!("{} {} {} {}", id, mob.extra.leader.unwrap_or(0), mob.extra.followers, target)
        }
        Some("spawning") => game.naturalist_fixture_piranha_spawn(),
        _ => "invalid operation".to_owned(),
    };
    game.feedback(Text::literal(format!("Piranha fixture {output}")));
    ControlFlow::Break(true)
}
