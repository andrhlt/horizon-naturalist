fn firefly_meta(mob: &Mob, meta: &mut Vec<(u8, i32, Value)>, defaults: &mut Vec<(u8, i32, Value)>) {
    use horizon_client::mobs::goals::naturalist_firefly_behavior::{GLOW_META, glow_meta};
    if mob.kind.id == "naturalist:firefly" {
        let phase = glow_meta(mob);
        if phase != 0 {
            meta.push((GLOW_META, serializer::BYTE, Value::Byte(phase)));
        }
        defaults.push((GLOW_META, serializer::BYTE, Value::Byte(0)));
    }
}
