fn black_bear_swing(mob: &mut Mob) {
    if mob.kind.id == super::naturalist_bears::BLACK_BEAR && !mob.silent {
        mob.outbox.push(MobEvent::SoundId(
            "naturalist:entity.black_bear.attack".into(), mob.position.components(), 1.0, 1.0,
        ));
    }
}
