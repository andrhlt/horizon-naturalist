fn bass_natural_cleanup(_result: (), game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_bass_state as bass;
    let mut leaders = HashMap::new();
    let mut followers = HashMap::<u32, i32>::new();
    for (index, mob) in game.mobs.iter_mut().enumerate() {
        if let Some(group) = bass::take_group(&mut mob.ai.nav) {
            if group.index == 0 {
                leaders.insert(mob.id, index);
            } else if let Some(id) = mob.extra.leader {
                *followers.entry(id).or_default() += 1;
            }
        }
    }
    for (id, index) in leaders {
        // Join events can cancel one of the requested followers.
        game.mobs[index].extra.followers = followers.get(&id).copied().unwrap_or_default();
    }
}
