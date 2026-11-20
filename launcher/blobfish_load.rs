fn blobfish_load(mob: &mut Mob, tag: &Tag) {
    use crate::mobs::nav::naturalist_blobfish_state as blobfish;
    if mob.kind.id == "naturalist:blobfish" {
        blobfish::set_gray(&mut mob.ai.nav, flag(tag, "Gray"));
        blobfish::set_conversion(&mut mob.ai.nav, number(tag.get("ConversionTime")).unwrap_or(0) as i32);
    }
}
