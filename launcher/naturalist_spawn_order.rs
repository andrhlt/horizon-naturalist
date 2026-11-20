fn naturalist_spawn_order(
    biomes: &mut J,
    modifiers: &[Modifier],
    biome_reg: &Registry<'_>,
    feature_reg: &Registry<'_>,
    entity_reg: &Registry<'_>,
    entity_categories: &BTreeMap<String, String>,
    warnings: &mut Vec<String>,
) -> std::ops::ControlFlow<Result<Vec<(String, String)>>> {
    use std::ops::ControlFlow;

    // Java 2.0.7 NaturalistSpawns.forEachSpawn registers these together.
    // Horizon splits the additions into files, whose sort order is different.
    const ORDER: &[&str] = &[
        "alligator.json",
        "bear.json",
        "black_bear.json",
        "bird_bluejay.json",
        "boar.json",
        "bird_canary.json",
        "bird_cardinal.json",
        "capybara.json",
        "coral_snake.json",
        "crab.json",
        "deer.json",
        "desert_scorpion.json",
        "jungle_scorpion.json",
        "duck.json",
        "elephant.json",
        "bird_finch.json",
        "forest_foxes.json",
        "forest_rabbits.json",
        "hedgehog.json",
        "giraffe.json",
        "hippo.json",
        "komodo_dragon.json",
        "lion.json",
        "lizard.json",
        "mammoth.json",
        "mole.json",
        "ostrich.json",
        "rat.json",
        "rhino.json",
        "bird_robin.json",
        "snail.json",
        "snake.json",
        "rattlesnake.json",
        "bird_sparrow.json",
        "tiger.json",
        "tortoise.json",
        "turkey.json",
        "vulture.json",
        "zebra.json",
    ];
    let mut known: Vec<_> = modifiers
        .iter()
        .enumerate()
        .filter_map(|(index, modifier)| {
            let [mod_id, file, _] = &modifier.source;
            let Action::AddSpawns(spawners) = &modifier.action else { return None; };
            if mod_id != "naturalist" || spawners.is_empty() || !spawners.iter().all(|s| entity_categories.get(&s.entity).is_some_and(|c| c == "creature")) { return None; }
            let filename = file.strip_prefix("data/naturalist/neoforge/biome_modifier/")?;
            let rank = ORDER.iter().position(|name| *name == filename)?;
            Some((index, rank))
        })
        .collect();
    if known.is_sorted_by_key(|&(_, rank)| rank) {
        return ControlFlow::Continue(());
    }
    let slots: Vec<_> = known.iter().map(|&(index, _)| index).collect();
    known.sort_by_key(|&(_, rank)| rank);
    let mut indices: Vec<_> = (0..modifiers.len()).collect();
    for (slot, (index, _)) in slots.into_iter().zip(known) {
        indices[slot] = index;
    }

    let mut reordered = Vec::with_capacity(modifiers.len());
    for index in indices {
        let [mod_id, file, text] = &modifiers[index].source;
        match Modifier::read(mod_id, file, text, &mut Vec::new()) {
            Ok(Some(modifier)) => reordered.push(modifier),
            _ => return ControlFlow::Continue(()),
        }
    }
    ControlFlow::Break(apply(
        biomes,
        &reordered,
        biome_reg,
        feature_reg,
        entity_reg,
        entity_categories,
        warnings,
    ))
}
