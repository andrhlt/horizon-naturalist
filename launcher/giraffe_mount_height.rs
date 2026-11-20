fn giraffe_mount_height(mut seat: Vec3, mount: &horizon_client::mobs::Mob) -> Vec3 {
    use horizon_client::mobs::nav::naturalist_elephants;
    if mount.kind.id == "naturalist:giraffe" {
        seat.y = mount.position.y + f64::from(mount.height()) * 0.6;
    } else if naturalist_elephants::includes(&mount.kind.id) {
        let [x, y, z] = naturalist_elephants::seat(mount);
        seat = mount.position.add_vector(Vec3::new(x, y, z));
    }
    seat
}
