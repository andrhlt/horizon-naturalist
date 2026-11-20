use super::Game;
use horizon_core::random::RandomSource;
use horizon_client::mobs::naturalist_spawn_variants::{Variants, choose};
use horizon_world::nbt::Tag;
use std::collections::BTreeMap;

const DEER: &str = "naturalist:deer";
const DUCK: &str = "naturalist:duck";

fn registry(kind: &str) -> Option<&'static str> {
    match kind {
        "naturalist:elephant" => Some("naturalist/elephant_variant"),
        "naturalist:mammoth" => Some("naturalist/mammoth_variant"),
        "naturalist:mole" | "naturalist:dirt_trail" => Some("naturalist/mole_variant"),
        "naturalist:ostrich" => Some("naturalist/ostrich_variant"),
        "naturalist:lion" => Some("naturalist/lion_variant"),
        "naturalist:rat" => Some("naturalist/rat_variant"),
        "naturalist:snake" => Some("naturalist/snake_variant"),
        "naturalist:tiger" => Some("naturalist/tiger_variant"),
        "naturalist:tortoise" => Some("naturalist/tortoise_variant"),
        "naturalist:zebra" => Some("naturalist/zebra_variant"),
        "naturalist:bear" => Some("naturalist/bear_variant"),
        "naturalist:black_bear" => Some("naturalist/black_bear_variant"),
        "naturalist:rhino" => Some("naturalist/rhino_variant"),
        "naturalist:komodo_dragon" => Some("naturalist/komodo_dragon_variant"),
        "naturalist:lizard" => Some("naturalist/lizard_variant"),
        "naturalist:lizard_tail" => Some("naturalist/lizard_tail_variant"),
        "naturalist:desert_scorpion" => Some("naturalist/desert_scorpion_variant"),
        "naturalist:jungle_scorpion" => Some("naturalist/jungle_scorpion_variant"),
        "naturalist:capybara" => Some("naturalist/capybara_variant"),
        "naturalist:boar" => Some("naturalist/boar_variant"),
        "naturalist:turkey" => Some("naturalist/turkey_variant"),
        DEER => Some("naturalist/deer_variant"),
        DUCK => Some("naturalist/duck_variant"),
        "naturalist:giraffe" => Some("naturalist/giraffe_variant"),
        "naturalist:whale" => Some("naturalist/whale_variant"),
        "naturalist:great_white_shark" => Some("naturalist/great_white_shark_variant"),
        "naturalist:hippo" => Some("naturalist/hippo_variant"),
        "naturalist:alligator" => Some("naturalist/alligator_variant"),
        "naturalist:ant" => Some("naturalist/ant_variant"),
        "naturalist:hedgehog" => Some("naturalist/hedgehog_variant"),
        "naturalist:snail" => Some("naturalist/snail_variant"),
        "naturalist:vulture" => Some("naturalist/vulture_variant"),
        "naturalist:bird" => Some("naturalist/bird_variant"),
        "naturalist:butterfly" => Some("naturalist/butterfly_variant"),
        "naturalist:caterpillar" => Some("naturalist/caterpillar_variant"),
        "naturalist:dragonfly" => Some("naturalist/dragonfly_variant"),
        "naturalist:firefly" => Some("naturalist/firefly_variant"),
        "naturalist:giant_isopod" => Some("naturalist/giant_isopod_variant"),
        "naturalist:clam" => Some("naturalist/clam_variant"),
        "naturalist:crab" => Some("naturalist/crab_variant"),
        "naturalist:starfish" => Some("naturalist/starfish_variant"),
        "naturalist:anglerfish" => Some("naturalist/anglerfish_variant"),
        "naturalist:ray" => Some("naturalist/ray_variant"),
        "naturalist:jellyfish" => Some("naturalist/jellyfish_variant"),
        "naturalist:blobfish" => Some("naturalist/blobfish_variant"),
        "naturalist:piranha" => Some("naturalist/piranha_variant"),
        "naturalist:catfish" => Some("naturalist/catfish_variant"),
        "naturalist:bass" => Some("naturalist/bass_variant"),
        _ => None,
    }
}

