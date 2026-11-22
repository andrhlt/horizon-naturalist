use mod_api::{
    AttachmentError, AttachmentType, EntityPart, EntityRef, Goal, GoalContext,
    GoalFlags, GoalRegistrar, Holder, ItemEntityRef,
};

static EAT_COOLDOWN: AttachmentType<i32> = AttachmentType {
    namespace: "naturalist",
    name: "bird_eat_cooldown",
    default: || 0,
    codec: None,
    copy_on_death: false,
    sync: false,
};

struct BirdPart;

#[mod_api::horizon_part]
impl EntityPart for BirdPart {
    fn register_goals(&self, goals: &mut GoalRegistrar<'_>) {
        if goals.mob().id() == "naturalist:bird" {
            goals.add_goal(1, Flock::default());
            goals.add_goal(6, SeedForaging::default());
        }
    }
}

#[derive(Default)]
struct Flock {
    leader: Option<EntityRef>,
}

impl Goal for Flock {
    fn flags(&self) -> GoalFlags { GoalFlags::MOVE }

    fn can_use(&mut self, cx: &mut GoalContext<'_>) -> bool {
        self.leader = nearest_mob(cx);
        self.leader.is_some()
    }

    fn can_continue(&mut self, cx: &mut GoalContext<'_>) -> bool {
        let Some(leader) = self.leader else { return false };
        let Some(at) = cx.mob().entity().position(cx.host()) else { return false };
        let Some(leader_at) = leader.position(cx.host()) else { return false };
        distance_squared(at, leader_at) > 4.0 && distance_squared(at, leader_at) < 256.0
    }

