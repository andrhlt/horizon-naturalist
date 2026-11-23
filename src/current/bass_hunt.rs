//! Bass hunt goal and its resident pre-tick candidate snapshot.
use mod_api::{
    AttachmentQueries, AttachmentType, Compound, EntityRef, Goal, GoalContext, GoalFlags,
    GoalRegistrar, Holder, LivingRef, RandomSource, Tag,
};

#[derive(Clone, Default)]
struct Candidates {
    tier: u8,
    fish: Vec<(EntityRef, u8)>,
}
static CANDIDATES: AttachmentType<Candidates> = AttachmentType {
    namespace: "naturalist",
    name: "bass_hunt_candidates",
    default: Candidates::default,
    codec: None,
    copy_on_death: false,
    sync: false,
};

/// Java NumericTag.intValue: integer casts wrap; floats floor then saturate.
pub fn legacy_numeric_index(value: &Tag) -> Option<i32> {
    Some(match value {
        Tag::Byte(n) => i32::from(*n),
        Tag::Short(n) => i32::from(*n),
        Tag::Int(n) => *n,
        Tag::Long(n) => *n as i32,
        Tag::Float(bits) => f32::from_bits(*bits).floor() as i32,
        Tag::Double(bits) => f64::from_bits(*bits).floor() as i32,
        _ => return None,
    })
}

/// Interpret legacy numeric tags the same way for gameplay, rendering and buckets.
pub fn variant(value: Option<&Tag>) -> Option<String> {
    let value = value?;
    if let Tag::String(s) = value {
        return Some(s.to_string_lossy());
    }
    let index = legacy_numeric_index(value)?.rem_euclid(3) as usize;
    Some(
        [
            "naturalist:bass",
            "naturalist:bass_medium",
            "naturalist:bass_large",
        ][index]
            .to_owned(),
    )
}

pub fn tier(data: &Compound) -> u8 {
    match variant(data.get(&"Variant".into())).as_deref() {
        Some("naturalist:bass_medium") => 1,
        Some("naturalist:bass_large") => 2,
        _ => 0,
    }
}

/// Called before the mob pass while the level's entity lists are resident.
pub fn prepare(
    host: &mut dyn mod_api::ServerHost,
    id: u32,
    data: &Compound,
) -> Result<(), mod_api::AttachmentError> {
    let Some(entity) = host.entity(id) else {
        return Err(mod_api::AttachmentError::MissingHolder);
    };
    let own_tier = tier(data);
    if let Some(mob) = host.mob(entity) {
        let has_melee = mob
            .goals(host)
            .is_some_and(|goals| goals.iter().any(|goal| goal.id == "melee_attack"));
        if !has_melee {
            mob.add_goal(
                host,
                3,
                &mod_api::GoalSpec::MeleeAttack {
                    speed: 1.4,
                    follow_unseen: true,
                },
            );
        }
    }
    let mut fish = Vec::new();
    if own_tier > 0 {
        if let (Some(level), Some(position), Some(living)) = (
            entity.level(host),
            entity.position(host),
            host.living(entity),
        ) {
            let range = living
                .attribute_value(host, "minecraft:follow_range")
                .unwrap_or(16.0);
            let min = [position[0] - range, position[1] - 4.0, position[2] - range];
            let max = [position[0] + range, position[1] + 4.0, position[2] + range];
            for other in level.entities_in(host, min, max).unwrap_or_default() {
                if other == entity
                    || other
                        .kind(host)
                        .is_none_or(|kind| kind.id(host) != "naturalist:bass")
                {
                    continue;
                }
                if let Some(data) = other.nbt(host) {
                    let other_tier = tier(&data);
                    if other_tier < own_tier {
                        fish.push((other, other_tier));
                    }
                }
            }
        }
    }
    CANDIDATES.set(
        host,
        Holder::Entity(id),
        Candidates {
            tier: own_tier,
            fish,
        },
    )
}

pub struct BassPart;
#[mod_api::horizon_part]
impl mod_api::EntityPart for BassPart {
    fn spawn_cluster_size(
        &self,
        _: &dyn mod_api::Host,
        _: mod_api::UnspawnedMobRef,
        reason: mod_api::SpawnReason,
        _: Option<mod_api::SpawnSource>,
        _: u32,
    ) -> Option<u32> {
        (reason == mod_api::SpawnReason::Natural).then_some(6)
    }

