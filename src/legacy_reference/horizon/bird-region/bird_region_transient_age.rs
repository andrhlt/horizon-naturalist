fn bird_region_transient_age(mut saved: Compound, mobs: &[(Mob, Option<Tag>)], i: usize) -> Compound {
    if let Some((mob, _)) = mobs.get(i) {
        if mob.kind.id == "naturalist:bird" { saved.insert("Age".into(), Tag::Int(mob.extra.age)); }
    }
    saved
}
