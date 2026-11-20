fn antivenom_cloud(game: &mut Game<'_>, cloud: &mut Breath, players: &[FightPlayer]) ->
    std::ops::ControlFlow<bool> {
    use super::naturalist_antivenom as antivenom;
    use std::ops::ControlFlow;

    let Some(potion) = cloud.potion.as_ref() else { return ControlFlow::Continue(()) };
    if !potion.effects.iter().any(|(id, ..)| *id == antivenom::ID) {
        return ControlFlow::Continue(());
    }
    let Some(_scope) = antivenom::CloudScope::enter() else { return ControlFlow::Continue(()) };
    let active = game.tick_potion_cloud(cloud, players);
    let Some(potion) = cloud.potion.as_ref() else { return ControlFlow::Break(active) };
    if cloud.age >= potion.settings.wait_time && cloud.age % 5 == 0 {
        let next = cloud.age.wrapping_add(potion.settings.reapplication_delay);
        for (victim, until) in &potion.victims {
            if *until != next {
                continue;
            }
            match *victim {
                Victim::Player(who) => game.as_fight_player(who, antivenom::clear_player),
                Victim::Mob(id) => {
                    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
                        antivenom::clear_mob(mob);
                        antivenom::clear_effect(mob, antivenom::ID);
                    }
                }
            }
        }
    }
    ControlFlow::Break(active)
}