    fn register_goals(&self, goals: &mut GoalRegistrar<'_>) {
        goals.add_target_goal(1, BassHunt::default());
    }
}

#[derive(Default)]
struct BassHunt {
    target: Option<LivingRef>,
    unseen: u32,
}
impl Goal for BassHunt {
    fn flags(&self) -> GoalFlags {
        GoalFlags::TARGET
    }
    fn name(&self) -> &'static str {
        "NaturalistBassHunt"
    }
    fn can_use(&mut self, cx: &mut GoalContext<'_>) -> bool {
        let own = cx.mob().entity();
        let query = |host: &dyn mod_api::Host| {
            let key = mod_api::AttachmentHolder::Entity(own);
            let cooldown = host.query(&super::COOLDOWN).ok()?.get(key).ok().flatten()?;
            let candidates = host.query(&CANDIDATES).ok()?.get(key).ok().flatten()?;
            Some((cooldown, candidates))
        };
        let Some((cooldown, candidates)) = query(cx.host()) else {
            return false;
        };
        if candidates.tier == 0 || cooldown > 0 {
            return false;
        }
        if cx.random().next_int_bound(5).unwrap_or(1) != 0 {
            return false;
        }
        let Some(own_living) = cx.host().living(own) else {
            return false;
        };
        let Some(pos) = own.position(cx.host()) else {
            return false;
        };
        let range = own_living
            .attribute_value(cx.host(), "minecraft:follow_range")
            .unwrap_or(16.0);
        let mut best: Option<(f64, LivingRef)> = None;
        for (entity, other_tier) in candidates.fish {
            if other_tier >= candidates.tier {
                continue;
            }
            let Some(living) = cx.host().living(entity) else {
                continue;
            };
            if living.alive(cx.host()) != Some(true)
                || own_living.has_line_of_sight(cx.host(), entity) != Some(true)
            {
                continue;
            }
            let Some(other_pos) = entity.position(cx.host()) else {
                continue;
            };
            let distance = (0..3).map(|i| (pos[i] - other_pos[i]).powi(2)).sum::<f64>();
            if distance <= range * range && best.as_ref().is_none_or(|(old, _)| distance < *old) {
                best = Some((distance, living));
            }
        }
        self.target = best.map(|(_, target)| target);
        self.target.is_some()
    }
    fn start(&mut self, cx: &mut GoalContext<'_>) {
        self.unseen = 0;
        cx.set_target(self.target);
    }
    fn can_continue(&mut self, cx: &mut GoalContext<'_>) -> bool {
        let Some(target) = self.target else {
            return false;
        };
        let own = cx.mob().entity();
        let Some(living) = cx.host().living(own) else {
            return false;
        };
        if target.alive(cx.host()) != Some(true) {
            return false;
        }
        let (Some(pos), Some(other)) =
            (own.position(cx.host()), target.entity().position(cx.host()))
        else {
            return false;
        };
        let range = living
            .attribute_value(cx.host(), "minecraft:follow_range")
            .unwrap_or(16.0);
        if (0..3).map(|i| (pos[i] - other[i]).powi(2)).sum::<f64>() > range * range {
            return false;
        }
        if living.has_line_of_sight(cx.host(), target.entity()) == Some(true) {
            self.unseen = 0;
        } else {
            self.unseen += 1;
        }
        self.unseen <= 60
    }
    fn stop(&mut self, cx: &mut GoalContext<'_>) {
        self.target = None;
        cx.set_target(None);
    }
}

#[cfg(test)]
mod variant_tests {
    use super::*;
    #[test]
    fn numeric_nbt_uses_java_int_value_and_floor_mod() {
        let cases = [
            (Tag::Byte(-2), 1),
            (Tag::Short(-2), 1),
            (Tag::Int(-2), 1),
            (Tag::Long(-2), 1),
            (Tag::Float((-1.2_f32).to_bits()), 1),
            (Tag::Double((-1.2_f64).to_bits()), 1),
            (Tag::Long(4_294_967_294), 1),
            (Tag::Int(4), 1),
            (Tag::Int(-1), 2),
        ];
        for (value, expected) in cases {
            let mut data = Compound::default();
            data.insert("Variant".into(), value);
            assert_eq!(tier(&data), expected, "{data:?}");
        }
    }
}
