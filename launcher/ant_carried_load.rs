fn ant_carried_load(
    game: &mut Game<'_>,
    kind: &str,
    tag: &Tag,
    uuid: Option<[i32; 4]>,
) -> std::ops::ControlFlow<std::result::Result<(), Tag>> {
    if kind != "naturalist:carried_food" {
        return std::ops::ControlFlow::Continue(());
    }
    let mut tag = tag.clone();
    if let Tag::Compound(fields) = &mut tag {
        fields.insert("Age".into(), Tag::Short(i16::MIN));
    }
    game.load_item(tag, uuid);
    std::ops::ControlFlow::Break(Ok(()))
}
