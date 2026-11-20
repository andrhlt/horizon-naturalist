fn zebra_inherited_jump(_result: (), child: &mut Mob) {
    if child.kind.id == super::naturalist_zebra::KIND {
        let jump = child.extra.vector[0];
        set_base(child, Attr::JumpStrength, jump);
    }
}
