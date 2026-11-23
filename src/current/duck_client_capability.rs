fn duck_client_capability(game: &mut crate::game::Game<'_>) {
    if game.remote.is_none() {
        return;
    }
    use horizon_protocol::packets::{configuration::CustomPayload, play::Serverbound};
    game.send(Serverbound::CustomPayload(CustomPayload {
        channel: "minecraft:register".into(),
        data: b"naturalist:duck_bucket_atomic_v1\0naturalist:duck_pet_atomic_v1\0naturalist:rat_pet_atomic_v1\0naturalist:capybara_pet_atomic_v1".to_vec(),
    }));
}
