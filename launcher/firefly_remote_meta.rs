fn firefly_remote_meta(mob: &mut Mob, values: &[(u8, i32, Value)]) {
    use horizon_client::mobs::goals::naturalist_firefly_behavior::{GLOW_META, apply_glow_meta};
    if mob.kind.id == "naturalist:firefly"
        && let Some((_, _, Value::Byte(phase))) =
            values.iter().find(|&(index, _, _)| *index == GLOW_META)
    {
        apply_glow_meta(mob, *phase);
    }
}
