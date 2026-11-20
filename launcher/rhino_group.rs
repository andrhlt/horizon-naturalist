fn rhino_group(kind: &str, group: &mut Option<crate::mobs::finalize::Group>) {
    use crate::mobs::finalize::{Ageable, Group};
    if matches!(kind, "naturalist:rhino" | "naturalist:bear" | "naturalist:black_bear") && group.is_none() {
        *group = Some(Group::Ageable(Ageable::chance(1.0)));
    }
}