pub(super) fn biome_matches(game: &Game<'_>, selector: &str, biome: &str) -> bool {
    let Some(tag) = selector.strip_prefix('#') else {
        return selector == biome;
    };
    let entries = game.tag_entries("worldgen/biome", tag);
    entries.iter().any(|id| id == biome)
        || (entries.is_empty() && game.res.mob_data().biome_has(tag, biome))
}

fn spawn_variants(
    game: &Game<'_>,
    kind: &str,
    position: horizon_core::geometry::Vec3,
) -> Option<(Variants, String)> {
    let registry = registry(kind)?;
    let variants = Variants::load(&game.options.resources, &game.pack_files(), registry);
    let biome = game.biome_at(position.components()).to_owned();
    Some((variants, biome))
}

pub(super) fn select_variant(
    game: &mut Game<'_>,
    kind: &str,
    position: horizon_core::geometry::Vec3,
) -> Option<String> {
    let (variants, biome) = spawn_variants(game, kind, position)?;
    let candidates = variants.candidates(|selector| biome_matches(game, selector, &biome));
    choose(candidates, &mut game.world.random)
}

pub(super) fn choose_spawn_variant(
    game: &Game<'_>,
    mob: &horizon_client::mobs::Mob,
    random: &mut impl RandomSource,
) -> Option<String> {
    let (variants, biome) = spawn_variants(game, &mob.kind.id, mob.position)?;
    choose(variants.candidates(|selector| biome_matches(game, selector, &biome)), random)
}

pub(super) fn select_group_variant(
    game: &Game<'_>,
    mob: &mut horizon_client::mobs::Mob,
) -> Option<String> {
    let (variants, biome) = spawn_variants(game, &mob.kind.id, mob.position)?;
    choose(variants.candidates(|selector| biome_matches(game, selector, &biome)), &mut mob.random)
}

pub(super) fn put_variant(game: &mut Game<'_>, entity_id: u32, variant: &str) {
    let tag = game
        .persist
        .raw
        .entry(entity_id)
        .or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = tag {
        fields.insert("Variant".into(), Tag::String(variant.into()));
    }
}

pub(super) fn stage_offspring(game: &mut Game<'_>, parent: u32) {
    let Some(kind) = game.mobs.iter().find(|mob| mob.id == parent).map(|mob| mob.kind.id.as_str()) else {
        return;
    };
    if registry(kind).is_none() {
        return;
    }
    let variant = horizon_client_core::nbt_value::text(
        game.persist
            .raw
            .get(&parent)
            .and_then(|tag| tag.get("Variant")),
    )
    .unwrap_or_else(|| kind.to_owned());
    // The stock offspring path immediately allocates this id without
    // finalizeSpawn. A cancelled join discards the same raw tag.
    put_variant(game, game.next_entity + 1, &variant);
}

pub(super) fn assign_variant_at(game: &mut Game<'_>, index: usize) {
    let mob = &game.mobs[index];
    if registry(&mob.kind.id).is_none() {
        return;
    }
    if game.persist.raw.get(&mob.id).is_some_and(|tag| tag.get("Variant").is_some()) {
        return;
    }
    let id = mob.id;
    let position = mob.position;
    if mob.kind.id == "naturalist:anglerfish"
        && super::natural_spawning::anglerfish_spawn::deep_glow(game, position.components().map(|p| p.floor() as i32))
    {
        put_variant(game, id, "naturalist:glow");
        return;
    }
    let kind = mob.kind.id.clone();
    if let Some(variant) = select_variant(game, &kind, position) {
        put_variant(game, id, &variant);
    }
}

const BREED_PARTNER: horizon_world::attachments::AttachmentType<u32> =
    horizon_world::attachments::AttachmentType {
        namespace: "naturalist",
        name: "breed_partner",
        default: || 0,
        codec: None,
        copy_on_death: false,
        sync: false,
    };

