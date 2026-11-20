fn ant_hill_spill(level: &mut GameLevel<'_>, pos: BlockPos, old: BlockStateId, side_effects: bool) {
    use horizon_world::nbt::Tag;
    if !side_effects || level.facts.get(old).name.as_ref() != super::naturalist_ant_hill::HILL {
        return;
    }
    let Some(BlockEntity::Mod(hill)) = level.world.block_entities.get(&(pos.x, pos.y, pos.z))
    else {
        return;
    };
    let storage = if let Some(Tag::List(items)) = hill.nbt.get(&"Storage".into()) {
        items.as_slice()
    } else {
        &[]
    };
    let items = (0..9)
        .map(|slot| {
            storage
                .get(slot)
                .and_then(horizon_client::save::item_from_nbt)
        })
        .collect();
    level.drop_lost_contents(pos, items);
}
