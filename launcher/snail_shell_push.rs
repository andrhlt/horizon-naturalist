fn snail_shell_push(block: &NewBlock, row: &mut J) {
    if block.decl.id == "naturalist:snail_shell" {
        row.set("push", J::Str("POPPED".into()));
    }
}
