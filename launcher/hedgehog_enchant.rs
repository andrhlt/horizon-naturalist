fn hedgehog_enchant(item: &str, id: &str) -> std::ops::ControlFlow<bool> {
    if item != "naturalist:hedgehog" {
        return std::ops::ControlFlow::Continue(());
    }
    std::ops::ControlFlow::Break(matches!(id,
        "minecraft:unbreaking" | "minecraft:thorns" | "minecraft:punch" |
        "minecraft:flame" | "minecraft:looting" | "minecraft:loyalty"
    ))
}