pub(super) fn record_breeding_partner(game: &mut Game<'_>, parent: u32, partner: u32) {
    if game
        .mobs
        .iter()
        .any(|mob| mob.id == parent && registry(&mob.kind.id).is_some())
    {
        let uuid = game.uuid_for(parent);
        let _ = game
            .entity_attachments
            .entry(uuid)
            .or_default()
            .set(&BREED_PARTNER, partner);
    }
}

pub(super) fn bred_coat(game: &mut Game<'_>, parent: u32) -> Option<bool> {
    let mob = game
        .mobs
        .iter_mut()
        .find(|mob| mob.id == parent && registry(&mob.kind.id).is_some())?;
    let kind = mob.kind.id.clone();
    let coin = mob.random.next_boolean();
    let partner = game
        .persist
        .uuids
        .get(&parent)
        .and_then(|uuid| game.entity_attachments.get_mut(uuid))
        .and_then(|attachments| attachments.remove(&BREED_PARTNER).ok().flatten());
    let source = partner
        .filter(|id| {
            !coin
                && game
                    .mobs
                    .iter()
                    .any(|mob| mob.id == *id && mob.kind.id == kind)
        })
        .unwrap_or(parent);
    stage_offspring(game, source);
    if matches!(kind.as_str(), "naturalist:capybara" | "naturalist:rat")
        && let Some(parent) = game.mobs.iter().find(|mob| mob.id == parent && mob.extra.tame)
        && let Some(owner) = parent.extra.owner
        && let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&(game.next_entity + 1))
    {
        fields.insert("Owner".into(), Tag::IntArray(owner.to_vec()));
        let color = if parent.extra.color <= 15 { parent.extra.color } else { 14 };
        if kind == "naturalist:capybara" {
            fields.insert("DyeColor".into(), Tag::Int(i32::from(color)));
        }
    }
    Some(coin)
}

use horizon_world::{
    attachments::{AttachmentCodec, AttachmentError, AttachmentType, Holder},
    nbt::Compound,
};

#[derive(Clone, Default, PartialEq, Eq)]
struct Appearance {
    variant: String,
    adult: String,
    fawn: String,
    refreshed: bool,
    dye: Option<u8>,
    sitting: bool,
    sleep: u8,
    missing_tail: bool,
    gray: bool,
    swim_target: bool,
    clam_open: bool,
    clam_treasure: bool,
    crab_pose: u8,
    bear_pose: u8,
}

const APPEARANCE: AttachmentType<Appearance> = AttachmentType {
    namespace: "naturalist",
    name: "deer_appearance",
    default: Appearance::default,
    codec: Some(AttachmentCodec {
        encode: encode_appearance,
        decode: decode_appearance,
    }),
    copy_on_death: false,
    sync: true,
};

fn encode_appearance(appearance: &Appearance) -> Result<Compound, AttachmentError> {
    Ok([
        (
            "variant".into(),
            Tag::String(appearance.variant.as_str().into()),
        ),
        (
            "adult".into(),
            Tag::String(appearance.adult.as_str().into()),
        ),
        ("fawn".into(), Tag::String(appearance.fawn.as_str().into())),
        ("dye".into(), Tag::Int(appearance.dye.map_or(-1, i32::from))),
        ("sitting".into(), Tag::Byte(i8::from(appearance.sitting))),
        ("gray".into(), Tag::Byte(i8::from(appearance.gray))),
        ("angler_target".into(), Tag::Byte(i8::from(appearance.swim_target))),
        ("clam_open".into(), Tag::Byte(i8::from(appearance.clam_open))),
        ("clam_treasure".into(), Tag::Byte(i8::from(appearance.clam_treasure))),
        ("crab_pose".into(), Tag::Byte(appearance.crab_pose as i8)),
        ("sleep".into(), Tag::Byte(appearance.sleep as i8)),
        ("bear_pose".into(), Tag::Int(i32::from(appearance.bear_pose))),
        ("missing_tail".into(), Tag::Byte(i8::from(appearance.missing_tail))),
    ]
    .into_iter()
    .collect())
}

