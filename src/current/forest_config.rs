use std::sync::atomic::{AtomicU8, Ordering};

const FOREST_FOX: u8 = 1;
const FOREST_RABBIT: u8 = 2;
static DISABLED: AtomicU8 = AtomicU8::new(0);

pub fn default_config() -> &'static str {
    "[disable_mobs.land_mobs]\nforest_fox_removed = false\nforest_rabbit_removed = false\n"
}

pub fn update(text: &str) {
    let mut flags = 0;
    for line in text.lines().map(str::trim) {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let enabled = value.trim().eq_ignore_ascii_case("true");
        match key.trim() {
            "forest_fox_removed" if enabled => flags |= FOREST_FOX,
            "forest_rabbit_removed" if enabled => flags |= FOREST_RABBIT,
            _ => {}
        }
    }
    DISABLED.store(flags, Ordering::Release);
}

pub fn remove_forest_fox() -> bool {
    DISABLED.load(Ordering::Acquire) & FOREST_FOX != 0
}

pub fn remove_forest_rabbit() -> bool {
    DISABLED.load(Ordering::Acquire) & FOREST_RABBIT != 0
}

pub fn filter(entries: &mut Vec<mod_api::SpawnEntry>) {
    let fox = remove_forest_fox();
    let rabbit = remove_forest_rabbit();
    entries.retain(|entry| {
        !(entry.entity == "minecraft:fox" && fox) && !(entry.entity == "minecraft:rabbit" && rabbit)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_land_mob_removals_and_reloads_them() {
        update(
            "[disable_mobs.land_mobs]\nforest_fox_removed = true\nforest_rabbit_removed = false\n",
        );
        assert!(remove_forest_fox());
        assert!(!remove_forest_rabbit());
        update(default_config());
        assert!(!remove_forest_fox());
        assert!(!remove_forest_rabbit());
    }
}
