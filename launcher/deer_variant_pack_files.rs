fn deer_variant_pack_files(store: &mut DataStore, packs: &Vec<&DataPack>) {
    for registry in ["naturalist/ant_variant", "naturalist/deer_variant", "naturalist/duck_variant", "naturalist/bass_variant", "naturalist/bird_variant", "naturalist/boar_variant", "naturalist/turkey_variant", "naturalist/capybara_variant", "naturalist/desert_scorpion_variant", "naturalist/jungle_scorpion_variant", "naturalist/lizard_variant", "naturalist/lizard_tail_variant", "naturalist/komodo_dragon_variant", "naturalist/rhino_variant", "naturalist/bear_variant", "naturalist/black_bear_variant", "naturalist/zebra_variant", "naturalist/tortoise_variant", "naturalist/tiger_variant", "naturalist/lion_variant", "naturalist/snake_variant", "naturalist/rat_variant"] {
        for pack in packs {
        for path in pack.list_folder(registry) {
            let Some((namespace, tail)) = path
                .strip_prefix("data/")
                .and_then(|path| path.split_once('/'))
            else {
                continue;
            };
            let Some(name) = tail
                .strip_prefix(registry)
                .and_then(|tail| tail.strip_prefix('/'))
                .and_then(|tail| tail.strip_suffix(".json"))
            else {
                continue;
            };
            if let Some(bytes) = pack.read(&path) {
                store
                    .files
                    .entry(format!("{registry}/{namespace}:{name}"))
                    .or_default()
                    .push(bytes);
            }
        }
    }
    }
}
