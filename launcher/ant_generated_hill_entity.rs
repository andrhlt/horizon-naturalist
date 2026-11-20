fn ant_generated_hill_entity(block: &str, nbt: &Compound) -> std::ops::ControlFlow<Option<BlockEntity>> {
    use std::ops::ControlFlow;
    if block != "naturalist:ant_hill" {
        return ControlFlow::Continue(());
    }
    let mut tag = nbt.clone();
    for axis in ["x", "y", "z"] {
        tag.insert(axis.into(), Tag::Int(0));
    }
    ControlFlow::Break(
        crate::save::decode_block_entity(&Tag::Compound(tag)).map(|(_, entity)| entity),
    )
}
