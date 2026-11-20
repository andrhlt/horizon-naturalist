use super::{Game, lan_particles::QueuedParticle};
use horizon_client::items::ItemStack;
use horizon_client::particles::{ParticleOptions, ParticleType};

impl Game<'_> {
    pub(in crate::game) fn naturalist_hippo_eat(
        &mut self,
        index: usize,
        held: ItemStack,
        actor: [i32; 4],
    ) {
        use horizon_client::mobs::nav::naturalist_hippo_state as hippo;
        let mob = &mut self.mobs[index];
        let id = mob.id;
        let yaw = f64::from((mob.yaw + 90.0).to_radians());
        let position = mob.position.components();
        let at = [
            position[0] + yaw.cos(),
            position[1] + 0.6,
            position[2] + yaw.sin(),
        ];
        let spread = [mob.width(), mob.height(), mob.width()].map(|len| f64::from(len / 4.0));
        let silent = mob.silent;
        hippo::state_mut(&mut mob.ai.nav).eating = 10;
        mob.extra.set_main_hand_stack(Some(held));
        mob.swing();
        mob.extra.in_love = 600;
        mob.extra.love_cause = Some(actor);
        self.love_cause.insert(id, self.lan_acting);
        self.native_entity_enter_love_mode(id, hippo::KIND, self.lan_acting_uuid);
        self.mob_event_particles(id, 18);

        if !silent {
            self.audio
                .play("minecraft:entity.horse.eat", Some(position), 1.0, 1.0);
            self.audio
                .play("minecraft:block.wood.break", Some(position), 1.0, 1.0);
        }
        if let Some(state) = self.pack.assets.registry.default_state("minecraft:melon") {
            let block = at.map(|v| v.floor() as i32);
            if let Some((sprite, tint)) = self.particle_sprite(state, block) {
                let block_options = ParticleOptions::block(ParticleType::Block, sprite, tint);
                self.particles
                    .send_particles(&block_options, at, spread, 0.05, 100);
            }
            self.creatures.lan_particles.push(QueuedParticle {
                dimension: self.dim.id(),
                options: ParticleOptions::simple(ParticleType::Block),
                state: Some(state),
                at,
                spread,
                speed: [0.05; 3],
                count: 100,
                randomization: 0,
            });
        }
        let slice = self.item_particle(&ItemStack::new("minecraft:melon_slice", 1));
        self.particles.send_particles(&slice, at, spread, 0.05, 100);
        self.creatures.lan_particles.push(QueuedParticle {
            dimension: self.dim.id(),
            options: slice,
            state: None,
            at,
            spread,
            speed: [0.05; 3],
            count: 100,
            randomization: 0,
        });
    }
}
