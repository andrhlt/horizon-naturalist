fn catfish_save(tag: &mut Tag, mob: &Mob) {
    use horizon_client::mobs::nav::naturalist_catfish_state as catfish;
    if matches!(mob.kind.id.as_str(), "naturalist:great_white_shark" | "naturalist:catfish" | "naturalist:anglerfish" | "naturalist:alligator" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake") && let Tag::Compound(fields) = tag {
        if mob.kind.id == "naturalist:alligator" {
            fields.insert("HasEgg".into(), Tag::Byte(i8::from(mob.extra.flag)));
        }
        fields.insert("HuntingCooldown".into(), Tag::Int(catfish::cooldown(&mob.ai.nav)));
    }
}
