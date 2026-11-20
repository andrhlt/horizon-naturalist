fn deer_chunk_spawn(
    finalizer: &StructureMobFinalizer,
    kind: &str,
    position: Vec3,
    yaw: f32,
    biome: &str,
    random: &mut dyn PieceRandom,
    group: &mut Option<Group>,
) -> std::ops::ControlFlow<Option<Compound>> {
    use crate::mobs::{
        finalize::{Ageable, baby_start_age, set_age},
        naturalist_spawn_variants::generation_variant,
    };
    use std::ops::ControlFlow;

    if kind == "naturalist:bird" {
        fn spawn_bird(
            finalizer: &StructureMobFinalizer,
            position: Vec3,
            yaw: f32,
            biome: &str,
            random: &mut impl RandomSource,
            group: &mut Option<Group>,
        ) -> Option<Compound> {
            let data = finalizer.data()?;
            let mut bird = Mob::new(1, data.types.get("naturalist:bird")?.clone(), position, yaw);
            let variant = generation_variant(
                &finalizer.root, "naturalist/bird_variant", biome, &data.biome_tags, random,
            ).unwrap_or_else(|| "naturalist:american_robin".to_owned());
            let age = match group.get_or_insert_with(|| Group::Ageable(Ageable::babies(true))) {
                Group::Ageable(age) => age,
                _ => return None,
            };
            if age.should_spawn_baby && age.size > 0 && random.next_float() <= age.baby_chance {
                let start = baby_start_age(bird.species());
                set_age(&mut bird, start);
            }
            age.size += 1;
            crate::mobs::naturalist_deer_spawn::finish(&mut bird, random);
            let mut saved = tree(&[(bird, None)], 0, &finalizer.dimension);
            saved.insert("Variant".into(), Tag::String(variant.as_str().into()));
            Some(saved)
        }
        let random = random.as_any();
        return if let Some(random) = random.downcast_mut::<LegacyRandom>() {
            ControlFlow::Break(spawn_bird(finalizer, position, yaw, biome, random, group))
        } else if let Some(random) = random.downcast_mut::<XoroshiroRandom>() {
            ControlFlow::Break(spawn_bird(finalizer, position, yaw, biome, random, group))
        } else {
            ControlFlow::Continue(())
        };
    }

    if kind != "naturalist:deer" {
        return ControlFlow::Continue(());
    }

    fn spawn(
        finalizer: &StructureMobFinalizer,
        kind: &str,
        position: Vec3,
        yaw: f32,
        biome: &str,
        random: &mut impl RandomSource,
        group: &mut Option<Group>,
    ) -> Option<Compound> {
        let data = finalizer.data()?;
        let mut deer = Mob::new(1, data.types.get(kind)?.clone(), position, yaw);
        let variant = generation_variant(
            &finalizer.root, "naturalist/deer_variant", biome, &data.biome_tags, random,
        ).unwrap_or_else(|| kind.to_owned());
        let age = match group.get_or_insert_with(|| Group::Ageable(Ageable::babies(true))) {
            Group::Ageable(age) => age,
            _ => return None,
        };
        if age.should_spawn_baby && age.size > 0 && random.next_float() <= age.baby_chance {
            let start_age = baby_start_age(deer.species());
            set_age(&mut deer, start_age);
        }
        age.size += 1;
        crate::mobs::naturalist_deer_spawn::finish(&mut deer, random);
        let mut fields = Compound::new();
        fields.insert("Variant".into(), Tag::String(variant.as_str().into()));
        Some(tree(&[(deer, Some(Tag::Compound(fields)))], 0, &finalizer.dimension))
    }

    let random = random.as_any();
    if let Some(random) = random.downcast_mut::<LegacyRandom>() {
        ControlFlow::Break(spawn(finalizer, kind, position, yaw, biome, random, group))
    } else if let Some(random) = random.downcast_mut::<XoroshiroRandom>() {
        ControlFlow::Break(spawn(finalizer, kind, position, yaw, biome, random, group))
    } else {
        ControlFlow::Continue(())
    }
}
