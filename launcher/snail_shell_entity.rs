fn snail_shell_entity(block: &str, entity: &mut Option<(String, Compound)>) {
    if block == super::naturalist_snail_shell::SHELL {
        *entity = Some((block.to_owned(), Compound::default()));
    }
}
