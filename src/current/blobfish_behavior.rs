//! Blobfish pressure appearance and conversion timer from Naturalist's Java AI.

use mod_api::{Compound, Tag};

pub const CONVERSION_TICKS: i32 = 100;
pub const PRESSURE_DEPTH: i32 = 33;

/// Advance the pressure-driven skin conversion by one server tick.
pub fn advance(gray: bool, conversion_time: i32, deep: bool) -> (bool, i32) {
    if gray == deep {
        return (gray, 0);
    }
    let elapsed = conversion_time.max(0).saturating_add(1);
    if elapsed >= CONVERSION_TICKS {
        (deep, 0)
    } else {
        (gray, elapsed)
    }
}

fn gray(data: &Compound) -> bool {
    matches!(data.get(&"Gray".into()), Some(Tag::Byte(value)) if *value != 0)
}

fn conversion_time(data: &Compound) -> i32 {
    match data.get(&"ConversionTime".into()) {
        Some(Tag::Int(value)) => *value,
        Some(Tag::Short(value)) => i32::from(*value),
        Some(Tag::Byte(value)) => i32::from(*value),
        _ => 0,
    }
}

/// Update the saved mob state from the current level's sea level and position.
pub fn tick(host: &mut dyn mod_api::ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let (Some(level), Some(position), Some(data)) =
        (entity.level(host), entity.position(host), entity.nbt(host))
    else {
        return;
    };
    let Some(sea_level) = level.sea_level(host) else {
        return;
    };
    // Java's 33-block depth boundary is inclusive: sea level - 33 converts gray.
    let deep = position[1] <= f64::from(sea_level.saturating_sub(PRESSURE_DEPTH));
    let current_gray = gray(&data);
    let current_time = conversion_time(&data);
    let (next_gray, next_time) = advance(current_gray, current_time, deep);
    if next_gray == current_gray && next_time == current_time {
        return;
    }
    let patch: Compound = [
        ("Gray".into(), Tag::Byte(if next_gray { 1 } else { 0 })),
        ("ConversionTime".into(), Tag::Int(next_time)),
    ]
    .into();
    if !entity.merge_nbt(host, patch) {
        return;
    }
    if next_gray != current_gray {
        crate::ocean_renderer::record_variant(host, entity);
    }
}

#[cfg(test)]
mod tests {
    use super::{advance, CONVERSION_TICKS};

    #[test]
    fn pressure_conversion_takes_one_hundred_ticks_in_either_direction() {
        let mut state = (false, 0);
        for _ in 0..CONVERSION_TICKS - 1 {
            state = advance(state.0, state.1, true);
        }
        assert_eq!(state, (false, 99));
        assert_eq!(advance(state.0, state.1, true), (true, 0));

        let mut state = (true, 0);
        for _ in 0..CONVERSION_TICKS - 1 {
            state = advance(state.0, state.1, false);
        }
        assert_eq!(state, (true, 99));
        assert_eq!(advance(state.0, state.1, false), (false, 0));
    }

    #[test]
    fn returning_to_current_pressure_cancels_and_resets_progress() {
        assert_eq!(advance(true, 40, true), (true, 0));
        assert_eq!(advance(false, 40, false), (false, 0));
    }
}
