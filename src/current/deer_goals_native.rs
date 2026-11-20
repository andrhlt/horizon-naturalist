use horizon_client::mobs::goals::{self, Cx, Goal, LOOK, MOVE};
use horizon_client::mobs::{BreedExperience, Mob, MobEvent, breeding_experience};
use horizon_core::random::RandomSource;

#[derive(Clone, Debug, Default)]
struct SameKindBreed {
    partner: Option<u32>,
    love_time: i32,
}
impl Goal for SameKindBreed {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.in_love <= 0 {
            return false;
        }
        let id = m.id;
        let kind = m.kind.id.clone();
        self.partner =
            cx.w.nearby(m.position, 8.0, move |s| {
                s.id != id && s.kind.id == kind && s.in_love && !s.baby && !s.panicking
            })
            .find(|s| m.distance_sqr(s.position) <= 64.0)
            .map(|s| s.id);
        self.partner.is_some()
    }
    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        self.partner
            .and_then(|p| cx.w.seen(p))
            .is_some_and(|s| s.alive && s.in_love && !s.panicking)
            && self.love_time < 60
    }
    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.partner = None;
        self.love_time = 0;
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(p) = self.partner.and_then(|p| cx.w.seen(p)).cloned() else {
            return;
        };
        let x = m.max_head_x_rot();
        m.look_at_speed([p.position.x, p.eye_y, p.position.z], 10.0, x);
        m.nav_move_to_entity(cx.w, [p.position.x, p.position.y, p.position.z], 1, 1.0);
        self.love_time += 1;
        if self.love_time >= goals::reduced(60) && m.distance_sqr(p.position) < 9.0 {
            let parent_variant = Some(m.random.next_boolean());
            let experience = if !cx.w.asks_baby_spawn {
                BreedExperience::Drawn(breeding_experience(m, cx.w.mob_drops))
            } else {
                BreedExperience::Deferred
            };
            m.extra.in_love = 0;
            m.extra.age = 6000;
            cx.events.push(MobEvent::Breed {
                parent: m.id,
                partner: p.id,
                equine: None,
                parent_variant,
                wolf_voice: None,
                experience,
            });
        }
    }
}

pub fn install(m: &mut Mob, predators: std::collections::HashSet<String>) {
    use goals::{Float, LookAtPlayer, Panic, RandomLookAround, Stroll, Tempt};
    if m.kind.id != "minecraft:cow"
        && !(m.kind.id.starts_with("naturalist:")
            && m.species() == horizon_client::mobs::Species::Cow)
    {
        return;
    }
    let deer = m.kind.id == "naturalist:deer";
    let food = m.kind.food.clone();
    let g = &mut m.ai.goals;
    g.clear();
    g.add(0, Float);
    if deer {
        g.add(1, DeerPanic::default());
    } else {
        g.add(1, Panic::new(2.0));
    }
    g.add(2, SameKindBreed::default());
    g.add(3, Tempt::new(1.25, food, deer));
    if deer {
        use goals::{Avoid, AvoidEntity};
        use horizon_client::mobs::{drops, species};
        g.add(
            4,
            AvoidEntity::new(Avoid::Player, 8.0, 1.5, 2.0)
                .player_when(|_, p| !p.creative && !p.spectator && !p.sneaking),
        );
        let monsters = species::ALL
            .iter()
            .map(|d| d.species)
            .filter(|&s| drops::is_monster(s))
            .collect();
        g.add(4, AvoidEntity::new(Avoid::Species(monsters), 4.0, 1.5, 2.0));
        g.add(
            4,
            PredatorAvoid {
                types: predators,
                from: [0.0; 3],
            },
        );
    }
    g.add(
        if deer { 5 } else { 4 },
        SameKindFollow {
            speed: if deer { 1.5 } else { 1.25 },
            parent: None,
            recalc: 0,
        },
    );
    if deer {
        g.add(6, DeerEatBlock::default());
    }
    g.add(if deer { 7 } else { 5 }, Stroll::avoiding_water(1.0));
    g.add(if deer { 8 } else { 6 }, LookAtPlayer::new(6.0));
    g.add(if deer { 9 } else { 7 }, RandomLookAround::default());
}

