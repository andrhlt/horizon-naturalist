fn bass_save(game: &Game<'_>, mob: &Mob, tag: &mut Tag) {
    if mob.kind.id != "naturalist:bass" {
        return;
    }
    if let (Some(ticks), Tag::Compound(fields)) = (naturalist::saved_cooldown(game, mob.id), tag) {
        fields.insert("EatCooldown".into(), Tag::Int(ticks));
    }
}
