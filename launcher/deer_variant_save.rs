fn deer_variant_save(tag: &mut Tag) {
    let Some(entity) = kind(tag) else { return };
    if !matches!(entity.as_str(), "naturalist:ant" | "naturalist:hedgehog" | "naturalist:whale" | "naturalist:great_white_shark" | "naturalist:hippo" | "naturalist:alligator" | "naturalist:deer" | "naturalist:giraffe" | "naturalist:elephant" | "naturalist:mammoth" | "naturalist:duck" | "naturalist:bass" | "naturalist:catfish" | "naturalist:piranha" | "naturalist:blobfish" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:anglerfish" | "naturalist:starfish" | "naturalist:giant_isopod" | "naturalist:clam" | "naturalist:crab" | "naturalist:vulture" | "naturalist:bird" | "naturalist:boar" | "naturalist:turkey" | "naturalist:capybara" | "naturalist:desert_scorpion" | "naturalist:jungle_scorpion" | "naturalist:lizard" | "naturalist:lizard_tail" | "naturalist:komodo_dragon" | "naturalist:bear" | "naturalist:black_bear" | "naturalist:rhino" | "naturalist:zebra" | "naturalist:tortoise" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake" | "naturalist:rat") {
        return;
    }
    let normalized = text(tag.get("Variant")).and_then(|id| {
        let (namespace, path) = id.split_once(':').unwrap_or(("minecraft", id.as_str()));
        let valid_namespace = !namespace.is_empty()
            && namespace
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"_-.".contains(&b));
        let valid_path = !path.is_empty()
            && path
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"_-./".contains(&b));
        (valid_namespace && valid_path).then(|| format!("{namespace}:{path}"))
    });
    let legacy_bass = (entity == "naturalist:bass")
        .then(|| number(tag.get("Variant")).or_else(|| number(tag.get("variant"))))
        .flatten()
        .and_then(|value| match value {
            0 => Some("naturalist:bass"),
            1 => Some("naturalist:bass_medium"),
            2 => Some("naturalist:bass_large"),
            _ => None,
        });
    let numeric = (entity == "naturalist:jellyfish").then(|| number(tag.get("Variant"))).flatten();
    let legacy_jellyfish = numeric.map(|value| {
        ["naturalist:white", "naturalist:orange", "naturalist:pink", "naturalist:blue", "naturalist:green"][value.rem_euclid(5) as usize]
    });
    let legacy_ray = (entity == "naturalist:ray").then(|| number(tag.get("Variant"))).flatten().map(|value| {
        ["naturalist:eagle_ray", "naturalist:mobula_ray", "naturalist:stingray"][value.rem_euclid(3) as usize]
    });
    let legacy_anglerfish = (entity == "naturalist:anglerfish")
        .then(|| number(tag.get("Variant"))).flatten()
        .map(|value| ["naturalist:red", "naturalist:glow"][value.rem_euclid(2) as usize]);
    let legacy_starfish = (entity == "naturalist:starfish")
        .then(|| number(tag.get("Variant"))).flatten()
        .map(|value| ["naturalist:orange", "naturalist:purple", "naturalist:blue", "naturalist:red"][value.rem_euclid(4) as usize]);
    let legacy_giant_isopod = (entity == "naturalist:giant_isopod")
        .then(|| number(tag.get("Variant"))).flatten()
        .map(|value| ["naturalist:brown", "naturalist:blue"][value.rem_euclid(2) as usize]);
    let legacy_clam = (entity == "naturalist:clam")
        .then(|| number(tag.get("Variant"))).flatten()
        .map(|value| ["naturalist:brown", "naturalist:white"][value.rem_euclid(2) as usize]);
    let legacy_crab = (entity == "naturalist:crab")
        .then(|| number(tag.get("Variant"))).flatten()
        .map(|value| ["naturalist:blue", "naturalist:brown", "naturalist:orange", "naturalist:red", "naturalist:yellow"][value.rem_euclid(5) as usize]);
    let legacy_tortoise = (entity == "naturalist:tortoise")
        .then(|| number(tag.get("Variant"))).flatten()
        .map(|value| ["naturalist:brown", "naturalist:green", "naturalist:black"][value.rem_euclid(3) as usize]);
    let legacy_tiger = (entity == "naturalist:tiger")
        .then(|| number(tag.get("Variant"))).flatten()
        .map(|value| crate::mobs::goals::naturalist_cat_goals::COLORS[value.rem_euclid(4) as usize]);
    let default = match entity.as_str() {
        "naturalist:rat" => "naturalist:black",
        "naturalist:tiger" => "naturalist:black_panther",
        "naturalist:tortoise" | "naturalist:hedgehog" => "naturalist:brown",
        "naturalist:crab" => "naturalist:blue",
        "naturalist:anglerfish" => "naturalist:red",
        "naturalist:starfish" => "naturalist:orange",
        "naturalist:clam" => "naturalist:brown",
        "naturalist:giant_isopod" => "naturalist:brown",
        "naturalist:ray" => "naturalist:eagle_ray",
        "naturalist:jellyfish" => "naturalist:white",
        "naturalist:bird" => "naturalist:american_robin",
        "naturalist:jungle_scorpion" => match number(tag.get("Variant")) {
            Some(1) => "naturalist:green",
            _ => "naturalist:black",
        },
        "naturalist:lizard" | "naturalist:lizard_tail" => match number(tag.get("Variant")) {
            Some(1) => "naturalist:brown",
            Some(2) => "naturalist:beardie",
            Some(3) => "naturalist:leopard_gecko",
            _ => "naturalist:green",
        },
        _ => &entity,
    };
    put(
        tag,
        "Variant",
        string(normalized.as_deref().or(legacy_bass).or(legacy_jellyfish).or(legacy_ray).or(legacy_anglerfish).or(legacy_starfish).or(legacy_giant_isopod).or(legacy_clam).or(legacy_crab).or(legacy_tortoise).or(legacy_tiger).unwrap_or(default)),
    );
}
