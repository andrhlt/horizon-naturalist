fn elephant_goals(mob: &mut Mob) {
    use super::{
        Seen, Species, Who,
        goals::{
            Avoid, AvoidEntity, Breed, Cx, Float, Goal, HurtByTarget, LookAtPlayer, MeleeAttack,
            NearestTarget, Panic, RandomLookAround, Stroll, Tempt,
        },
        golem::ResetUniversalAnger,
        naturalist_giraffe::DistancedFollowParent,
        nav::naturalist_elephants::{self, ELEPHANT, MAMMOTH},
    };
    use std::sync::Arc;

    let herd_member: fn(&Seen) -> bool = match mob.kind.id.as_str() {
        // Mammoth extends Elephant in Java's hurt-child alert search.
        ELEPHANT => |other| other.effects.contains(&ELEPHANT) || other.effects.contains(&MAMMOTH),
        MAMMOTH => |other| other.effects.contains(&MAMMOTH),
        _ => return,
    };
    let kind = if mob.kind.id == ELEPHANT {
        ELEPHANT
    } else {
        MAMMOTH
    };
    let food: Arc<[String]> = Arc::from(["minecraft:melon_slice".to_owned()]);
    let mob_type = Arc::make_mut(&mut mob.kind);
    mob_type.food = food.clone();
    mob_type.tempt = food.clone();
    mob.extra.anger_end = -1;

    let goals = &mut mob.ai.goals;
    goals.clear();
    goals.add(0, Float);
    goals.add(
        1,
        AvoidEntity::new(Avoid::Species(vec![Species::Bee]), 8.0, 1.3, 1.3),
    );
    goals.add(2, MeleeAttack::new(1.2, true));
    goals.add(3, Panic::new(1.25));
    goals.add(4, DistancedFollowParent::new(kind, 1.2, 24.0, 6.0, 12.0));
    goals.add(5, Breed::new(1.0));
    goals.add(6, Tempt::new(1.0, food, false));
    goals.add(7, Stroll::avoiding_water(0.8));
    goals.add(8, LookAtPlayer::new(6.0));
    goals.add(9, RandomLookAround::default());

    let targets = &mut mob.ai.targets;
    targets.clear();
    targets.add(
        1,
        HerdRetaliation(HurtByTarget::new().polar_bear().alert_if(herd_member)),
    );
    targets.add(
        3,
        NearestTarget::player(true).when(naturalist_elephants::angry_player),
    );
    targets.add(5, ResetUniversalAnger::default());

    #[derive(Clone, Debug)]
    struct HerdRetaliation(HurtByTarget);

    impl Goal for HerdRetaliation {
        fn flags(&self) -> u8 {
            self.0.flags()
        }

        fn name(&self) -> &'static str {
            self.0.name()
        }

        fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx<'_, '_>) -> bool {
            self.0.can_use(mob, cx)
        }

        fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx<'_, '_>) -> bool {
            self.0.can_continue(mob, cx)
        }

        fn start(&mut self, mob: &mut Mob, cx: &mut Cx<'_, '_>) {
            let first_alert = cx.events.len();
            self.0.start(mob, cx);
            let Some(player) = &cx.w.player else { return };
            // The engine's ordinary alert carries only Who::Player. Preserve
            // the calf's selected real player until each adult's next turn.
            let alerts = &mut naturalist_elephants::state_mut(mob).player_alerts;
            for event in &cx.events[first_alert..] {
                if let MobEvent::SetTarget { mob: adult, target: Who::Player } = event {
                    alerts.push((*adult, player.uuid));
                }
            }
        }

        fn stop(&mut self, mob: &mut Mob, cx: &mut Cx<'_, '_>) {
            self.0.stop(mob, cx);
        }

        fn box_clone(&self) -> Box<dyn Goal> {
            Box::new(self.clone())
        }
    }
}
