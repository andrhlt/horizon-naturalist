fn bear_items(game: &mut Game<'_>) {
    use horizon_client::mobs::naturalist_bears as bear;
    use horizon_core::geometry::{Aabb, Vec3};
    let griefing = game.ui.rules.bool("mob_griefing");
    let ticking = game.entity_ticking_area();
    let mut spit = Vec::new();
    let mut raids = Vec::new();
    for i in 0..game.mobs.len() {
        let mob = &game.mobs[i];
        if !bear::is_bear(&mob.kind.id) {
            continue;
        }
        if mob.extra.anger > 0 && mob.ai.target.is_none()
            && let Some(uuid) = bear::state(&mob.ai.nav).loaded_anger_target
            && let Some(Who::Mob(id)) = game.who_by_uuid(uuid)
            && game.mobs.iter().any(|other| {
                other.id == id && other.id != mob.id && other.alive()
                    && (mob.extra.owner.is_none() || mob.extra.owner != other.extra.owner)
            })
        {
            let mob = &mut game.mobs[i];
            mob.extra.angry_at = Some(Who::Mob(id));
            mob.ai.target = Some(Who::Mob(id));
            bear::state_mut(&mut mob.ai.nav).loaded_anger_target = None;
        }
        let mob = &mut game.mobs[i];
        let at = mob.position;
        let angle = f64::from(mob.yaw.to_radians());
        let status = bear::state_mut(&mut mob.ai.nav);
        if let Some(stack) = status.pending_spit.take() {
            spit.push((stack, [at.x - angle.sin(), at.y + 1.0, at.z + angle.cos()], !mob.silent));
        }
        if let Some(target) = status.harvest.take() {
            raids.push((i, target));
        }
        status.food_position = None;
        if !griefing || status.sleeping || !mob.alive() || mob.is_baby() || mob.extra.main_hand.is_some()
            || !mob.extra.pick_up_loot || !ticking(at.x.floor() as i32, at.z.floor() as i32)
        {
            continue;
        }
        let scope = mob.bounds().inflate(Vec3::new(8.0, 2.0, 8.0));
        let nearby = game.dropped.iter().position(|item| {
            scope.intersects(Aabb::new(item.position, item.position.add_vector(Vec3::new(0.25, 0.25, 0.25))))
                && mob.kind.food.iter().any(|food| food == item.stack.item.as_ref())
        });
        let Some(item) = nearby else { continue };
        let item_pos = game.dropped[item].position;
        bear::state_mut(&mut mob.ai.nav).food_position = Some(item_pos);
        let pickup = mob.bounds().inflate(Vec3::new(1.0, 0.0, 1.0));
        if pickup.intersects(Aabb::new(item_pos, item_pos.add_vector(Vec3::new(0.25, 0.25, 0.25)))) {
            let stack = game.remove_dropped(item).stack;
            let mob = &mut game.mobs[i];
            mob.extra.set_main_hand_stack(Some(stack));
            horizon_client::mobs::finalize::set_drop_chance(mob, horizon_client::mobs::finalize::MAINHAND, 2.0);
        }
    }
    for (index, pos) in raids {
        bear_harvest(game, index, pos);
    }
    for (stack, at, sound) in spit {
        game.spawn_item_own(stack, at, 80);
        if sound {
            game.audio.play("naturalist:entity.bear.spit", Some(at), 1.0, 1.0);
        }
    }

    fn bear_harvest(game: &mut Game<'_>, index: usize, pos: [i32; 3]) {
        use horizon_client::{containers::BlockEntity, mobs::naturalist_bears::{self as bear, FoodSource}};
        use horizon_core::random::RandomSource;
        let mob = &game.mobs[index];
        if !mob.alive() || mob.extra.main_hand.is_some() || !game.ui.rules.bool("mob_griefing") {
            return;
        }
        let id = mob.id;
        let at = mob.position.components();
        let silent = mob.silent;
        let Some(source) = bear::food_source(&game.mob_world_base(&[]), pos, &mob.kind.food) else { return };
        let [x, y, z] = pos;
        let state = game.world.block(x, y, z);
        let key = (x, y, z);
        let sound = match source {
            FoodSource::Berries(age) => {
                let count = 1 + game.world.random.next_int_bound(2).unwrap_or(0) as u32 + u32::from(age == 3);
                game.pop_resource(pos, ItemStack::new("minecraft:sweet_berries", count));
                if let Some(state) = game.with_props(state, &[("age", "1")]) {
                    let _ = game.level_set(pos, state);
                }
                "minecraft:block.sweet_berry_bush.pick_berries"
            }
            FoodSource::Hive => {
                for stack in game.sequence_loot("minecraft:harvest/beehive", "minecraft:harvest/beehive", None) {
                    game.pop_resource(pos, stack);
                }
                if let Some(state) = game.with_props(state, &[("honey_level", "0")]) {
                    let _ = game.level_set(pos, state);
                }
                "minecraft:block.beehive.shear"
            }
            FoodSource::Campfire(slot) => {
                if let Some(BlockEntity::Campfire(campfire)) = game.world.block_entities.get_mut(&key)
                    && let Some(stack) = campfire.items[slot].take()
                {
                    campfire.progress[slot] = 0;
                    campfire.total[slot] = 0;
                    game.campfire_drop(key, stack);
                    game.mark_block_entity_updated(key);
                    game.world.modified.insert((x >> 4, z >> 4));
                }
                return;
            }
        };
        game.mobs[index].swing();
        if !silent {
            game.audio.play(sound, Some(at), 1.0, 1.0);
        }
        game.post_block_game_event("minecraft:block_change", pos, Some(Who::Mob(id)), Some(state));
    }
}