fn decode_appearance(data: &Compound) -> Result<Appearance, AttachmentError> {
    let value = |key: &str| {
        horizon_client_core::nbt_value::text(data.get(&key.into())).ok_or(AttachmentError::Decode)
    };
    // Re-resolve entries read from a world save against its current pack stack.
    Ok(Appearance {
        variant: value("variant")?,
        adult: value("adult")?,
        fawn: value("fawn")?,
        refreshed: false,
        dye: horizon_client_core::nbt_value::number(data.get(&"dye".into()))
            .and_then(|number| u8::try_from(number).ok()).filter(|id| *id <= 15),
        sitting: matches!(data.get(&"sitting".into()), Some(Tag::Byte(1))),
        gray: matches!(data.get(&"gray".into()), Some(Tag::Byte(1))),
        swim_target: matches!(data.get(&"angler_target".into()), Some(Tag::Byte(1))),
        clam_open: matches!(data.get(&"clam_open".into()), Some(Tag::Byte(1))),
        clam_treasure: matches!(data.get(&"clam_treasure".into()), Some(Tag::Byte(1))),
        crab_pose: match data.get(&"crab_pose".into()) { Some(Tag::Byte(bits)) => *bits as u8, _ => 0 },
        bear_pose: horizon_client_core::nbt_value::number(data.get(&"bear_pose".into()))
            .and_then(|value| u8::try_from(value).ok()).unwrap_or(0),
        missing_tail: matches!(data.get(&"missing_tail".into()), Some(Tag::Byte(1))),
        sleep: horizon_client_core::nbt_value::number(data.get(&"sleep".into()))
            .and_then(|value| u8::try_from(value).ok()).unwrap_or(0),
    })
}

