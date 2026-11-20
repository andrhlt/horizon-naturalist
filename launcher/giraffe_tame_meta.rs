fn giraffe_tame_meta(
    mob: &Mob,
    meta: &mut Vec<(u8, i32, Value)>,
    defaults: &mut Vec<(u8, i32, Value)>,
) {
    if mob.kind.id == "naturalist:rat" {
        let owner = mob.extra.owner.map(horizon_client::lan::playerdata::ints_uuid);
        if owner.is_some() {
            meta.push((251, serializer::OPTIONAL_LIVING_ENTITY_REFERENCE, Value::OptUuid(owner)));
        }
        defaults.push((251, serializer::OPTIONAL_LIVING_ENTITY_REFERENCE, Value::OptUuid(None)));
        return;
    }
    if !matches!(mob.kind.id.as_str(), "naturalist:giraffe" | "naturalist:lion" | "naturalist:tiger") {
        return;
    }
    if mob.extra.tame {
        meta.push((251, serializer::BOOLEAN, Value::Bool(true)));
    }
    defaults.push((251, serializer::BOOLEAN, Value::Bool(false)));
}
