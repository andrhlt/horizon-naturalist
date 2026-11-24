//! Shared goal wiring and join-time defaults for the target species.
use mod_api::{GoalSpec, MobFilter};

pub fn defaults(kind: &str) -> Option<&'static str> {
    match kind {
        "naturalist:komodo_dragon" => Some("naturalist:komodo_dragon"),
        "naturalist:lizard" | "naturalist:lizard_tail" => Some("naturalist:green"),
        "naturalist:mole" => Some("naturalist:mole"),
        "naturalist:tortoise" => Some("naturalist:brown"),
        _ => None,
    }
}

pub fn register_goals(host: &mut dyn mod_api::ServerHost, entity: mod_api::EntityRef, kind: &str) {
    let Some(mob) = host.mob(entity) else { return; };
    let mut add = |priority, spec: GoalSpec| mob.add_goal(host, priority, &spec);
    add(0, GoalSpec::Float);
    match kind {
        "naturalist:komodo_dragon" => {
            add(1, GoalSpec::Panic { speed: 1.2, water_range: None });
            add(2, GoalSpec::MeleeAttack { speed: 1.4, follow_unseen: true });
            add(3, GoalSpec::Breed { speed: 1.0 });
            add(4, GoalSpec::Tempt { speed: 1.0, items: komodo_food(), can_scare: true });
            add(5, GoalSpec::Stroll { speed: 1.0, interval: 120, water_avoiding: None, swim: false, flying: false });
        }
        "naturalist:lizard" => {
            add(1, GoalSpec::Panic { speed: 1.2, water_range: None });
            add(2, GoalSpec::SitWhenOrdered);
            add(3, GoalSpec::Tempt { speed: 1.0, items: vec!["minecraft:spider_eye".into()], can_scare: true });
            add(4, GoalSpec::Stroll { speed: 1.0, interval: 120, water_avoiding: None, swim: false, flying: false });
        }
        "naturalist:tortoise" => {
            add(1, GoalSpec::Panic { speed: 1.2, water_range: Some(5) });
            add(2, GoalSpec::Breed { speed: 1.0 });
            add(3, GoalSpec::Tempt { speed: 1.0, items: vec![
                "minecraft:bamboo".into(), "minecraft:dandelion".into(),
                "minecraft:brown_mushroom".into(), "minecraft:cactus".into(),
            ], can_scare: true });
            add(4, GoalSpec::FollowParent { speed: 1.1 });
            add(5, GoalSpec::Stroll { speed: 1.0, interval: 120, water_avoiding: None, swim: false, flying: false });
        }
        "naturalist:mole" => {
            add(1, GoalSpec::Panic { speed: 2.0, water_range: None });
            add(3, GoalSpec::Breed { speed: 1.0 });
            add(4, GoalSpec::Tempt { speed: 1.25, items: vec!["minecraft:carrot".into()], can_scare: false });
            add(5, GoalSpec::FollowParent { speed: 1.25 });
            add(6, GoalSpec::Stroll { speed: 1.0, interval: 120, water_avoiding: Some(0.0), swim: false, flying: false });
            add(7, GoalSpec::AvoidEntity {
                what: MobFilter::Player,
                max_dist: 6.0,
                walk: 1.0,
                sprint: 1.25,
            });
        }
        _ => return,
    }
}

fn komodo_food() -> Vec<String> {
    ["minecraft:beef", "minecraft:chicken", "minecraft:cooked_beef", "minecraft:cooked_chicken",
     "minecraft:cooked_mutton", "minecraft:cooked_porkchop", "minecraft:cooked_rabbit",
     "minecraft:mutton", "minecraft:porkchop", "minecraft:rabbit", "naturalist:bushmeat",
     "naturalist:clam_meat", "naturalist:cooked_bushmeat", "naturalist:cooked_clam_meat",
     "naturalist:cooked_crab_meat", "naturalist:cooked_drumstick", "naturalist:cooked_duck",
     "naturalist:cooked_lizard_tail", "naturalist:cooked_mammoth_meat", "naturalist:cooked_venison",
     "naturalist:crab_meat", "naturalist:drumstick", "naturalist:duck", "naturalist:lizard_tail",
     "naturalist:mammoth_meat", "naturalist:morsel", "naturalist:venison"]
        .into_iter().map(str::to_owned).collect()
}
