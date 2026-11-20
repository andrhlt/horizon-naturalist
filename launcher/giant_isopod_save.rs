fn giant_isopod_save(tag: &mut Tag, mob: &Mob) {
    if mob.kind.id == "naturalist:giant_isopod" && let Tag::Compound(fields) = tag {
        fields.insert("FromBucket".into(), Tag::Byte(i8::from(mob.extra.from_bucket)));
    }
}
