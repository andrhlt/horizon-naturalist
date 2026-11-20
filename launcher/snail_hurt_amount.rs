fn snail_hurt_amount(amount: f32, mob: &Mob) -> f32 {
    if mob.kind.id == "naturalist:snail" && naturalist_snail::snail(mob).hiding
        || mob.kind.id == "naturalist:tortoise" && self::goals::naturalist_tortoise_goals::hidden(mob)
    {
        return amount * 0.8;
    }
    amount
}