#[derive(Clone, Debug)]
struct PredatorAvoid {
    types: std::collections::HashSet<String>,
    from: [f64; 3],
}
impl Goal for PredatorAvoid {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn flags(&self) -> u8 {
        MOVE
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        let id = m.id;
        let y = m.position.y;
        let hit =
            cx.w.nearby(m.position, 10.0, |s| {
                s.id != id
                    && self.types.contains(&s.kind.id)
                    && horizon_client::mobs::drops::is_animal(s.species)
                    && (s.position.y - y).abs() <= 13.0
            })
            .find(|s| m.sees(cx.w, horizon_client::mobs::Who::Mob(s.id)));
        let Some(hit) = hit else {
            return false;
        };
        let from = hit.position;
        if m.distance_sqr(from) > 100.0 {
            return false;
        }
        let Some(pos) = m.default_random_pos_away(cx.w, 16, 7, [from.x, from.y, from.z]) else {
            return false;
        };
        if (from.x - pos[0]).powi(2) + (from.y - pos[1]).powi(2) + (from.z - pos[2]).powi(2)
            < m.distance_sqr(from)
        {
            return false;
        }
        self.from = [from.x, from.y, from.z];
        m.ai.pending_path = m.create_path_to(
            cx.w,
            [
                pos[0].floor() as i32,
                pos[1].floor() as i32,
                pos[2].floor() as i32,
            ],
            0,
        );
        m.ai.pending_path.is_some()
    }
    fn can_continue(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        !m.ai.nav.is_done()
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        let path = m.ai.pending_path.take();
        m.nav_move_to_path(cx.w, path, 1.5);
    }
    fn tick(&mut self, m: &mut Mob, _: &mut Cx) {
        let d = (m.position.x - self.from[0]).powi(2)
            + (m.position.y - self.from[1]).powi(2)
            + (m.position.z - self.from[2]).powi(2);
        m.ai.nav.speed_modifier = if d < 49.0 { 2.0 } else { 1.5 };
    }
}

#[derive(Clone, Debug, Default)]
struct DeerEatBlock(goals::EatBlock);
impl Goal for DeerEatBlock {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn flags(&self) -> u8 {
        self.0.flags()
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_use(m, cx)
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(m, cx)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx)
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.stop(m, cx)
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let before = cx.events.len();
        self.0.tick(m, cx);
        let ate = cx.events[before..].iter().any(|event| matches!(event, MobEvent::GameEvent { event: "minecraft:eat", source, .. } if *source == m.id));
        if ate && horizon_client::mobs::ai::can_age_up(m) {
            horizon_client::mobs::ai::age_up(m, 60, false);
        }
    }
}

