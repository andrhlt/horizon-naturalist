fn ant_hill_spill(entity: &crate::containers::ModBlockEntity, drops: &mut Vec<EngineStack>) {
    if entity.id() != "naturalist:ant_hill" {
        return;
    }
    if let Some(horizon_core::nbt::Tag::List(items)) = entity.nbt.get(&"Storage".into()) {
        drops.extend(items.iter().take(9).filter_map(horizon_client::save::item_from_nbt));
    }
}