    fn start(&mut self, cx: &mut GoalContext<'_>) { self.follow(cx); }
    fn tick(&mut self, cx: &mut GoalContext<'_>) { self.follow(cx); }
    fn stop(&mut self, cx: &mut GoalContext<'_>) {
        self.leader = None;
        cx.stop_navigation();
    }
}

impl Flock {
    fn follow(&self, cx: &mut GoalContext<'_>) {
        if let Some(leader) = self.leader && let Some(at) = leader.position(cx.host()) {
            cx.move_to(at, 1.0);
        }
    }
}

#[derive(Default)]
struct SeedForaging {
    target: Option<EntityRef>,
}

impl Goal for SeedForaging {
    fn flags(&self) -> GoalFlags {
        GoalFlags::MOVE
    }

    fn can_use(&mut self, cx: &mut GoalContext<'_>) -> bool {
        self.target = nearest_seed(cx);
        self.target.is_some()
    }

    fn can_continue(&mut self, cx: &mut GoalContext<'_>) -> bool {
        let Some(target) = self.target else {
            return false;
        };
        let Some(bird_pos) = cx.mob().entity().position(cx.host()) else {
            return false;
        };
        let Some(seed_pos) = target.position(cx.host()) else {
            return false;
        };
        is_seed(cx.host(), target)
            && distance_squared(bird_pos, seed_pos) > 2.25
            && distance_squared(bird_pos, seed_pos) < 256.0
    }

    fn start(&mut self, cx: &mut GoalContext<'_>) {
        self.walk_to_seed(cx);
    }

    fn tick(&mut self, cx: &mut GoalContext<'_>) {
        self.walk_to_seed(cx);
    }

    fn stop(&mut self, cx: &mut GoalContext<'_>) {
        self.target = None;
        cx.stop_navigation();
    }
}

impl SeedForaging {
    fn walk_to_seed(&self, cx: &mut GoalContext<'_>) {
        let Some(target) = self.target else {
            return;
        };
        if let Some(pos) = target.position(cx.host()) {
            cx.move_to(pos, 1.0);
        }
    }
}

fn nearest_seed(cx: &GoalContext<'_>) -> Option<EntityRef> {
    let host = cx.host();
    let bird = cx.mob().entity();
    let pos = bird.position(host)?;
    let level = bird.level(host)?;
    let items = level.entities_in(
        host,
        [pos[0] - 16.0, pos[1] - 4.0, pos[2] - 16.0],
        [pos[0] + 16.0, pos[1] + 4.0, pos[2] + 16.0],
    )?;
    items
        .into_iter()
        .filter(|item| is_seed(host, *item))
        .min_by(|left, right| {
            let bird_pos = bird.position(host).unwrap_or(pos);
            let left_pos = left.position(host).unwrap_or(bird_pos);
            let right_pos = right.position(host).unwrap_or(bird_pos);
            distance_squared(bird_pos, left_pos).total_cmp(&distance_squared(bird_pos, right_pos))
        })
}

fn nearest_mob(cx: &GoalContext<'_>) -> Option<EntityRef> {
    let host = cx.host();
    let bird = cx.mob().entity();
    let pos = bird.position(host)?;
    let level = bird.level(host)?;
    level.entities_in(
        host,
        [pos[0] - 16.0, pos[1] - 8.0, pos[2] - 16.0],
        [pos[0] + 16.0, pos[1] + 8.0, pos[2] + 16.0],
    )?.into_iter()
        .filter(|entity| entity.id() != bird.id() && host.mob(*entity).is_some())
        .min_by(|left, right| {
            let left_pos = left.position(host).unwrap_or(pos);
            let right_pos = right.position(host).unwrap_or(pos);
            distance_squared(pos, left_pos).total_cmp(&distance_squared(pos, right_pos))
        })
}

fn is_seed(host: &dyn mod_api::Host, entity: EntityRef) -> bool {
    let Some(item) = host.item_entity(entity) else {
        return false;
    };
    let Some(stack) = item.stack(host) else {
        return false;
    };
    matches!(
        stack.item().id(host).as_str(),
        "minecraft:wheat_seeds"
            | "minecraft:melon_seeds"
            | "minecraft:pumpkin_seeds"
            | "minecraft:beetroot_seeds"
    )
}

fn distance_squared(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

pub(crate) fn register(registrar: &mut dyn mod_api::Registrar) {
    registrar.entity_part("naturalist:bird", BirdPart);
}

pub(crate) fn eat_nearby_seeds(
    host: &mut dyn mod_api::ServerHost,
    bird: EntityRef,
) -> Result<(), AttachmentError> {
    if host.mob(bird).is_some_and(|mob| {
        mob.no_ai(host).unwrap_or(false) || mob.ai_paused(host).unwrap_or(false)
    }) {
        return Ok(());
    }
    let holder = Holder::Entity(bird.id());
    let remaining = EAT_COOLDOWN.get(host, holder)?;
    if remaining > 0 {
        EAT_COOLDOWN.set(host, holder, remaining - 1)?;
        return Ok(());
    }

    let Some(pos) = bird.position(host) else {
        return Ok(());
    };
    let Some(level) = bird.level(host) else {
        return Ok(());
    };
    let Some(entities) = level.entities_in(
        host,
        [pos[0] - 1.0, pos[1] - 1.0, pos[2] - 1.0],
        [pos[0] + 1.0, pos[1] + 1.0, pos[2] + 1.0],
    ) else {
        return Ok(());
    };
    let Some(item) = entities.into_iter().find_map(|entity| {
        if distance_squared(pos, entity.position(host)?) > 2.25 {
            return None;
        }
        host.item_entity(entity)
            .filter(|item| stack_is_seed(host, *item))
            .map(|item| (entity, item))
    }) else {
        return Ok(());
    };

    let (entity, item) = item;
    let Some(mut stack) = item.stack(host) else {
        return Ok(());
    };
    if stack.count() == 1 {
        entity.discard(host);
    } else if stack.set_count(stack.count() - 1).is_ok() {
        let _ = item.set_stack(host, stack);
    }
    // Java's bird forage goal waits two seconds before another seed pickup.
    // EntityTickPreEvent runs once per game tick, so preserve the full 40-tick
    // interval rather than allowing a pickup on the twentieth tick.
    EAT_COOLDOWN.set(host, holder, 40)
}

fn stack_is_seed(host: &dyn mod_api::Host, item: ItemEntityRef) -> bool {
    item.stack(host).is_some_and(|stack| {
        matches!(
            stack.item().id(host).as_str(),
            "minecraft:wheat_seeds"
                | "minecraft:melon_seeds"
                | "minecraft:pumpkin_seeds"
                | "minecraft:beetroot_seeds"
        )
    })
}
