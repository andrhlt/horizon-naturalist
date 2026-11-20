fn deer_content(
    plan: &Plan<'_>,
    bytes: &[u8],
) -> std::ops::ControlFlow<std::result::Result<Vec<u8>, String>> {
    use std::ops::ControlFlow;
    if plan.entity_types.iter().all(|entity| !matches!(entity.id.as_str(), "naturalist:ostrich" | "naturalist:mole" | "naturalist:dirt_trail" | "naturalist:elephant" | "naturalist:mammoth" | "naturalist:hedgehog" | "naturalist:snake" | "naturalist:rat" | "naturalist:ant" | "naturalist:whale" | "naturalist:great_white_shark" | "naturalist:hippo" | "naturalist:alligator" | "naturalist:deer" | "naturalist:snail" | "naturalist:giraffe" | "naturalist:crab" | "naturalist:duck" | "naturalist:bass" | "naturalist:vulture" | "naturalist:bird" | "naturalist:catfish" | "naturalist:piranha" | "naturalist:blobfish" | "naturalist:ray" | "naturalist:anglerfish" | "naturalist:starfish" | "naturalist:giant_isopod" | "naturalist:clam" | "naturalist:jellyfish" | "naturalist:boar" | "naturalist:turkey" | "naturalist:dragonfly" | "naturalist:firefly" | "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:capybara" | "naturalist:desert_scorpion" | "naturalist:jungle_scorpion" | "naturalist:lizard" | "naturalist:lizard_tail" | "naturalist:komodo_dragon" | "naturalist:bear" | "naturalist:black_bear" | "naturalist:rhino" | "naturalist:zebra" | "naturalist:lion" | "naturalist:tiger" | "naturalist:tortoise")) {
        return ControlFlow::Continue(());
    }

    fn deer_rows(plan: &Plan<'_>, bytes: &[u8]) -> std::result::Result<Vec<u8>, String> {
        use serde_json::value::RawValue;
        let rows: BTreeMap<String, Box<RawValue>> =
            serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
        let mut out = bytes.trim_ascii_end().to_vec();
        if out.pop() != Some(b'}') {
            return Err("not an object".into());
        }
        let mut empty = rows.is_empty();
        for entity in &plan.entity_types {
            if rows.contains_key(&entity.id) {
                continue;
            }
            let (template, dimensions) = match entity.id.as_str() {
                "naturalist:ostrich" => ("minecraft:donkey", Some([1.2_f32, 2.5_f32, 2.125_f32])),
                "naturalist:mole" => ("minecraft:cow", Some([0.7_f32, 0.65_f32, 0.5525_f32])),
                "naturalist:dirt_trail" => ("minecraft:cow", Some([0.7_f32, 0.3_f32, 0.0_f32])),
                "naturalist:carried_food" => ("minecraft:item", Some([0.25, 0.25, 0.2125])),
                "naturalist:elephant" => ("minecraft:cow", Some([2.5_f32, 3.5_f32, 3.5_f32 * 0.85_f32])),
                "naturalist:mammoth" => ("minecraft:cow", Some([2.5_f32, 4.0_f32, 4.0_f32 * 0.85_f32])),
                "naturalist:rat" => ("minecraft:cow", Some([0.9_f32, 0.6_f32, 0.6_f32 * 0.85_f32])),
                "naturalist:snake" => ("minecraft:cow", Some([0.6_f32, 0.7_f32, 0.7_f32 * 0.85_f32])),
                "naturalist:hedgehog" => ("minecraft:wolf", Some([0.6_f32, 0.5_f32, 0.425_f32])),
                "naturalist:ant" => ("minecraft:wolf", Some([0.6_f32, 0.5_f32, 0.425_f32])),
                "naturalist:tiger" | "naturalist:lion" => ("minecraft:cow", Some([1.5_f32, 1.8_f32, 1.8_f32 * 0.85_f32])),
                "naturalist:tortoise" => ("minecraft:cow", Some([1.2_f32, 0.875_f32, 0.875_f32 * 0.85_f32])),
                "naturalist:whale" => ("minecraft:cow", Some([3.0_f32, 2.5_f32, 2.125_f32])),
                "naturalist:great_white_shark" => ("minecraft:cow", Some([1.4_f32, 1.6_f32, 1.36_f32])),
                "naturalist:hippo" => ("minecraft:cow", Some([1.8_f32, 1.8_f32, 1.53_f32])),
                "naturalist:alligator" => ("minecraft:cow", Some([1.8_f32, 0.8_f32, 0.68_f32])),
                "naturalist:bear" => ("minecraft:cow", Some([1.4_f32, 1.7_f32, 1.7_f32 * 0.85_f32])),
                "naturalist:black_bear" => ("minecraft:cow", Some([1.3_f32, 1.3_f32, 1.3_f32 * 0.85_f32])),
                "naturalist:capybara" => ("minecraft:cow", Some([0.9_f32, 0.9_f32, 0.9_f32 * 0.85_f32])),
                "naturalist:desert_scorpion" => ("minecraft:cow", Some([0.7_f32, 0.3_f32, 0.3_f32 * 0.85_f32])),
                "naturalist:jungle_scorpion" => ("minecraft:cow", Some([1.2_f32, 1.0_f32, 0.85_f32])),
                "naturalist:komodo_dragon" => ("minecraft:cow", Some([1.3_f32, 0.7_f32, 0.7_f32 * 0.85_f32])),
                "naturalist:zebra" => ("minecraft:donkey", Some([1.3964844_f32, 1.5_f32, 1.5_f32 * 0.85_f32])),
                "naturalist:rhino" => ("minecraft:cow", Some([2.5_f32, 3.0_f32, 3.0_f32 * 0.85_f32])),
                "naturalist:lizard" => ("minecraft:cow", Some([0.8_f32, 0.5_f32, 0.425_f32])),
                "naturalist:lizard_tail" => ("minecraft:cow", Some([0.7_f32, 0.5_f32, 0.425_f32])),
                "naturalist:boar" => ("minecraft:pig", Some([0.9_f32, 0.9_f32, 0.9_f32 * 0.85_f32])),
                "naturalist:turkey" => ("minecraft:chicken", Some([0.6_f32, 0.8_f32, 0.8_f32 * 0.85_f32])),
                "naturalist:deer" => ("minecraft:cow", Some([1.3_f32, 1.6_f32, 1.36_f32])),
                "naturalist:giraffe" => ("minecraft:cow", Some([1.9_f32, 5.4_f32, 5.4_f32 * 0.85_f32])),
                "naturalist:snail" => ("minecraft:cow", Some([0.7_f32, 0.7_f32, 0.7_f32 * 0.85_f32])),
                "naturalist:vulture" => ("minecraft:parrot", Some([0.9_f32, 0.5_f32, 0.425_f32])),
                "naturalist:bird" => ("minecraft:parrot", Some([0.5_f32, 0.6_f32, 0.6_f32 * 0.85_f32])),
                "naturalist:dragonfly" => ("minecraft:parrot", Some([0.9_f32, 0.7_f32, 0.7_f32 * 0.85_f32])),
                "naturalist:firefly" => ("minecraft:parrot", Some([0.7_f32, 0.6_f32, 0.6_f32 * 0.85_f32])),
                "naturalist:butterfly" => ("minecraft:parrot", Some([0.7_f32, 0.6_f32, 0.6_f32 * 0.85_f32])),
                "naturalist:caterpillar" => ("minecraft:cow", Some([0.4_f32, 0.4_f32, 0.4_f32 * 0.85_f32])),
                "naturalist:jellyfish" => ("minecraft:cod", Some([1.0_f32, 1.0_f32, 0.85_f32])),
                "naturalist:giant_isopod" => ("minecraft:cow", Some([0.6_f32, 0.4_f32, 0.34_f32])),
                "naturalist:clam" => ("minecraft:cod", Some([1.0_f32, 0.6_f32, 0.6_f32 * 0.85_f32])),
                "naturalist:crab" => ("minecraft:wolf", Some([0.5_f32, 0.6_f32, 0.6_f32 * 0.85_f32])),
                "naturalist:starfish" => ("minecraft:cod", Some([0.6_f32, 0.3_f32, 0.255_f32])),
                "naturalist:anglerfish" => ("minecraft:cod", Some([0.9_f32, 0.8_f32, 0.68_f32])),
                "naturalist:ray" => ("minecraft:cod", Some([1.0_f32, 0.4_f32, 0.34_f32])),
                "naturalist:blobfish" => ("minecraft:cod", Some([0.8_f32, 0.4_f32, 0.34_f32])),
                "naturalist:piranha" => ("minecraft:cod", Some([0.5_f32, 0.6_f32, 0.6_f32 * 0.85_f32])),
                "naturalist:catfish" => ("minecraft:cod", Some([0.7_f32, 0.4_f32, 0.34_f32])),
                "naturalist:bass" => ("minecraft:cod", Some([0.3_f32, 0.4_f32, 0.34_f32])),
                "naturalist:duck" => ("minecraft:chicken", Some([0.6_f32, 1.0_f32, 0.85_f32])),
                _ => (entity.template.as_str(), None),
            };
            let mut custom;
            let entity = if dimensions.is_some() {
                custom = (*entity).clone();
                custom.template = template.to_owned();
                &custom
            } else {
                entity
            };
            let Some(row) = rows.get(&entity.template) else {
                continue;
            };
            if !std::mem::take(&mut empty) {
                out.push(b',');
            }
            out.extend(serde_json::to_vec(&entity.id).map_err(|error| error.to_string())?);
            out.push(b':');
            if let Some(dimensions) = dimensions {
                let mut fields: BTreeMap<String, Box<RawValue>> =
                    serde_json::from_str(row.get()).map_err(|error| error.to_string())?;
                for (key, value) in ["width", "height", "eye_height"].into_iter().zip(dimensions) {
                    let value = serde_json::value::to_raw_value(&f64::from(value))
                        .map_err(|error| error.to_string())?;
                    fields.insert(key.to_owned(), value);
                }
                if entity.id == "naturalist:ostrich" {
                    let mut attributes: BTreeMap<String, Box<RawValue>> = fields.get("attributes")
                        .map(|row| serde_json::from_str(row.get())).transpose()
                        .map_err(|error| error.to_string())?.unwrap_or_default();
                    for (attribute, value) in [
                        ("minecraft:max_health", 30.0),
                        ("minecraft:movement_speed", 0.24),
                        ("minecraft:attack_damage", 4.0),
                        ("minecraft:jump_strength", 0.75),
                        ("minecraft:step_height", 1.0625),
                    ] {
                        attributes.insert(attribute.to_owned(), serde_json::value::to_raw_value(&value)
                            .map_err(|error| error.to_string())?);
                    }
                    fields.insert("attributes".to_owned(), serde_json::value::to_raw_value(&attributes)
                        .map_err(|error| error.to_string())?);
                }
                let head_turn = match entity.id.as_str() {
                    "naturalist:giraffe" => Some(45.0),
                    "naturalist:elephant" | "naturalist:mammoth" => Some(35.0),
                    _ => None,
                };
                if let Some(head_turn) = head_turn {
                    fields.insert("max_head_y_rot".to_owned(),
                        serde_json::value::to_raw_value(&head_turn).map_err(|error| error.to_string())?);
                }
                if entity.id == "naturalist:caterpillar" {
                    fields.insert("baby_scale".to_owned(),
                        serde_json::value::to_raw_value(&1.0).map_err(|error| error.to_string())?);
                }
                if matches!(entity.id.as_str(), "naturalist:butterfly" | "naturalist:dragonfly" | "naturalist:firefly") {
                    fields.insert("category".to_owned(),
                        serde_json::value::to_raw_value("ambient").map_err(|error| error.to_string())?);
                }
                if matches!(entity.id.as_str(), "naturalist:elephant" | "naturalist:mammoth" | "naturalist:snake" | "naturalist:great_white_shark" | "naturalist:hippo" | "naturalist:lion" | "naturalist:tiger" | "naturalist:alligator" | "naturalist:tortoise" | "naturalist:bass" | "naturalist:vulture" | "naturalist:crab" | "naturalist:catfish" | "naturalist:piranha" | "naturalist:blobfish" | "naturalist:ray" | "naturalist:anglerfish" | "naturalist:boar" | "naturalist:turkey" | "naturalist:desert_scorpion" | "naturalist:jungle_scorpion" | "naturalist:lizard" | "naturalist:komodo_dragon" | "naturalist:bear" | "naturalist:black_bear" | "naturalist:rhino") {
                    let mut attributes: BTreeMap<String, Box<RawValue>> = fields.get("attributes")
                        .map(|row| serde_json::from_str(row.get())).transpose()
                        .map_err(|error| error.to_string())?.unwrap_or_default();
                    attributes.insert("minecraft:attack_damage".to_owned(), serde_json::value::to_raw_value(&1.0)
                        .map_err(|error| error.to_string())?);
                    if matches!(entity.id.as_str(), "naturalist:boar" | "naturalist:elephant" | "naturalist:mammoth") {
                        attributes.insert("minecraft:attack_knockback".to_owned(), serde_json::value::to_raw_value(&0.0)
                            .map_err(|error| error.to_string())?);
                    }
                    fields.insert("attributes".to_owned(), serde_json::value::to_raw_value(&attributes)
                        .map_err(|error| error.to_string())?);
                }
                if matches!(entity.id.as_str(),
                    "naturalist:bird" | "naturalist:butterfly" | "naturalist:dragonfly" |
                    "naturalist:firefly" | "naturalist:jellyfish" | "naturalist:starfish" | "naturalist:rat" | "naturalist:whale" | "naturalist:hedgehog")
                {
                    if let Some(row) = fields.get_mut("attributes") {
                        let mut attributes: BTreeMap<String, Box<RawValue>> =
                            serde_json::from_str(row.get()).map_err(|error| error.to_string())?;
                        attributes.remove("minecraft:attack_damage");
                        *row = serde_json::value::to_raw_value(&attributes)
                            .map_err(|error| error.to_string())?;
                    }
                }
                if entity.id == "naturalist:dirt_trail" {
                    fields.insert("category".to_owned(), serde_json::value::to_raw_value("misc")
                        .map_err(|error| error.to_string())?);
                    fields.insert("fire_immune".to_owned(), serde_json::value::to_raw_value(&true)
                        .map_err(|error| error.to_string())?);
                }
                if matches!(entity.id.as_str(), "naturalist:whale" | "naturalist:great_white_shark") {
                    let category = serde_json::value::to_raw_value(&"water_creature")
                        .map_err(|error| error.to_string())?;
                    fields.insert("category".to_owned(), category);
                }
                if entity.id == "naturalist:giant_isopod" {
                    let category = serde_json::value::to_raw_value(&"water_ambient")
                        .map_err(|error| error.to_string())?;
                    fields.insert("category".to_owned(), category);
                }
                let row = serde_json::value::to_raw_value(&fields).map_err(|error| error.to_string())?;
                out.extend(mod_entity_row(&row, entity)?);
            } else {
                out.extend(mod_entity_row(row, entity)?);
            }
        }
        out.push(b'}');
        Ok(out)
    }
    ControlFlow::Break(deer_rows(plan, bytes))
}
