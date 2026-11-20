fn snail_appearance_meta(
    mob: &Mob,
    meta: &mut Vec<(u8, i32, Value)>,
    defaults: &mut Vec<(u8, i32, Value)>,
) {
    use horizon_client::mobs::naturalist_snail::{APPEARANCE_META, NORMAL_META, snail};
    if mob.kind.id != "naturalist:snail" {
        return;
    }
    let state = snail(mob);
    let appearance = state.color as i8 | (i8::from(state.hiding) << 4);
    if appearance != 12 {
        meta.push((APPEARANCE_META, serializer::BYTE, Value::Byte(appearance)));
    }
    defaults.push((APPEARANCE_META, serializer::BYTE, Value::Byte(12)));

    let normal = state.surface.normal;
    if state.surface.attached && (normal.x != 0.0 || normal.y != 1.0 || normal.z != 0.0) {
        meta.push((
            NORMAL_META,
            serializer::VECTOR3,
            Value::Floats(vec![normal.x as f32, normal.y as f32, normal.z as f32]),
        ));
    }
    defaults.push((
        NORMAL_META,
        serializer::VECTOR3,
        Value::Floats(vec![0.0, 1.0, 0.0]),
    ));
}
