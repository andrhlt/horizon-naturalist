//! Restore Anglerfish variant data when a filled bucket releases it.
use horizon_client::{Result, items::ItemStack};
use horizon_core::nbt::Tag;

pub(crate) fn restore(game: &mut super::Game<'_>, id: u32, stack: &ItemStack) -> Result<()> {
    game.mob_placed(id, stack)?;
    if let Some(variant) = stack
        .data()
        .and_then(|data| data.other("minecraft:bucket_entity_data"))
        .and_then(|data| data.get("Variant"))
        .cloned()
    {
        let raw = game
            .persist
            .raw
            .entry(id)
            .or_insert_with(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = raw {
            fields.insert("Variant".into(), variant);
        }
    }
    Ok(())
}
