use super::{Mob, MobWorld, PlayerView};
use horizon_core::geometry::Vec3;

pub const KIND: &str = "naturalist:hedgehog";
pub const FOOD: [&str; 2] = ["minecraft:sweet_berries", "minecraft:glow_berries"];

#[derive(Clone, Debug, Default)]
pub struct State {
    pub hiding: bool,
    pub visible_lan_player: Option<bool>,
    pub rolling: bool,
    pub ground_roll: bool,
    pub following_cooldown: u8,
    pub return_ticks: Option<u16>,
    pub thrower: Option<[i32; 4]>,
    pub enchantments: Vec<(std::sync::Arc<str>, u32)>,
}

pub fn state(mob: &Mob) -> &State {
    mob.ai.nav.__horizon_mixins_naturalist_hedgehog_state()
}

pub fn state_mut(mob: &mut Mob) -> &mut State {
    mob.ai.nav.__horizon_mixins_naturalist_hedgehog_state_mut()
}

pub fn food(item: &str) -> bool {
    FOOD.contains(&item)
}

pub fn can_hide(mob: &Mob, world: &MobWorld<'_>) -> bool {
    let pose = state(mob);
    if mob.extra.tame || pose.rolling || pose.ground_roll || mob.extra.sitting_pose {
        return false;
    }
    pose.visible_lan_player.unwrap_or_else(|| world.player.as_ref()
        .is_some_and(|player| visible_player(mob, world, player)))
}

pub fn visible_player(mob: &Mob, world: &MobWorld<'_>, player: &PlayerView) -> bool {
    let area = mob.bounds().inflate(Vec3::new(6.0, 3.0, 6.0));
    let range = (6.0 * player.visibility_percent(Some(KIND), world.items)).max(2.0);
    player.alive
        && !player.creative
        && !player.spectator
        && !player.sneaking
        && !player.main_hand.as_deref().is_some_and(food)
        && !player.off_hand.as_deref().is_some_and(food)
        && area.contains(player.position)
        && mob.distance_sqr(player.position) <= range * range
        && world.line_of_sight(
            Vec3::new(mob.position.x, mob.eye_y(), mob.position.z),
            Vec3::new(player.eye[0], player.eye[1], player.eye[2]),
        )
}

pub fn stop_rolling(mob: &mut Mob) {
    let pose = state_mut(mob);
    pose.rolling = false;
    pose.return_ticks = None;
    pose.following_cooldown = 40;
    mob.no_gravity = false;
    mob.fire_ticks = 0;
}

pub fn render_bits(mob: &Mob) -> u8 {
    let pose = state(mob);
    u8::from(pose.hiding)
        | (u8::from(pose.rolling) << 1)
        | (u8::from(pose.ground_roll) << 2)
}

pub fn enchantment(mob: &Mob, name: &str) -> u32 {
    state(mob).enchantments.iter().find(|(id, _)| id.as_ref() == name).map_or(0, |(_, level)| *level)
}

pub fn save_enchantments(mob: &Mob) -> Option<horizon_world::nbt::Tag> {
    use horizon_world::nbt::Tag;
    let enchantments = &state(mob).enchantments;
    if enchantments.is_empty() {
        return None;
    }
    Some(Tag::Compound(enchantments.iter().map(|(name, level)| {
        (name.as_ref().into(), Tag::Int(*level as i32))
    }).collect()))
}

pub fn load_enchantments(mob: &mut Mob, tag: Option<&horizon_world::nbt::Tag>) {
    use horizon_world::nbt::Tag;
    let Some(Tag::Compound(entries)) = tag else { return; };
    state_mut(mob).enchantments = entries.iter().filter_map(|(id, value)| {
        let Tag::Int(level) = value else { return None; };
        (*level > 0).then(|| (id.to_string_lossy().into(), *level as u32))
    }).collect();
}
