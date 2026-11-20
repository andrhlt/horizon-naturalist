fn deer_breed_variant(game: &mut crate::game::Game<'_>, parent: u32, partner: u32, child: Option<u32>) {
    let Some(coin) = crate::game::naturalist_deer_goals::take_breed_coin(parent) else { return; };
    let Some(child) = child else { return; };
    let source = if coin { parent } else { partner };
    let Some(variant) = game.persist.raw.get(&source).and_then(|tag| tag.get("Variant")).cloned() else { return; };
    let raw = game.persist.raw.entry(child).or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
    if let horizon_core::nbt::Tag::Compound(fields) = raw { fields.insert("Variant".into(), variant); }
}
