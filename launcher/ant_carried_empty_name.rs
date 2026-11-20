fn ant_carried_empty_name(id: &str) -> std::ops::ControlFlow<String> {
    if id == "naturalist:carried_food" {
        return std::ops::ControlFlow::Break("block.minecraft.air".into());
    }
    std::ops::ControlFlow::Continue(())
}
