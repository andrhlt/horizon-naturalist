fn catfish_load(mob: &mut Mob, tag: &Tag) {
    use crate::mobs::nav::naturalist_catfish_state as catfish;
    if matches!(mob.kind.id.as_str(), "naturalist:great_white_shark" | "naturalist:catfish" | "naturalist:anglerfish" | "naturalist:alligator" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake") {
        if mob.kind.id == "naturalist:alligator" {
            mob.extra.flag = number(tag.get("HasEgg")).unwrap_or(0) != 0;
        }
        catfish::set_cooldown(&mut mob.ai.nav, number(tag.get("HuntingCooldown")).unwrap_or(0) as i32);
    }
}
