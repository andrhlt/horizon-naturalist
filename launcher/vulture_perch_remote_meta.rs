fn vulture_perch_remote_meta(mob: &mut Mob, values: &[(u8, i32, Value)]) {
    use horizon_client::mobs::goals::naturalist_vulture_goals::state_mut;
    if mob.kind.id == "naturalist:vulture"
        && let Some((_, _, Value::Bool(perched))) =
            values.iter().find(|&(index, _, _)| *index == 251)
    {
        state_mut(mob).perched_ticks = i32::from(*perched);
    }
}
