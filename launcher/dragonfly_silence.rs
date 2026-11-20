fn dragonfly_silence(
    mob: &Mob,
    meta: &mut Vec<(u8, i32, Value)>,
    defaults: &mut Vec<(u8, i32, Value)>,
) {
    if mob.kind.id != "naturalist:dragonfly" {
        return;
    }
    if mob.silent {
        meta.push((4, serializer::BOOLEAN, Value::Bool(true)));
    }
    defaults.push((4, serializer::BOOLEAN, Value::Bool(false)));
}
