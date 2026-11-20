fn lion_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{
        finalize::{Attr, modifier_amount},
        goals::{naturalist_cat_goals as cats, naturalist_lion_goals as lion},
        nav::naturalist_catfish_state as hunt, Who,
    };
    use std::ops::ControlFlow;
    let Some(query) = command.strip_prefix("naturalist_fixture_lion ") else { return ControlFlow::Continue(()) };
    let Some(mob) = game.mobs.iter().find(|mob| mob.kind.id == lion::KIND && mob.alive()
        && mob.position.x < 14.0 && (mob.is_baby() == (query == "cub") || query == "follower"))
    else { return ControlFlow::Break(false) };
    if let Some(case) = query.strip_prefix("roll ") {
        use horizon_core::random::RandomSource;
        let values: Vec<_> = case.split_whitespace().filter_map(|value| value.parse::<u32>().ok()).collect();
        let [seed, index, base_cub] = values.as_slice() else { return ControlFlow::Break(false) };
        let mut sample = mob.clone();
        sample.random.set_seed(i64::from(*seed));
        lion::natural_pride_member(&mut sample, *index, *base_cub != 0);
        let result = serde_json::json!([sample.extra.age, sample.extra.flag]);
        game.feedback(Text::literal(format!("Lion fixture {result}")));
        return ControlFlow::Break(true);
    }
    let world = game.mob_world_base(&[]);
    let (width, height, eye) = mob.dims();
    let dims = [width, height, eye].map(|value| (value * 10000.0).round() as i32);
    let leader = lion::pride(&mob.ai.nav).leader
        .and_then(|id| game.persist.uuids.get(&id))
        .map_or(-1, |uuid| uuid[3]);
    let target = match mob.ai.target {
        Some(Who::Mob(id)) => game.mobs.iter().find(|other| other.id == id).map(|other| other.kind.id.as_str()),
        Some(Who::Player) => Some("player"),
        _ => None,
    };
    let result = serde_json::json!({
        "dims": dims, "food_ids": mob.kind.food.as_ref(), "ambient_interval": mob.ambient_interval(),
        "leader": leader, "movement": mob.movement_speed(), "mane": mob.extra.flag,
        "follow_spawn_bonus": modifier_amount(mob, Attr::FollowRange, lion::SPAWN_BONUS),
        "running_pride": mob.ai.goals.is_running("FollowLeader"),
        "clock": world.day_time, "can_sleep": cats::can_sleep(mob, &world),
        "sleeping": cats::sleeping(mob), "target": target, "cooldown": hunt::cooldown(&mob.ai.nav),
        "pos": mob.position.components(),
    });
    game.feedback(Text::literal(format!("Lion fixture {result}")));
    ControlFlow::Break(true)
}
