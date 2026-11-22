/// Naturalist's Java Anglerfish hunt timer, in game ticks.
pub const HUNTING_COOLDOWN: i32 = 2400;

/// Advance a persisted hunt timer without allowing it to become negative.
pub const fn next_hunting_cooldown(ticks: i32) -> i32 {
    if ticks > 0 { ticks - 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::{HUNTING_COOLDOWN, next_hunting_cooldown};

    #[test]
    fn first_retaliatory_hit_starts_java_timer() {
        assert_eq!(HUNTING_COOLDOWN, 2400);
        assert_eq!(next_hunting_cooldown(HUNTING_COOLDOWN), 2399);
    }

    #[test]
    fn persisted_timer_is_clamped_at_zero() {
        assert_eq!(next_hunting_cooldown(0), 0);
        assert_eq!(next_hunting_cooldown(-1), 0);
    }
}
