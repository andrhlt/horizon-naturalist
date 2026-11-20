fn insect_loaded_nbt(mob: &mut Mob, tag: &Tag) {
    if mob.kind.id == "naturalist:mole" {
        let phase = number(tag.get("MoleState")).unwrap_or(0) as i32;
        crate::mobs::goals::naturalist_mole_goals::load_phase(mob, phase);
    }
    if mob.kind.id == "naturalist:ant" {
        let ant = crate::mobs::naturalist_ant::state_mut(mob);
        ant.cooldown = number(tag.get("HillCooldown")).unwrap_or(0) as i32;
        if let Some(Tag::IntArray(uuid)) = tag.get("angry_at") {
            ant.anger_actor = uuid.as_slice().try_into().ok();
        }
    }
    if mob.kind.id == "naturalist:hedgehog" {
        crate::mobs::naturalist_hedgehog::load_enchantments(mob, tag.get("ThrowEnchantments"));
    }
    if mob.kind.id == "naturalist:snail" {
        let snail = crate::mobs::naturalist_snail::snail_mut(mob);
        snail.color = number(tag.get("Color")).filter(|color| (0..=15).contains(color)).unwrap_or(0) as u8;
        snail.has_egg = flag(tag, "HasEgg");
        let normal_component = |name| match tag.get(name) {
            Some(Tag::Float(value)) => f64::from(f32::from_bits(*value)),
            Some(Tag::Double(value)) => f64::from_bits(*value),
            _ => 0.0,
        };
        if tag.get("ClimbNormalY").is_some() {
            let normal = horizon_core::geometry::Vec3::new(
                normal_component("ClimbNormalX"),
                normal_component("ClimbNormalY"),
                normal_component("ClimbNormalZ"),
            );
            if normal.length_squared() > 1.0e-6 {
                snail.surface.normal = normal.normalize();
            }
        }
    }
    let kind = mob.kind.id.as_str();
    if matches!(kind, "naturalist:butterfly" | "naturalist:firefly") {
        mob.extra.age = number(tag.get("Age")).unwrap_or(0) as i32;
    }
    if matches!(kind, "naturalist:ant" | "naturalist:hedgehog" | "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:desert_scorpion" | "naturalist:snail") {
        mob.persistent |= flag(tag, "FromHand");
    }
}
