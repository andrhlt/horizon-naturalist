fn snail_shell_lan_fixture(
    game: &Game<'_>,
    line: &str,
) -> std::ops::ControlFlow<Result<Option<Value>>> {
    use horizon_client::containers::BlockEntity;
    use std::ops::ControlFlow;

    if line != "naturalist_snail_shell_lan" || !game.options.headless {
        return ControlFlow::Continue(());
    }
    let pos = [8, 100, 8];
    let state = game.world.block(pos[0], pos[1], pos[2]);
    let block = game.pack.assets.registry.state(state);
    let properties = block.map(|block| &block.properties);
    let entity = match game.world.block_entities.get(&(pos[0], pos[1], pos[2])) {
        Some(BlockEntity::Mod(entity)) => Some(entity),
        _ => None,
    };
    let mut vertices = Vec::new();
    game.mod_block_entity_geometry(
        [([8, 100, 8], state.0)].iter(),
        glam::DVec3::new(10.5, 102.0, 6.5),
        0.0,
        &mut vertices,
    );
    let max_y = vertices.iter().map(|v| v.pos[1]).reduce(f32::max);
    ControlFlow::Break(Ok(Some(serde_json::json!({
        "remote": game.remote.is_some(),
        "block": block.map(|block| &block.name),
        "color": properties.and_then(|props| props.get("color")),
        "rotation": properties.and_then(|props| props.get("rotation")),
        "entity": entity.map(|entity| &entity.id),
        "flower": entity.and_then(|entity| entity.nbt.get(&"Flower".into()))
            .and_then(horizon_client::save::item_from_nbt)
            .map(|item| serde_json::json!([item.item.as_ref(), item.count])),
        "vertices": vertices.len(),
        "max_y": max_y,
    }))))
}
