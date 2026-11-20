fn giant_isopod_load(mob: &mut Mob, tag: &Tag) {
    if mob.kind.id == "naturalist:giant_isopod" {
        mob.extra.from_bucket = flag(tag, "FromBucket");
    }
}
