fn animal_lan_fixture(result: Result<Option<Value>>, game: &Game<'_>, line: &str) -> Result<Option<Value>> {
    if line.trim() != "status" {
        return result;
    }
    let Some(mut status) = result? else { return Ok(None) };
    let variant = &super::naturalist_deer_variants::appearance_variant;
    let rows: Vec<_> = game.mobs.iter().filter_map(|mob| {
        let (coat, dye, sits) = match mob.kind.id.as_str() {
            "naturalist:duck" => super::naturalist_deer_variants::duck_appearance(game, mob.id, mob.is_baby()),
            "naturalist:deer" => (super::naturalist_deer_variants::appearance_texture(game, mob.id, mob.is_baby()), None, false),
            _ => return None,
        };
        Some(serde_json::json!({
            "type": mob.kind.id.as_str(),
            "variant": variant(game, mob.id),
            "coat": coat,
            "dye": dye,
            "sits": sits,
            "scale": mob.model_scale(),
            "baby": mob.is_baby(),
        }))
    }).collect();
    status["naturalist_fixture"] = serde_json::json!(rows);
    Ok(Some(status))
}
