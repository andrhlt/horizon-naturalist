fn deer_variant_egg(game: &mut Game<'_>, index: usize) {
    let parent = &game.mobs[index];
    if matches!(parent.kind.id.as_str(), "naturalist:deer" | "naturalist:duck" | "naturalist:boar" | "naturalist:turkey" | "naturalist:capybara" | "naturalist:komodo_dragon" | "naturalist:rhino" | "naturalist:bear" | "naturalist:black_bear" | "naturalist:zebra")
        && parent.alive()
        && horizon_client::mobs::species::can_be_a_baby(parent.species())
        && let Some(stack) = game.held()
        && horizon_client::mobs::finalize::spawn_egg_is_for(&stack.item, &parent.kind.id)
    {
        let parent_id = parent.id;
        super::super::naturalist_deer_variants::stage_offspring(game, parent_id);
    }
}
