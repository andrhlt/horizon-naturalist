fn clam_velocity_fixture(connection: u32, velocity: Option<Vec3>) {
    if let Some(velocity) = velocity && velocity.y > 2.0 {
        clam_motion_fixture::BEFORE.with_borrow(|before| {
            if let Some((_, previous)) = before.iter().find(|(id, _)| *id == connection) {
                let change = velocity.y - previous;
                log::info!("Clam fixture launch motion {change}");
            }
        });
    }
}
