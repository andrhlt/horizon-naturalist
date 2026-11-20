fn clam_save(tag: &mut Tag, mob: &Mob) {
    use horizon_client::mobs::nav::naturalist_clam as clam;
    if mob.kind.id == clam::KIND && let Tag::Compound(fields) = tag {
        let shell = clam::state(&mob.ai.nav);
        fields.insert("Open".into(), Tag::Byte(i8::from(shell.open)));
        fields.insert("HasTreasure".into(), Tag::Byte(i8::from(shell.has_treasure)));
    }
}
