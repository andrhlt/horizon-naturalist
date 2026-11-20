fn blobfish_save(tag: &mut Tag, mob: &Mob) {
    use horizon_client::mobs::nav::naturalist_blobfish_state as blobfish;
    if mob.kind.id == "naturalist:blobfish" && let Tag::Compound(fields) = tag {
        fields.insert("Gray".into(), Tag::Byte(i8::from(blobfish::gray(&mob.ai.nav))));
        fields.insert("ConversionTime".into(), Tag::Int(blobfish::conversion(&mob.ai.nav)));
    }
}