#[derive(Clone, Debug)]
struct SameKindFollow {
    speed: f64,
    parent: Option<u32>,
    recalc: i32,
}
impl Goal for SameKindFollow {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn flags(&self) -> u8 {
        0
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        use horizon_core::geometry::{Aabb, Vec3};
        if m.extra.age >= 0 {
            return false;
        }
        let p = m.position;
        let mut best: Option<(u32, f64)> = None;
        let area = Aabb::new(
            Vec3::new(p.x - 9.0, p.y - 5.0, p.z - 9.0),
            Vec3::new(p.x + 9.0, p.y + 5.0, p.z + 9.0),
        );
        for s in cx.w.others_near(area).filter(|s| {
            s.alive
                && s.kind.id == m.kind.id
                && !s.baby
                && (s.position.x - p.x).abs() <= 8.0
                && (s.position.y - p.y).abs() <= 4.0
                && (s.position.z - p.z).abs() <= 8.0
        }) {
            let d = m.distance_sqr(s.position);
            if best.is_none_or(|b| d <= b.1) {
                best = Some((s.id, d));
            }
        }
        match best {
            Some((id, d)) if d >= 9.0 => {
                self.parent = Some(id);
                true
            }
            _ => false,
        }
    }
    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.age >= 0 {
            return false;
        }
        let Some(p) = self.parent.and_then(|p| cx.w.seen(p)) else {
            return false;
        };
        p.alive && (9.0..=256.0).contains(&m.distance_sqr(p.position))
    }
    fn start(&mut self, _: &mut Mob, _: &mut Cx) {
        self.recalc = 0;
    }
    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.parent = None;
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.recalc -= 1;
        if self.recalc <= 0 {
            self.recalc = goals::reduced(10);
            if let Some(p) = self.parent.and_then(|p| cx.w.seen(p)) {
                m.nav_move_to_entity(
                    cx.w,
                    [p.position.x, p.position.y, p.position.z],
                    1,
                    self.speed,
                );
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct DeerPanic {
    pos: [f64; 3],
}
impl Goal for DeerPanic {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn flags(&self) -> u8 {
        MOVE
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if !goals::should_panic(m) {
            return false;
        }
        let at = [m.position.x, m.position.y, m.position.z].map(|p| p.floor() as i32);
        if m.fire_ticks > 0 && cx.w.collision_bounds(at).is_none() {
            if let Some(water) = cx.w.nearest_block(at, 10, 2, |[x, y, z], _| {
                cx.w.fluid(x, y, z) == horizon_client::blocks::Fluid::Water
            }) {
                self.pos = water.map(f64::from);
                return true;
            }
        }
        let Some(pos) = m.default_random_pos(cx.w, 15, 4) else {
            return false;
        };
        self.pos = pos;
        true
    }
    fn can_continue(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        !m.ai.nav.is_done()
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        m.nav_move_to(cx.w, self.pos, 1.6);
        m.ai.panicking = true;
    }
    fn stop(&mut self, m: &mut Mob, _: &mut Cx) {
        m.ai.panicking = false;
    }
}

static VARIANT_TEXTURES: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, Option<(String, String)>>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));
pub fn clear_variant_textures() {
    VARIANT_TEXTURES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clear();
}
pub fn variant_textures(game: &crate::game::Game<'_>, variant: &str) -> Option<(String, String)> {
    if let Some(found) = VARIANT_TEXTURES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(variant)
    {
        return found.clone();
    }
    let value = (|| {
        let (namespace, path) = variant.split_once(':')?;
        let id =
            mod_api::Identifier::parse(&format!("{namespace}:naturalist/deer_variant/{path}.json"))
                .ok()?;
        let bytes = game.resource(mod_api::PackType::ServerData, &id)?;
        let document: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        let adult = document.get("texture")?.as_str()?.to_owned();
        let baby = document
            .get("baby_texture")
            .and_then(|v| v.as_str())
            .unwrap_or(&adult)
            .to_owned();
        Some((adult, baby))
    })();
    VARIANT_TEXTURES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(variant.to_owned(), value.clone());
    value
}


