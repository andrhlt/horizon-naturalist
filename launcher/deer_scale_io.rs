use super::Game;
use horizon_client::mobs::Mob;
use horizon_world::{
    attributes::{Modifier, Operation, Packed},
    nbt::Tag,
};

pub(super) const SCALE: &str = "minecraft:scale";

fn number(tag: &Tag) -> Option<f64> {
    match *tag {
        Tag::Double(value) => Some(f64::from_bits(value)),
        Tag::Float(value) => Some(f64::from(f32::from_bits(value))),
        Tag::Long(value) => Some(value as f64),
        Tag::Int(value) => Some(f64::from(value)),
        Tag::Short(value) => Some(f64::from(value)),
        Tag::Byte(value) => Some(f64::from(value)),
        _ => None,
    }
}

fn string(tag: &Tag) -> Option<String> {
    match tag {
        Tag::String(value) => Some(value.to_string_lossy()),
        _ => None,
    }
}

fn operation_name(operation: Operation) -> &'static str {
    match operation {
        Operation::AddValue => "add_value",
        Operation::AddMultipliedBase => "add_multiplied_base",
        Operation::AddMultipliedTotal => "add_multiplied_total",
    }
}

fn modifier(tag: &Tag) -> Option<Modifier> {
    let id = string(tag.get("id")?)?;
    let amount = number(tag.get("amount")?)?;
    let name = string(tag.get("operation")?)?;
    let operation = Operation::ALL.into_iter().find(|op| operation_name(*op) == name)?;
    Modifier::new(&id, amount, operation).ok()
}

fn is_scale(tag: &Tag) -> bool {
    matches!(tag.get("id").and_then(string).as_deref(), Some("minecraft:scale" | "scale"))
}

pub(super) fn is_naturalist(tag: &Tag) -> bool {
    tag.get("id").and_then(string).is_some_and(|id| id.starts_with("naturalist:"))
}

pub(super) fn read(tag: &Tag) -> Option<Packed> {
    let Tag::List(attributes) = tag.get("attributes")? else { return None };
    let scale = attributes.iter().find(|entry| is_scale(entry))?;
    let base = number(scale.get("base")?)?;
    let modifiers = match scale.get("modifiers") {
        Some(Tag::List(entries)) => entries.iter().filter_map(modifier).collect(),
        _ => Vec::new(),
    };
    Some(Packed { base, modifiers })
}

pub(super) fn save(game: &Game<'_>, mob: &Mob, tag: &mut Tag) {
    use super::commands::Target;

    if !mob.kind.id.starts_with("naturalist:") {
        return;
    }
    let Some(attributes) = game.cmds.attributes.get(&Target::Mob(mob.id)) else { return };
    let Some(instance) = attributes.catalog().id(SCALE).and_then(|key| attributes.instance(key)) else {
        return;
    };
    let packed = instance.pack();
    let modifiers = packed.modifiers.iter().map(|modifier| {
        Tag::compound([
            ("id", Tag::String(modifier.id().into())),
            ("amount", Tag::Double(modifier.amount().to_bits())),
            ("operation", Tag::String(operation_name(modifier.operation()).into())),
        ])
    }).collect();
    let Tag::Compound(fields) = tag else { return };
    let mut list = match fields.remove(&"attributes".into()) {
        Some(Tag::List(entries)) => entries,
        _ => Vec::new(),
    };
    list.retain(|entry| !is_scale(entry));
    list.push(Tag::compound([
        ("id", Tag::String(SCALE.into())),
        ("base", Tag::Double(packed.base.to_bits())),
        ("modifiers", Tag::List(modifiers)),
    ]));
    fields.insert("attributes".into(), Tag::List(list));
}

pub(super) fn restore(game: &mut Game<'_>, id: u32, packed: Option<Packed>) {
    use super::commands::Target;

    let Some(mob) = game.mobs.iter().find(|mob| mob.id == id && mob.kind.id.starts_with("naturalist:")) else {
        return;
    };
    let target = Target::Mob(id);
    if packed.is_none() && !game.cmds.attributes.contains_key(&target) {
        return;
    }
    if !game.cmds.attributes.contains_key(&target) {
        let Some(mut attributes) = game.motion.attributes.create("minecraft:cow") else { return };
        if let Some(step) = attributes.catalog().id("minecraft:step_height") {
            attributes.edit(step, |state, observer| state.set_base_value(mob.kind.step_height, observer));
        }
        game.cmds.attributes.insert(target, attributes);
    }
    let Some(attributes) = game.cmds.attributes.get_mut(&target) else { return };
    let Some(key) = attributes.catalog().id(SCALE) else { return };
    if let Some(packed) = packed {
        attributes.edit(key, |state, observer| state.apply(&packed, observer));
    }
    let scale = attributes.value(key).unwrap_or(1.0);
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
        mob.extra.model_scale = scale as f32;
    }
}
