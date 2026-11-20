fn giraffe_tame_remote(mob: &mut Mob, values: &[(u8, i32, Value)]) {
    if mob.kind.id == "naturalist:rat" {
        for (field, _, value) in values {
            if *field == 251 && let Value::OptUuid(owner) = value {
                mob.extra.owner = owner.map(horizon_client::lan::playerdata::uuid_ints);
                mob.extra.tame = owner.is_some();
            }
        }
        return;
    }
    if matches!(mob.kind.id.as_str(), "naturalist:giraffe" | "naturalist:lion" | "naturalist:tiger") {
        for (field, _, value) in values {
            if *field == 251 && let Value::Bool(tame) = value {
                mob.extra.tame = *tame;
            }
        }
    }
}