pub(super) fn refresh_appearances(game: &mut Game<'_>, force: bool) {
    let mut loaded = BTreeMap::new();
    for index in 0..game.mobs.len() {
        let mob = &game.mobs[index];
        let Some(registry) = registry(&mob.kind.id) else { continue };
        if mob.removed {
            continue;
        }
        let kind = mob.kind.id.as_str();
        let bear = horizon_client::mobs::naturalist_bears::is_bear(kind);
        let dye = ((bear || matches!(kind, DUCK | "naturalist:ostrich" | "naturalist:bird" | "naturalist:capybara" | "naturalist:lizard" | "naturalist:hedgehog")) && mob.extra.color <= 15).then_some(mob.extra.color);
        let sitting = (bear || matches!(kind, DUCK | "naturalist:ostrich" | "naturalist:bird" | "naturalist:capybara" | "naturalist:lizard" | "naturalist:hippo" | "naturalist:hedgehog")) && mob.extra.sitting_pose;
        let bear_pose = if bear {
            horizon_client::mobs::naturalist_bears::render_pose(mob)
        } else {
            0
        };
        let missing_tail = kind == "naturalist:lizard"
            && horizon_client::mobs::naturalist_lizard::missing_tail(&mob.ai.nav);
        let sleep = if kind == "naturalist:ostrich" {
            let pose = horizon_client::mobs::naturalist_ostrich::state(mob);
            u8::from(pose.hiding) | (u8::from(mob.extra.sprinting) << 1) | (u8::from(pose.attack_pose_ticks > 0) << 2)
        } else if kind == "naturalist:mole" {
            horizon_client::mobs::goals::naturalist_mole_goals::phase(mob) as u8
        } else if kind == "naturalist:dirt_trail" {
            mob.extra.variant
        } else if kind == "naturalist:hedgehog" {
            horizon_client::mobs::naturalist_hedgehog::render_bits(mob)
        } else if matches!(kind, "naturalist:tiger" | "naturalist:lion") {
            horizon_client::mobs::goals::naturalist_cat_goals::render_pose(
                mob, game.persist.uuids.get(&mob.id).copied()
            )
        } else if kind == "naturalist:rat" {
            horizon_client::mobs::goals::naturalist_rat_goals::render_pose(mob)
        } else if kind == "naturalist:snake" {
            horizon_client::mobs::goals::naturalist_snake_goals::render_pose(mob)
        } else if kind == "naturalist:tortoise" {
            horizon_client::mobs::goals::naturalist_tortoise_goals::render_pose(mob)
        } else if kind == "naturalist:capybara" {
            horizon_client::mobs::naturalist_capybara_sleep::pose(&mob.ai.nav)
        } else if kind == "naturalist:rhino" {
            use horizon_client::mobs::naturalist_rhino;
            if naturalist_rhino::stun_ticks(&mob.ai.nav) > 0 { 2 }
            else { u8::from(naturalist_rhino::preparing(&mob.ai.nav)) }
        } else if kind == "naturalist:komodo_dragon" {
            u8::from(horizon_client::mobs::naturalist_komodo::basking(&mob.ai.nav))
        } else {
            0
        };
        let shell = (kind == "naturalist:clam")
            .then(|| horizon_client::mobs::nav::naturalist_clam::state(&mob.ai.nav));
        let clam_open = shell.is_some_and(|shell| shell.open);
        let clam_treasure = shell.is_some_and(|shell| shell.has_treasure);
        let crab_pose = crab_pose(mob);
        let swim_target = matches!(kind, "naturalist:anglerfish" | "naturalist:great_white_shark") && mob.ai.target.is_some();
        let gray = kind == "naturalist:blobfish"
            && horizon_client::mobs::nav::naturalist_blobfish_state::gray(&mob.ai.nav);
        let id = mob.id;
        let saved_variant = game.persist.raw.get(&id).and_then(|tag| tag.get("Variant"));
        let variant = horizon_client_core::nbt_value::text(saved_variant)
        .or_else(|| {
            use horizon_client::mobs::goals::{naturalist_cat_goals, naturalist_tortoise_goals};
            let colors: &[&str] = match kind {
                "naturalist:tiger" => &naturalist_cat_goals::COLORS,
                "naturalist:tortoise" => &naturalist_tortoise_goals::COLORS,
                _ => return None,
            };
            horizon_client_core::nbt_value::number(saved_variant)
                .map(|legacy| colors[legacy.rem_euclid(colors.len() as i64) as usize].to_owned())
        })
        .unwrap_or_else(|| match kind {
            "naturalist:anglerfish" => "naturalist:red",
            "naturalist:starfish" => "naturalist:orange",
            "naturalist:mole" | "naturalist:dirt_trail" => "naturalist:mole",
            "naturalist:rat" => "naturalist:black",
            "naturalist:tiger" => "naturalist:black_panther",
            "naturalist:clam" | "naturalist:tortoise" | "naturalist:hedgehog" => "naturalist:brown",
            "naturalist:crab" => "naturalist:blue",
            "naturalist:giant_isopod" => "naturalist:brown",
            "naturalist:ray" => "naturalist:eagle_ray",
            "naturalist:jellyfish" => "naturalist:white",
            "naturalist:bird" => "naturalist:american_robin",
            "naturalist:jungle_scorpion" => "naturalist:black",
            "naturalist:lizard" | "naturalist:lizard_tail" => "naturalist:green",
            _ => kind,
        }.to_owned());
        if kind == "naturalist:bass" {
            game.mobs[index].extra.variant = horizon_client::mobs::nav::naturalist_bass_state::tier(&variant);
        } else if kind == "naturalist:snake" {
            game.mobs[index].extra.variant = horizon_client::mobs::goals::naturalist_snake_goals::variant_index(&variant);
        } else if kind == "naturalist:tortoise" {
            game.mobs[index].extra.variant = horizon_client::mobs::goals::naturalist_tortoise_goals::variant_index(&variant);
        }
        let stored_uuid = game.persist.uuids.get(&id).copied();
        let old = stored_uuid
            .and_then(|uuid| game.entity_attachments.get_mut(&uuid))
            .and_then(|map| map.get_existing(&APPEARANCE).ok().flatten());
        if !force
            && old
                .as_ref()
                .is_some_and(|appearance| appearance.refreshed && appearance.variant == variant
                    && appearance.dye == dye && appearance.sitting == sitting && appearance.sleep == sleep
                    && appearance.missing_tail == missing_tail && appearance.gray == gray && appearance.bear_pose == bear_pose
                    && appearance.swim_target == swim_target
                    && appearance.crab_pose == crab_pose
                    && appearance.clam_open == clam_open && appearance.clam_treasure == clam_treasure)
        {
            continue;
        }

        let variants = loaded.entry(registry)
            .or_insert_with(|| Variants::load(&game.options.resources, &game.pack_files(), registry));
        let appearance = variants.0.get(&variant).map_or_else(
            || Appearance {
                variant: variant.clone(),
                refreshed: true,
                dye,
                sitting,
                sleep,
                missing_tail,
                gray,
                swim_target,
                clam_open,
                clam_treasure,
                crab_pose,
                bear_pose,
                ..Default::default()
            },
            |definition| Appearance {
                variant: variant.clone(),
                adult: definition.texture.clone(),
                fawn: definition
                    .baby_texture
                    .clone()
                    .unwrap_or_else(|| definition.texture.clone()),
                refreshed: true,
                dye,
                sitting,
                sleep,
                missing_tail,
                gray,
                swim_target,
                clam_open,
                clam_treasure,
                crab_pose,
                bear_pose,
            },
        );
        if old.as_ref() != Some(&appearance) {
            let uuid = game.uuid_for(id);
            if game
                .entity_attachments
                .entry(uuid)
                .or_default()
                .set(&APPEARANCE, appearance)
                .is_ok()
            {
                game.world.attachment_syncs.insert(Holder::Entity(id));
            }
        }
    }
}

