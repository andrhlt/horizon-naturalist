fn ant_hill_tick_ready(block: &str, ticks: &mut bool) {
    if block == super::naturalist_ant_hill::HILL {
        *ticks = true;
    }
}
