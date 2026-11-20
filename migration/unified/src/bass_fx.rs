//! Server-side bass meal particle requests.
use mod_api::{
    Identifier, ItemStack, LevelRef, ParticleKey, ParticleOptions, Registrar, ServerHost,
};
use std::sync::OnceLock;

static ITEM_PARTICLE: OnceLock<ParticleKey> = OnceLock::new();

pub fn register(registrar: &mut dyn Registrar) {
    let _ = ITEM_PARTICLE.set(registrar.particle_key("minecraft:item"));
}

pub fn meal(host: &mut dyn ServerHost, level: LevelRef, positions: &[[f64; 3]]) {
    let (Some(key), Some(id)) = (
        ITEM_PARTICLE.get(),
        Identifier::parse_namespaced("naturalist:bass"),
    ) else {
        return;
    };
    let Some(item) = host.item_key(&id) else {
        return;
    };
    let options = ParticleOptions::item_stack(*key, ItemStack::of(item));
    for &position in positions {
        level.send_particles(host, &options, None, position, [0.0; 3], 0.05, 1);
    }
}