fn appearance(game: &Game<'_>, id: u32) -> Option<Appearance> {
    let attachments = if let Some(remote) = game.remote.as_ref() {
        remote.attachments.get(&Holder::Entity(id))
    } else {
        game.persist
            .uuids
            .get(&id)
            .and_then(|uuid| game.entity_attachments.get(uuid))
    }?;
    let data = Tag::Compound(attachments.to_nbt());
    let Tag::Compound(saved) = data.get(APPEARANCE.namespace)?.get(APPEARANCE.name)?.get("value")? else {
        return None;
    };
    decode_appearance(saved).ok()
}

pub(super) fn appearance_texture(game: &Game<'_>, id: u32, baby: bool) -> Option<String> {
    let appearance = appearance(game, id)?;
    let texture = if baby { appearance.fawn } else { appearance.adult };
    (!texture.is_empty()).then_some(texture)
}

pub(super) fn duck_appearance(game: &Game<'_>, id: u32, baby: bool) -> (Option<String>, Option<u8>, bool) {
    let Some(appearance) = appearance(game, id) else { return (None, None, false) };
    let texture = if appearance.variant == DUCK {
        None
    } else {
        let texture = if baby { appearance.fawn } else { appearance.adult };
        Some(if texture.is_empty() { "naturalist:entity/duck/duck".to_owned() } else { texture })
    };
    (texture, appearance.dye, appearance.sitting)
}

pub(super) fn appearance_variant(game: &Game<'_>, id: u32) -> Option<String> {
    appearance(game, id).map(|appearance| appearance.variant)
}

pub(super) fn capybara_appearance(game: &Game<'_>, id: u32) -> (Option<u8>, bool, u8) {
    appearance(game, id).map_or((None, false, 0), |appearance| {
        (appearance.dye, appearance.sitting, appearance.sleep)
    })
}

pub(super) fn lizard_appearance(game: &Game<'_>, id: u32) -> (Option<u8>, bool, bool) {
    appearance(game, id).map_or((None, false, false), |appearance| {
        (appearance.dye, appearance.sitting, appearance.missing_tail)
    })
}

