fn vulture_damage_immunity(entity: i32, kind: &str, damage_type: &str, amount: &mut f32) {
    if kind == "naturalist:vulture" && damage_type == "minecraft:cactus" {
        *amount = 0.0;
    }
}
