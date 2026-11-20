fn snail_appearance_remote_meta(mob: &mut Mob, values: &[(u8, i32, Value)]) {
    use horizon_client::mobs::naturalist_snail::{APPEARANCE_META, NORMAL_META, snail_mut};
    use horizon_core::geometry::Vec3;
    if mob.kind.id != "naturalist:snail" {
        return;
    }
    for (index, _, value) in values {
        match (*index, value) {
            (APPEARANCE_META, Value::Byte(appearance)) => {
                let snail = snail_mut(mob);
                snail.color = *appearance as u8 & 15;
                snail.hiding = *appearance & 16 != 0;
            }
            (NORMAL_META, Value::Floats(axes)) => {
                if let [x, y, z] = axes.as_slice() {
                    let normal = Vec3::new(f64::from(*x), f64::from(*y), f64::from(*z));
                    let surface = &mut snail_mut(mob).surface;
                    surface.normal = normal;
                    surface.attached = true;
                }
            }
            _ => {}
        }
    }
}
