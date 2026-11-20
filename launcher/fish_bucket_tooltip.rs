fn fish_bucket_tooltip(game: &Game<'_>, stack: &ItemStack, lines: &mut Vec<(Text, u32)>) {
    use super::naturalist_deer_variants::variant_tooltip;
    use horizon_world::identifier::Identifier;

    if stack.item.as_ref() == super::naturalist_glow_goop::ID {
        if horizon_client::mobs::naturalist_config::glow_goop_tooltip() {
            let grey = 0xAAAAAA;
            if game.has_shift_down() {
                for text in ["Place up to 3", "in one space!"] {
                    lines.push((Text::literal(text).with_color(grey), 0xFFAAAAAA));
                }
            } else {
                let text = Text::literal("Hold ").with_color(grey)
                    .append(Text::literal("Shift").with_color(0xFFFF55).italic())
                    .append(Text::literal(" for info!").with_color(grey));
                lines.push((text, 0xFFAAAAAA));
            }
        }
        return;
    }

    let (kind, names, prefix): (_, &[&str], _) = match stack.item.as_ref() {
        "naturalist:bass_bucket" => ("naturalist:bass", &["bass", "bass_medium", "bass_large"], None),
        "naturalist:jellyfish_bucket" => ("naturalist:jellyfish", &["white", "orange", "pink", "blue", "green"], Some("color.minecraft.")),
        "naturalist:ray_bucket" => ("naturalist:ray", &["eagle_ray", "mobula_ray", "stingray"], Some("tooltip.naturalist.ray_")),
        "naturalist:crab" => ("naturalist:crab", &["blue", "brown", "orange", "red", "yellow"], Some("tooltip.naturalist.crab_")),
        "naturalist:starfish_bucket" => ("naturalist:starfish", &["orange", "purple", "blue", "red"], Some("color.minecraft.")),
        "naturalist:giant_isopod_bucket" => ("naturalist:giant_isopod", &["brown", "blue"], Some("color.minecraft.")),
        "naturalist:anglerfish_bucket" => ("naturalist:anglerfish", &["red", "glow"], Some("tooltip.naturalist.anglerfish_")),
        "naturalist:blobfish_bucket" => ("naturalist:blobfish", &[], None),
        "naturalist:catfish_bucket" => ("naturalist:catfish", &[], None),
        "naturalist:piranha_bucket" => ("naturalist:piranha", &[], None),
        _ => return,
    };
    let Some(saved) = stack.data().and_then(|data| data.other("minecraft:custom_data"))
        .and_then(|tag| tag.get("Variant"))
    else { return };

    let variant = if let Some(id) = horizon_client_core::nbt_value::text(Some(saved)) {
        Identifier::parse(&id).ok()
    } else if !names.is_empty() && let Some(index) = horizon_client::save::number(Some(saved)) {
        Identifier::from_parts("naturalist", names[index.rem_euclid(names.len() as i64) as usize]).ok()
    } else {
        None
    };
    let Some(variant) = variant else { return };
    let tooltip = variant_tooltip(game, kind, &variant.to_string())
        .or_else(|| prefix.map(|prefix| Text::translate(&format!("{prefix}{}", variant.path()), &[])));
    if let Some(text) = tooltip {
        lines.push((text.with_color(0xAAAAAA), 0xFFAAAAAA));
    }
}
