fn vulture_perch_meta(
    mob: &Mob,
    meta: &mut Vec<(u8, i32, Value)>,
    defaults: &mut Vec<(u8, i32, Value)>,
) {
    use horizon_client::mobs::goals::naturalist_vulture_goals::state;
    if mob.kind.id == "naturalist:vulture" {
        if state(mob).perched_ticks > 0 {
            meta.push((251, serializer::BOOLEAN, Value::Bool(true)));
        }
        defaults.push((251, serializer::BOOLEAN, Value::Bool(false)));
    }
}
