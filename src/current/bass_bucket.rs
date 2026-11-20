//! Shared bass bucket entity restoration for player and dispenser pours.
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
        let variant = if let Some(value) = horizon_client::save::number(Some(&variant)) {
            Tag::String(
                [
                    "naturalist:bass",
                    "naturalist:bass_medium",
                    "naturalist:bass_large",
                ][value.rem_euclid(3) as usize]
                    .into(),
            )
        } else {
            variant
        };
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
