fn fish_no_push(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    if !matches!(mob.kind.id.as_str(), "naturalist:bass" | "naturalist:piranha" | "naturalist:jellyfish" | "naturalist:crab" | "naturalist:snake" | "naturalist:rat") {
        return ControlFlow::Continue(());
    }

    // These mobs leave `LivingEntity.pushEntities`'s cramming pass intact and
    // override only the per-neighbor push itself.
    mob.on_climbable = on_climbable(mob, world);
    let limit = world.max_entity_cramming;
    if limit > 0 {
        let bounds = mob.bounds();
        let own_team = world.teams.of(Who::Mob(mob.id));
        let mut targets = 0;
        let mut without_passengers = 0;
        let can_push = |who| crate::scoreboard::pushable_by(own_team, world.teams.of(who), true);
        if let Some(player) = &world.player
            && mob.rider.is_none() && player.alive && !player.spectator && player.bounds.intersects(bounds)
            && can_push(Who::Player)
        {
            targets += 1;
            without_passengers += i32::from(!player.passenger);
        }
        for other in world.others_near(bounds) {
            if other.id != mob.id && other.alive && other.pushable && other.bounds.intersects(bounds)
                && can_push(Who::Mob(other.id))
            {
                targets += 1;
                without_passengers += i32::from(!other.passenger);
            }
        }
        if targets >= limit && mob.random.next_int_bound(4).unwrap_or(1) == 0 && without_passengers >= limit {
            hurt(mob, world, events, 6.0, Damage::of(DamageKind::Cramming));
        }
    }
    ControlFlow::Break(())
}
