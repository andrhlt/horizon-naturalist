use mod_api::{
    AttachmentQueries, EntityPart, Goal, GoalContext, GoalFlags, GoalRegistrar, LivingRef,
};

static FROM_HAND: mod_api::AttachmentType<bool> = mod_api::AttachmentType {
    namespace: "naturalist",
    name: "scorpion_from_hand",
    default: || false,
    codec: None,
    copy_on_death: false,
    sync: false,
};

pub fn set_from_hand(host: &mut dyn mod_api::ServerHost, id: u32, value: bool) {
    let _ = FROM_HAND.set(host, mod_api::Holder::Entity(id), value);
}

pub struct ScorpionPart;

#[mod_api::horizon_part]
impl EntityPart for ScorpionPart {
    fn register_goals(&self, goals: &mut GoalRegistrar<'_>) {
        if matches!(
            goals.mob().id(),
            "naturalist:desert_scorpion" | "naturalist:jungle_scorpion"
        ) {
            goals.add_target_goal(1, Hunt::new("minecraft:player", true));
            goals.add_target_goal(2, Hunt::new("minecraft:zombie", false));
        }
    }
}

struct Hunt {
    target_kind: &'static str,
    needs_darkness: bool,
    target: Option<LivingRef>,
}

impl Hunt {
    fn new(target_kind: &'static str, needs_darkness: bool) -> Self {
        Self {
            target_kind,
            needs_darkness,
            target: None,
        }
    }

    fn allowed(&self, cx: &GoalContext<'_>) -> bool {
        let host = cx.host();
        let entity = cx.mob().entity();
        let Some(level) = entity.level(host) else {
            return false;
        };
        let from_hand = host
            .query(&FROM_HAND)
            .ok()
            .and_then(|query| {
                query
                    .get(mod_api::AttachmentHolder::Entity(entity))
                    .ok()
                    .flatten()
            })
            .unwrap_or(false);
        if self.target_kind == "minecraft:player"
            && (from_hand
                || level.difficulty(host) == Some("peaceful")
                || (self.needs_darkness
                    && entity.position(host).is_some_and(|p| {
                        let pos = [p[0] as i32, p[1] as i32, p[2] as i32];
                        let block = level
                            .brightness(host, mod_api::LightLayer::Block, pos)
                            .unwrap_or(15);
                        let sky = level
                            .brightness(host, mod_api::LightLayer::Sky, pos)
                            .unwrap_or(15);
                        let darken = level.get_sky_darken(host).unwrap_or(15);
                        block.max(sky - darken) >= 8
                    })))
        {
            return false;
        }
        if self.target_kind == "minecraft:zombie" && !from_hand {
            return false;
        }
        true
    }

    fn nearest(&self, cx: &GoalContext<'_>) -> Option<LivingRef> {
        let host = cx.host();
        let entity = cx.mob().entity();
        let pos = entity.position(host)?;
        let level = entity.level(host)?;
        level
            .entities_in(
                host,
                [pos[0] - 16.0, pos[1] - 8.0, pos[2] - 16.0],
                [pos[0] + 16.0, pos[1] + 8.0, pos[2] + 16.0],
            )?
            .into_iter()
            .filter(|other| {
                other.id() != entity.id()
                    && other
                        .kind(host)
                        .is_some_and(|kind| kind.id(host) == self.target_kind)
            })
            .filter_map(|other| host.living(other))
            .filter(|other| {
                other.alive(host) == Some(true)
                    && host.living(entity).is_some_and(|mob| {
                        mob.has_line_of_sight(host, other.entity()) == Some(true)
                    })
            })
            .min_by(|a, b| {
                let ap = a.entity().position(host).unwrap_or(pos);
                let bp = b.entity().position(host).unwrap_or(pos);
                dist2(pos, ap).total_cmp(&dist2(pos, bp))
            })
    }
}

impl Goal for Hunt {
    fn flags(&self) -> GoalFlags {
        GoalFlags::TARGET
    }
    fn can_use(&mut self, cx: &mut GoalContext<'_>) -> bool {
        self.target = if self.allowed(cx) {
            self.nearest(cx)
        } else {
            None
        };
        self.target.is_some()
    }
    fn can_continue(&mut self, cx: &mut GoalContext<'_>) -> bool {
        self.allowed(cx)
            && self
                .target
                .is_some_and(|target| target.alive(cx.host()) == Some(true))
    }
    fn start(&mut self, cx: &mut GoalContext<'_>) {
        let _ = cx.set_target(self.target);
    }
    fn tick(&mut self, cx: &mut GoalContext<'_>) {
        let _ = cx.set_target(self.target);
    }
    fn stop(&mut self, cx: &mut GoalContext<'_>) {
        if self.target.is_some() {
            let _ = cx.set_target(None);
        }
        self.target = None;
    }
}

fn dist2(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}
