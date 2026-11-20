fn ant_hill_entity(block: &str, entity: &mut Option<(String, Compound)>) {
    if block == super::naturalist_ant_hill::HILL {
        let mut data = Compound::default();
        data.insert("Storage".into(), horizon_world::nbt::Tag::List(Vec::new()));
        *entity = Some((block.to_owned(), data));
    }
}
