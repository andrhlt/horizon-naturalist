use super::{Game, Vec3};
use horizon_client::mobs::{Mob, PlayerView, nav::naturalist_clam as clam};
use horizon_client::particles::{ParticleOptions, ParticleType};

fn within(mob: &Mob, player: &PlayerView, horizontal: f64, vertical: f64) -> bool {
    !player.creative && !player.spectator
        && mob.bounds().inflate(Vec3::new(horizontal, vertical, horizontal)).intersects(player.bounds)
}

impl Game<'_> {
    pub(super) fn step_naturalist_clams(&mut self) {
        if !self.mobs.iter().any(|mob| mob.kind.id == clam::KIND) {
            return;
        }
        let host = self.player_view();
        let guests = self.lan_guest_views();
        let actors = || host.iter().map(|player| (None, player))
            .chain(guests.iter().map(|(id, player)| (Some(*id), player)));
        let mut effects = Vec::new();
        let mut host_launches = 0;
        let mut guest_launches = Vec::new();
        let mut sounds = Vec::new();
        for mob in self.mobs.iter_mut().filter(|mob| mob.kind.id == clam::KIND && mob.alive()
            && !mob.no_ai && self.creatures.ticked.contains(&mob.id))
        {
            let shell = clam::state(&mob.ai.nav);
            let can_start = shell.snap_cooldown <= 1;
            let closed = can_start && !shell.open && actors().any(|(_, player)|
                !player.sneaking && within(mob, player, 1.0, 0.5));
            let treasure = can_start && shell.open && shell.has_treasure && actors().any(|(_, player)|
                !player.sneaking && within(mob, player, 3.5, 2.0));
            let result = clam::advance(mob, closed, treasure);
            let at = mob.position.components();
            if result.launch {
                effects.push([at[0], at[1] + 0.3, at[2]]);
                for (id, _) in actors().filter(|(_, player)| within(mob, player, 1.2, 0.6)) {
                    match id {
                        Some(id) => guest_launches.push(id),
                        None => host_launches += 1,
                    }
                }
            }
            if result.launch || result.slam {
                sounds.push(at);
            }
        }
        for at in effects {
            for (kind, count, speed) in [(ParticleType::BubbleColumnUp, 30, 0.1), (ParticleType::Bubble, 20, 0.05)] {
                let options = ParticleOptions::simple(kind);
                self.particles.send_particles(&options, at, [0.4, 0.2, 0.4], speed, count);
                self.lan_level_particles(None, &options, at, [0.4, 0.2, 0.4], speed, count, None);
            }
        }
        if host_launches > 0 && let Some(player) = &mut self.player {
            player.velocity.y += 2.2 * f64::from(host_launches);
            player.health.sync_velocity = true;
        }
        if !guest_launches.is_empty() && let Some(mut lan) = self.lan.take() {
            for id in guest_launches {
                self.guest_hit_by(&mut lan, id, |game| {
                    if let Some(player) = &mut game.player {
                        player.velocity.y += 2.2;
                    }
                });
            }
            self.lan = Some(lan);
        }
        for at in sounds {
            self.audio.play("naturalist:entity.clam.launch", Some(at), 1.0, 1.0);
        }
    }
}
