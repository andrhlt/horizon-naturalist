fn bass_school_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Damage, DamageKind, Species, Who};
    use horizon_core::random::RandomSource;
    use std::ops::ControlFlow;

    let mut args = command.split_ascii_whitespace();
    if args.next() != Some("naturalist_fixture_bass_school") {
        return ControlFlow::Continue(());
    }
    let by_tag = |game: &Game<'_>, tag: &str| {
        game.mobs.iter().find(|mob| {
            game.tags_of(Target::Mob(mob.id)).iter().any(|value| value == tag)
        }).map(|mob| mob.id)
    };
    let output = match args.next() {
        Some("start") => {
            let id = by_tag(game, args.next().unwrap()).unwrap();
            for mob in &mut game.mobs {
                if mob.species() == Species::Cod {
                    mob.ai.goals.set_start_delay("FollowFlockLeaderGoal", if mob.id == id { 0 } else { 1000 });
                }
            }
            "ready".to_owned()
        }
        Some("attach") => {
            let follower = by_tag(game, args.next().unwrap()).unwrap();
            let leader = by_tag(game, args.next().unwrap()).unwrap();
            for mob in &mut game.mobs {
                if mob.id == follower { mob.extra.leader = Some(leader); }
                if mob.id == leader { mob.extra.followers += 1; }
            }
            "attached".to_owned()
        }
        Some("attack") => {
            let predator = by_tag(game, args.next().unwrap()).unwrap();
            let prey = by_tag(game, args.next().unwrap()).unwrap();
            let growth = args.next() == Some("growth");
            let random = &mut game.mobs.iter_mut().find(|mob| mob.id == predator).unwrap().random;
            for seed in 0..10_000 {
                random.set_seed(seed);
                if (random.next_float() < 0.2) == growth {
                    random.set_seed(seed);
                    break;
                }
            }
            let damage = Damage {
                kind: DamageKind::MobAttack,
                attacker: Some(Who::Mob(predator)),
                from: None,
                attacker_species: Some(Species::Cod),
            };
            format!("{}", game.mob_attacks_mob(prey, 1.0, damage))
        }
        Some("inspect") => {
            let id = by_tag(game, args.next().unwrap()).unwrap();
            let mob = game.mobs.iter().find(|mob| mob.id == id).unwrap();
            format!("{} {} {} {} {}", id, mob.extra.leader.unwrap_or(0), mob.extra.followers, mob.extra.variant, mob.alive() && !mob.removed)
        }
        _ => "invalid operation".to_owned(),
    };
    game.feedback(Text::literal(format!("Bass school fixture {output}")));
    ControlFlow::Break(true)
}
