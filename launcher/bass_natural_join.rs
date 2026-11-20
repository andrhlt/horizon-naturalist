fn bass_natural_join(game: &mut Game<'_>, id: u32) {
    use horizon_client::mobs::nav::naturalist_bass_state as bass;

    let index = game.mobs.len() - 1;
    let Some(group) = bass::group(&game.mobs[index].ai.nav).cloned() else {
        return;
    };
    if group.index > 0 {
        if let Some(leader) = game.mobs[..index].iter().rposition(|fish| {
            fish.kind.id == game.mobs[index].kind.id && !fish.removed && fish.alive()
                && bass::group(&fish.ai.nav).is_some_and(|head| head.index == 0 && head.leader_position == group.leader_position)
        }) {
            game.mobs[index].extra.leader = Some(game.mobs[leader].id);
            game.mobs[leader].extra.followers += 1;
        }
    }
    super::naturalist_deer_variants::put_variant(game, id, &group.variant);
}