pub(super) fn animal_rest_pose(game: &Game<'_>, id: u32) -> u8 {
    appearance(game, id).map_or(0, |appearance| appearance.sleep)
}

pub(super) fn bear_appearance(game: &Game<'_>, id: u32) -> (Option<u8>, u8) {
    appearance(game, id).map_or((None, 0), |appearance| (appearance.dye, appearance.bear_pose))
}

pub(super) fn blobfish_appearance(game: &Game<'_>, mob: &horizon_client::mobs::Mob) -> (bool, Option<String>) {
    let synced = appearance(game, mob.id);
    let gray = synced.as_ref().map_or_else(
        || horizon_client::mobs::nav::naturalist_blobfish_state::gray(&mob.ai.nav),
        |data| data.gray,
    );
    let texture = synced.and_then(|data| {
        (data.variant != "naturalist:blobfish" && !data.adult.is_empty()).then_some(data.adult)
    });
    (gray, texture)
}

pub(super) fn angler_appearance(game: &Game<'_>, mob: &horizon_client::mobs::Mob) -> (String, bool, bool) {
    let Some(render) = appearance(game, mob.id) else {
        return ("naturalist:entity/anglerfish/red".to_owned(), false, mob.ai.target.is_some());
    };
    let glowing = render.variant == "naturalist:glow";
    let texture = if render.adult.is_empty() { "naturalist:entity/anglerfish/red".to_owned() } else { render.adult };
    (texture, glowing, render.swim_target)
}

pub(super) fn shark_aggressive(game: &Game<'_>, mob: &horizon_client::mobs::Mob) -> bool {
    appearance(game, mob.id).map_or_else(|| mob.ai.target.is_some(), |state| state.swim_target)
}

pub(super) fn variant_tooltip(
    game: &Game<'_>, kind: &str, id: &str,
) -> Option<horizon_client::text::Text> {
    let variants = Variants::load(&game.options.resources, &game.pack_files(), registry(kind)?);
    let component = variants.0.get(id)?.tooltip.as_ref()?;
    Some(horizon_client_core::component::to_text(component, &mut |selector| horizon_client::text::Text::literal(selector)))
}

pub(super) fn clam_appearance(game: &Game<'_>, mob: &horizon_client::mobs::Mob) -> (String, bool, bool) {
    let fallback = || "naturalist:entity/clam/brown_clam".to_owned();
    let Some(render) = appearance(game, mob.id) else {
        let shell = horizon_client::mobs::nav::naturalist_clam::state(&mob.ai.nav);
        return (fallback(), shell.open, shell.has_treasure);
    };
    let texture = if render.adult.is_empty() { fallback() } else { render.adult };
    (texture, render.clam_open, render.clam_treasure)
}

fn crab_pose(mob: &horizon_client::mobs::Mob) -> u8 {
    use horizon_client::mobs::nav::naturalist_crab as crab;
    if mob.kind.id != crab::KIND { return 0; }
    let status = crab::state(&mob.ai.nav);
    u8::from(crab::hides(mob))
        | (u8::from(status.dancing) << 1)
        | (u8::from(mob.extra.sitting_pose) << 2)
        | (u8::from((mob.age as u64) < status.swing_until) << 3)
}

pub(super) fn crab_appearance(game: &Game<'_>, mob: &horizon_client::mobs::Mob, baby: bool) -> (String, u8) {
    let fallback = || (if baby { "naturalist:entity/crab/blue_crab_baby" } else { "naturalist:entity/crab/blue_crab" }).to_owned();
    let Some(render) = appearance(game, mob.id) else { return (fallback(), crab_pose(mob)); };
    let texture = if baby { render.fawn } else { render.adult };
    let texture = if texture.is_empty() { fallback() } else { texture };
    (texture, render.crab_pose)
}

pub(super) fn hippo_sitting(game: &Game<'_>, mob: &horizon_client::mobs::Mob) -> bool {
    appearance(game, mob.id).map_or(mob.extra.sitting_pose, |pose| pose.sitting)
}
