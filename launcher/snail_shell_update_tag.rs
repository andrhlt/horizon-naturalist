fn snail_shell_update_tag(entity: &ModBlockEntity, tag: &mut Option<Compound>) {
    if entity.id == super::naturalist_snail_shell::SHELL {
        *tag = Some(entity.nbt.clone());
    }
}