#[derive(Clone, Debug)]
struct SpawnVariant {
    id: String,
    biomes: Option<Vec<String>>,
    weight: u32,
    priority: i64,
}
static SPAWN_VARIANTS: std::sync::LazyLock<std::sync::Mutex<Vec<SpawnVariant>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
static PENDING_VARIANTS: mod_api::AttachmentType<std::collections::HashMap<u32, String>> = mod_api::AttachmentType {
    namespace: "naturalist", name: "pending_deer_variants", default: std::collections::HashMap::new,
    codec: None, copy_on_death: false, sync: false,
};
pub fn load_spawn_variants(stack: &crate::game::functions::PackStack) {
    let packs = stack
        .builtin
        .iter()
        .chain(std::iter::once(&stack.content))
        .chain(stack.enabled_packs());
    let mut all = std::collections::BTreeMap::new();
    for pack in packs {
        for path in pack.list_folder("naturalist/deer_variant") {
            let Some((namespace, tail)) =
                path.strip_prefix("data/").and_then(|s| s.split_once('/'))
            else {
                continue;
            };
            let Some(name) = tail
                .strip_prefix("naturalist/deer_variant/")
                .and_then(|s| s.strip_suffix(".json"))
            else {
                continue;
            };
            let Some(bytes) = pack.read(&path) else {
                continue;
            };
            let Ok(v) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
                continue;
            };
            if v.get("texture").and_then(|v| v.as_str()).is_none() {
                continue;
            }
            let biomes = match v.get("biomes") {
                None => None,
                Some(serde_json::Value::String(s)) => Some(vec![s.clone()]),
                Some(serde_json::Value::Array(a)) if a.iter().all(|x| x.is_string()) => Some(
                    a.iter()
                        .filter_map(|x| x.as_str().map(str::to_owned))
                        .collect(),
                ),
                _ => continue,
            };
            let Some(weight) = v
                .get("weight")
                .map_or(Some(1), |w| w.as_u64().and_then(|w| u32::try_from(w).ok()))
            else {
                continue;
            };
            let Some(priority) = v.get("priority").map_or(Some(0), |p| p.as_i64()) else {
                continue;
            };
            let id = format!("{namespace}:{name}");
            all.insert(
                id.clone(),
                SpawnVariant {
                    id,
                    biomes,
                    weight,
                    priority,
                },
            );
        }
    }
    *SPAWN_VARIANTS.lock().unwrap_or_else(|e| e.into_inner()) = all.into_values().collect();
}
pub fn choose_spawn_variant(game: &mut crate::game::Game<'_>, mob: &Mob, staged: bool) {
    if mob.kind.id != "naturalist:deer" {
        return;
    }
    let biome = game.biome_at(mob.position.components()).to_owned();
    let variants = SPAWN_VARIANTS.lock().unwrap_or_else(|e| e.into_inner());
    let matches = |selector: &str| {
        let Some(tag) = selector.strip_prefix('#') else {
            return selector == biome;
        };
        let canonical = if tag.contains(':') { tag.to_owned() } else { format!("minecraft:{tag}") };
        if let Some(members) = game.world_tables.as_ref()
            .and_then(|tables| tables.pack_tags.get("worldgen/biome"))
            .and_then(|tags| tags.get(&canonical)) {
            return members.contains(&biome);
        }
        let entries = game.tag_entries("worldgen/biome", &canonical);
        entries.iter().any(|id| id == &biome)
            || (entries.is_empty() && game.res.mob_data().biome_has(&canonical, &biome))
    };
    let mut pool: Vec<_> = variants
        .iter()
        .filter(|v| {
            v.weight > 0
                && v.biomes
                    .as_ref()
                    .is_some_and(|ids| ids.iter().any(|id| matches(id)))
        })
        .collect();
    if pool.is_empty() {
        pool = variants
            .iter()
            .filter(|v| v.weight > 0 && v.biomes.is_none())
            .collect();
    }
    if let Some(priority) = pool.iter().map(|v| v.priority).max() {
        pool.retain(|v| v.priority == priority);
    }
    let Some(total) = pool.iter().try_fold(0i32, |sum, v| {
        sum.checked_add(i32::try_from(v.weight).ok()?)
    }) else {
        return;
    };
    let Ok(mut roll) = game
        .world
        .random
        .next_int_bound(total)
        .map(u32::try_from)
        .and_then(|x| x.map_err(|_| horizon_core::random::InvalidBound))
    else {
        return;
    };
    for v in pool {
        if roll < v.weight {
            if staged {
                let holder = mod_api::Holder::Level;
                if let Ok(mut pending) = PENDING_VARIANTS.get(&mut game.world.attachments, holder) {
                    pending.insert(mob.id, v.id.clone());
                    let _ = PENDING_VARIANTS.set(&mut game.world.attachments, holder, pending);
                }
            }
            let raw = game
                .persist
                .raw
                .entry(mob.id)
                .or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
            if let horizon_core::nbt::Tag::Compound(fields) = raw {
                fields.insert(
                    "Variant".into(),
                    horizon_core::nbt::Tag::String(v.id.as_str().into()),
                );
            }
            break;
        } else {
            roll -= v.weight;
        }
    }
}

pub fn normalize_variant(game: &mut crate::game::Game<'_>, id: u32) {
    if !game
        .mobs
        .iter()
        .any(|m| m.id == id && m.kind.id == "naturalist:deer")
    {
        return;
    }
    let raw = game
        .persist
        .raw
        .entry(id)
        .or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
    if let horizon_core::nbt::Tag::Compound(fields) = raw {
        if !matches!(
            fields.get(&"Variant".into()),
            Some(horizon_core::nbt::Tag::String(_))
        ) {
            fields.insert(
                "Variant".into(),
                horizon_core::nbt::Tag::String("naturalist:deer".into()),
            );
        }
    }
}

pub fn take_pending_variant(game: &mut crate::game::Game<'_>, id: u32) -> Option<String> {
    let holder = mod_api::Holder::Level;
    let pending = &mut game.world.attachments;
    let mut variants = PENDING_VARIANTS.get_existing(pending, holder).ok().flatten()?;
    let variant = variants.remove(&id);
    if variants.is_empty() {
        let _ = PENDING_VARIANTS.remove(pending, holder);
    } else {
        let _ = PENDING_VARIANTS.set(pending, holder, variants);
    }
    variant
}
