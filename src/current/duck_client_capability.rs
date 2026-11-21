fn duck_client_capability(game: &mut crate::game::Game<'_>) {
    if game.remote.is_none() { return; }
    use horizon_protocol::packets::{play::Serverbound, configuration::CustomPayload};
    game.send(Serverbound::CustomPayload(CustomPayload {
        channel: "minecraft:register".into(),
        data: b"naturalist:duck_bucket_atomic_v1".to_vec(),
    }));
}
