fn firefly_receive_baby(can_be_a_baby: bool, mob: &Mob) -> bool {
    can_be_a_baby || mob.kind.id == "naturalist:firefly"
}
