use super::{NaturalLevel, Species, mob_spawn_rules};

pub fn animal(level: &impl NaturalLevel, x: i32, y: i32, z: i32) -> bool {
    mob_spawn_rules(Species::Cow, level, x, y, z)
}
