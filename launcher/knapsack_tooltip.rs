fn knapsack_tooltip(game: &Game<'_>, stack: &ItemStack, lines: &mut Vec<(Text, u32)>) {
    use super::naturalist_knapsack as sack;
    use horizon_client::scoreboard::{comp_text, tag_to_json};
    use horizon_world::identifier::Identifier;
    use serde_json::Value;

    if stack.item.as_ref() != sack::ITEM || !sack::filled(stack) {
        return;
    }
    let Some(tag) = stack.custom_data() else { return };
    fn gray_root(component: &mut Value) {
        match component {
            Value::Object(fields) => {
                fields.insert("color".into(), Value::String("gray".into()));
            }
            Value::Array(parts) => {
                if let Some(first) = parts.first_mut() {
                    gray_root(first);
                }
            }
            _ => {}
        }
    }
    let label = tag.get(&"CustomName".into())
        .and_then(horizon_client::block_entity_nbt::component)
        .map(|name| {
            let mut json = tag_to_json(&name);
            gray_root(&mut json);
            comp_text(&json)
        })
        .or_else(|| {
            let name = horizon_client_core::nbt_value::text(tag.get(&"id".into()))?;
            let kind = Identifier::parse(&name).ok()?;
            let id = kind.to_string();
            let known = game.entity_type_exists(&id) || game.res.mob_types().contains_key(&id);
            let key = format!("entity.{}.{}", kind.namespace(), kind.path().replace('/', "."));
            known.then(|| Text::translate(&key, &[]))
        });
    if let Some(text) = label {
        lines.push((text.with_color(0xAAAAAA), 0xFFAAAAAA));
    }
}
