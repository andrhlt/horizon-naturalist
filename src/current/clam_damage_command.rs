fn clam_damage_command(game: &mut Game<'_>, id: u32, amount: f32, damage: Damage) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::DamageKind;
    use horizon_core::nbt::Tag;
    let clam = game.mobs.iter().find(|mob| mob.id == id && mob.kind.id == "naturalist:clam");
    let _ = amount;
    if clam.is_some() && damage.kind != DamageKind::Drown {
        let open = game.persist.raw.get(&id).and_then(|tag| tag.get("NaturalistClamOpen"))
            .is_some_and(|tag| match tag { Tag::Byte(value) => *value != 0, Tag::Int(value) => *value != 0, _ => false });
        if !open { return std::ops::ControlFlow::Break(false); }
    }
    std::ops::ControlFlow::Continue(())
}
