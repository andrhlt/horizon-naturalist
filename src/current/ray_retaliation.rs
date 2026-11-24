//! Preserve Ray retaliation when its native hurt-by goal misses a recorded attacker.

use mod_api::ServerHost;

pub(crate) fn tick(host: &mut dyn ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let Some(mob) = host.mob(entity) else {
        return;
    };
    if mob.target(host).is_some() {
        return;
    }
    let Some(attacker) = host
        .living(entity)
        .and_then(|living| living.last_attacker(host))
    else {
        return;
    };
    if attacker.id() == id {
        return;
    }
    let Some(attacker_living) = host.living(attacker) else {
        return;
    };
    let _ = mob.set_target(host, Some(attacker_living));
}
