fn bird_support(mut state: StateData) -> StateData {
    if (state.flags & VALID_SPAWN != 0) != (state.flags & VALID_SPAWN_LEAVES != 0)
        && super::naturalist_spawn_variants::generation_is_bird() {
        state.flags = (state.flags & !VALID_SPAWN_LEAVES) | if state.flags & VALID_SPAWN != 0 { VALID_SPAWN_LEAVES } else { 0 };
    }
    state
}
