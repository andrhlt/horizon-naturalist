// Derived from Horizon's Game::breed at commit 0a875da91dbb0fdcb7ba8c42c11341aa229af51a.
// Horizon is licensed under LGPL-3.0-or-later; see LICENSE-Horizon. The only
// behavior change records an admitted Deer child's inherited Naturalist variant.
fn deer_breed_overwrite(
    game: &mut Game<'_>, parent: u32, partner: u32,
    equine: Option<Box<horizon_client::mobs::equine::Offspring>>,
    parent_variant: Option<bool>, wolf_voice: Option<u8>,
    experience: horizon_client::mobs::BreedExperience,
) -> Option<u32> {
        let p = game.mobs.iter().find(|m| m.id == parent).cloned()?;
        // BabyEntitySpawnEvent: not for an egg (turtle, frog, sniffer).
        if !matches!(
            p.species(),
            Species::Turtle | Species::Frog | Species::Sniffer
        ) && mod_api::__private::listened(mod_api::HookId::BabyEntitySpawn)
        {
            let mule = matches!(
                (
                    p.species(),
                    game.mobs
                        .iter()
                        .find(|m| m.id == partner)
                        .map(|m| m.species())
                ),
                (Species::Horse, Some(Species::Donkey)) | (Species::Donkey, Some(Species::Horse))
            );
            let kind = if p.kind.id == "naturalist:butterfly" {
                "naturalist:caterpillar".to_owned()
            } else if mule {
                "minecraft:mule".to_owned()
            } else {
                super::native_hooks::mob_kind(&p)
            };
            let conn = game
                .love_cause
                .get(&parent)
                .or(game.love_cause.get(&partner))
                .copied();
            let player = conn.and_then(|c| game.killer_uuid(c));
            let player_entity_id = conn.and_then(|c| game.player_entity_id_by_connection(c));
            let mut cancel = false;
            game.native_baby_entity_spawn(
                parent,
                partner,
                player,
                player_entity_id,
                &kind,
                &mut cancel,
            );
            if cancel {
                breeding_credit(&mut game.love_cause, parent, partner);
                if let Some(q) = game.mobs.iter_mut().find(|m| m.id == partner) {
                    q.extra.age = 6000;
                    q.extra.in_love = 0;
                }
                return None;
            }
        }
        let q = game.mobs.iter().find(|m| m.id == partner).cloned();
        // Sniffer.spawnChildFromBreeding: an egg instead of a baby.
        if p.species() == Species::Sniffer {
            game.finalize_breeding(&p, partner, None, OrbOrder::BeforeChild);
            game.lay_sniffer_egg(&p);
            return None;
        }
        // TurtleBreedGoal.breed: the goal gave the mother her egg; only the
        // experience is left, and only with mob loot on.
        if matches!(p.species(), Species::Turtle | Species::Frog) {
            game.finalize_breeding(&p, partner, None, OrbOrder::BeforeChild);
            // One `new ExperienceOrb`, not split as `ExperienceOrb.award` does.
            game.award_breeding_experience(parent, [p.position.x, p.position.y, p.position.z]);
            return None;
        }
        let coin = if horizon_client::mobs::draws_parent_variant(p.species()) {
            match parent_variant {
                Some(choice) => choice,
                None => {
                    let parent = game.mobs.iter_mut().find(|m| m.id == parent)?;
                    parent.random.next_boolean()
                }
            }
        } else {
            // Other species either draw their own variants or do not need a coin.
            false
        };
        // The baby's variant; a rabbit draws its own (Rabbit.getBreedOffspring).
        let rabbit_variant = game.bred_rabbit_variant(parent, &p, q.as_ref());
        // Axolotl.getBreedOffspring: useRareVariant (one in 1200) makes it blue,
        // else either parent's variant. The draws come from the mother's random.
        let axolotl = if p.species() == Species::Axolotl {
            let other = q.as_ref().map_or(p.extra.variant, |q| q.extra.variant);
            game.mobs.iter_mut().find(|m| m.id == parent).map(|m| {
                if m.random.next_int_bound(1200).unwrap_or(1) == 0 {
                    // Axolotl.Variant.getRareSpawnVariant: the one blue, still a draw.
                    let _ = m.random.next_int_bound(1);
                    4
                } else if m.random.next_boolean() {
                    p.extra.variant
                } else {
                    other
                }
            })
        } else {
            None
        };
        // MushroomCow.getOffspringVariant, from the mother's random: the other
        // colour 1 time in 1024 when both parents share one, otherwise either
        // parent's colour by a coin flip.
        let mooshroom = if p.species() == Species::Mooshroom {
            let mate = q.as_ref().map_or(p.extra.variant, |q| q.extra.variant);
            let own = p.extra.variant;
            game.mobs.iter_mut().find(|m| m.id == parent).map(|m| {
                if own == mate && m.random.next_int_bound(1024).unwrap_or(1) == 0 {
                    if own == 1 { 0 } else { 1 }
                } else if m.random.next_boolean() {
                    own
                } else {
                    mate
                }
            })
        } else {
            None
        };
        // A horse and a donkey have a mule.
        let kind = match (p.species(), q.as_ref().map(|q| q.species())) {
            _ if p.kind.id == "naturalist:butterfly" => "naturalist:caterpillar".to_owned(),
            (Species::Horse, Some(Species::Donkey)) | (Species::Donkey, Some(Species::Horse)) => {
                "minecraft:mule".to_owned()
            }
            _ => p.kind.id.clone(),
        };
        let mut child = game.create_mob_reason(
            &kind,
            [p.position.x, p.position.y, p.position.z],
            p.kind.id != "naturalist:butterfly",
            SpawnSite::Raw,
            if p.kind.id == "naturalist:butterfly" { Some(0.0) } else { None },
        )?;
        'offspring: {
            let child = &mut child;
            child.extra.age = -24000;
            // No Mob.finalizeSpawn either: no follow range bonus, right-handed.
            spawning::bred_offspring(child);
            // getBreedOffspring creates the baby without finalizeSpawn: it keeps
            // the default (`classic`) sound variant.
            child.extra.sound_variant = None;
            // Fox.getBreedOffspring runs no finalizeSpawn: nothing in the
            // baby fox's mouth.
            if p.species() == Species::Fox {
                child.extra.set_main_hand(None);
            }
            // Fox.FoxBreedGoal.breed: the baby trusts the players who fed its
            // parents into love.
            if p.species() == Species::Fox {
                let parent_cause = game
                    .love_cause
                    .get(&parent)
                    .copied()
                    .and_then(|conn| game.killer_uuid(conn));
                let partner_cause = game
                    .love_cause
                    .get(&partner)
                    .copied()
                    .and_then(|conn| game.killer_uuid(conn));
                horizon_client::mobs::fox::trust_breeders(
                    child,
                    parent_cause.map(horizon_client::lan::playerdata::uuid_ints),
                    partner_cause.map(horizon_client::lan::playerdata::uuid_ints),
                );
            }
            // Goat.getBreedOffspring: GoatAi.initMemories, then the baby
            // screams if the parent a coin toss picks does, or on a 2% roll.
            if p.species() == Species::Goat {
                child.extra.lost_horns = [false; 2];
                horizon_client::mobs::finalize::goat_init_memories(child, &mut game.world.random);
                let pick_parent = game.world.random.next_boolean();
                let parent_screams = if pick_parent {
                    p.extra.screaming
                } else {
                    q.as_ref().unwrap_or(&p).extra.screaming
                };
                child.extra.screaming = parent_screams || game.world.random.next_double() < 0.02;
            }
            // Inheritance was drawn synchronously in the breeding goal,
            // before the parent's later same-tick aiStep rolls.
            if let Some(o) = equine {
                horizon_client::mobs::equine::apply_offspring(child, &o);
                break 'offspring;
            }
            // Camel.getBreedOffspring creates a camel with default attributes.
            if horizon_client::mobs::camel::is_camel(p.species()) {
                break 'offspring;
            }
            if p.species() == Species::Cat {
                // Cat.getBreedOffspring.
                let other = q.as_ref().unwrap_or(&p);
                child.extra.variant = if coin {
                    p.extra.variant
                } else {
                    other.extra.variant
                };
                if p.extra.tame {
                    child.extra.tame = true;
                    child.extra.owner = p.extra.owner;
                    let mixed = dye_mix(
                        &game.loop_context,
                        world_recipes!(game),
                        &mut game.world.random,
                        p.extra.color,
                        other.extra.color,
                    );
                    child.extra.color = mixed;
                }
                break 'offspring;
            }
            if p.species() == Species::Wolf {
                // Wolf.getBreedOffspring: a parent's variant; tame parents
                // give a tame baby with their collar colours mixed.
                let other = q.as_ref().unwrap_or(&p);
                child.extra.variant = if coin {
                    p.extra.variant
                } else {
                    other.extra.variant
                };
                if p.extra.tame {
                    horizon_client::mobs::tame::set_tame(child, true);
                    child.extra.owner = p.extra.owner;
                    let mixed = dye_mix(
                        &game.loop_context,
                        world_recipes!(game),
                        &mut game.world.random,
                        p.extra.color,
                        other.extra.color,
                    );
                    child.extra.color = mixed;
                }
                child.extra.sound_variant = wolf_voice.or_else(|| game.bred_wolf_voice(parent));
                break 'offspring;
            }
            // Panda.getBreedOffspring: genes from both parents.
            if p.species() == Species::Panda {
                horizon_client::mobs::panda::inherit(child, &p, q.as_ref().unwrap_or(&p));
                break 'offspring;
            }
            // Axolotl.getBreedOffspring: the drawn variant, and the baby is persistent.
            if let Some(variant) = axolotl {
                child.extra.variant = variant;
                child.persistent = true;
                break 'offspring;
            }
            // Nautilus.getBreedOffspring: a tame parent's baby is tame too.
            if p.species() == Species::Nautilus && p.extra.tame {
                child.extra.tame = true;
                child.extra.owner = p.extra.owner;
                child.persistent = true;
            }
            // Offspring inherit a parent's variant (pigs/cows/chickens) or mixed wool.
            child.extra.variant = p.extra.variant;
            // Fox, Pig, Cow and Chicken.getBreedOffspring: either parent's variant.
            if horizon_client::mobs::draws_parent_variant(p.species())
                && p.species() != Species::Cat
                && !coin
            {
                child.extra.variant = q.as_ref().map_or(p.extra.variant, |q| q.extra.variant);
            }
            // The rabbit's own draw above.
            if let Some(variant) = rabbit_variant {
                child.extra.variant = variant;
            }
            if let Some(variant) = mooshroom {
                child.extra.variant = variant;
            }
            // Sheep.getBreedOffspring: the lamb's wool mixes both parents' dyes.
            if p.species() == Species::Sheep {
                let other = q.as_ref().unwrap_or(&p);
                let mixed = dye_mix(
                    &game.loop_context,
                    world_recipes!(game),
                    &mut game.world.random,
                    p.extra.color,
                    other.extra.color,
                );
                child.extra.color = mixed;
            }
        }
        // The goal already drew this coin on the parent stream. Keep it local to
        // this call and write a child's variant only after admission succeeds.
        let inherited = if p.kind.id == "naturalist:deer" {
            let source = if coin { partner } else { parent };
            Some(game.persist.raw.get(&source).and_then(|tag| tag.get("Variant")).cloned()
                .unwrap_or_else(|| horizon_core::nbt::Tag::String("naturalist:deer".into())))
        } else if p.kind.id == "naturalist:duck" {
            let source = if coin { partner } else { parent };
            Some(game.persist.raw.get(&source).and_then(|tag|tag.get("Variant")).cloned()
                .unwrap_or_else(||horizon_core::nbt::Tag::String("naturalist:duck".into())))
        } else if p.kind.id == "naturalist:butterfly" {
            Some(horizon_core::nbt::Tag::String("naturalist:caterpillar".into()))
        } else { None };
        let commit_variant = |game: &mut Game<'_>, joined: Option<u32>| {
            if let (Some(id), Some(variant)) = (joined, inherited) {
                let raw = game.persist.raw.entry(id).or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
                if let horizon_core::nbt::Tag::Compound(fields) = raw { fields.insert("Variant".into(), variant); }
            }
            joined
        };
        let order = if horizon_client::mobs::orb_after_child(p.species()) {
            OrbOrder::AfterChild
        } else {
            OrbOrder::BeforeChild
        };
        game.finalize_breeding(&p, partner, Some(&child), order);
        let at = [p.position.x, p.position.y, p.position.z];
        match order {
            OrbOrder::AfterChild => {
                let joined = game.insert_spawned_mob(child);
                game.mob_event_particles(parent, 18);
                game.breeding_orb(parent, at, experience);
                commit_variant(game, joined)
            }
            OrbOrder::BeforeChild => {
                game.breeding_orb(parent, at, experience);
                let joined = game.insert_spawned_mob(child);
                commit_variant(game, joined)
            }
        }
    }
