fn elephant_combat_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{Who, ai::do_hurt_target, goals::Cx};
    use horizon_core::{geometry::Vec3, random::LegacyRandom};
    use std::ops::ControlFlow;

    if let Some(kind) = command.strip_prefix("naturalist_fixture_elephant_spawn ") {
        use crate::game::naturalist_deer_placement::{Pass, PlacementPass};
        use horizon_client::mobs::{
            spawn_rules::{Reason, check_spawn_rules},
            spawn_selection::NaturalLevel,
        };
        let _placement = PlacementPass::enter(Pass::Natural);
        let level = game.natural_level();
        let Some((species, _, _, _)) = level.mob_type(kind) else {
            return ControlFlow::Break(false);
        };
        let allowed = check_spawn_rules(
            species, &level, Reason::Natural, &mut LegacyRandom::new(3781), 12, 130, 12,
        );
        game.feedback(Text::literal(format!("Elephant spawn {allowed}")));
        return ControlFlow::Break(true);
    }

    let Some(kind) = command.strip_prefix("naturalist_fixture_elephant_attack ") else {
        return ControlFlow::Continue(());
    };
    let Some(index) = game.mobs.iter().position(|mob| mob.alive() && mob.kind.id == kind) else {
        return ControlFlow::Break(false);
    };
    let Some(victim) = game.mobs.iter_mut().find(|mob| mob.alive() && mob.kind.id == "minecraft:zombie") else {
        return ControlFlow::Break(false);
    };
    victim.velocity = Vec3::ZERO;
    victim.on_ground = true;
    let target = victim.id;
    let mut attacker = game.mobs[index].clone();
    let mut random = LegacyRandom::new(3781);
    let mut events = Vec::new();
    let world = game.mob_world_base(&[]);
    do_hurt_target(&mut attacker, Who::Mob(target), &mut Cx {
        w: &world, lr: &mut random, events: &mut events,
    });
    if game.handle_mob_events(events).is_err() {
        return ControlFlow::Break(false);
    }
    let Some(victim) = game.mobs.iter().find(|mob| mob.id == target) else {
        return ControlFlow::Break(false);
    };
    let motion = victim.velocity.components();
    let result = serde_json::json!({
        "target_health": victim.health,
        "target_motion": motion,
    });
    game.feedback(Text::literal(format!("Elephant attack {result}")));
    ControlFlow::Break(true)
}
