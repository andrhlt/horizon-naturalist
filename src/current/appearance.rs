//! Deer appearance state shared by the authoritative server and tracked clients.
use mod_api::{AttachmentCodec, AttachmentError, AttachmentType, Compound, Tag};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeerAppearance {
    pub variant: String,
    pub adult_texture: String,
    pub baby_texture: String,
    pub eating: bool,
    pub sprinting: bool,
    pub movement_speed: f32,
}
impl Default for DeerAppearance {
    fn default() -> Self {
        Self {
            variant: "naturalist:deer".into(),
            adult_texture: "naturalist:entity/deer/deer".into(),
            baby_texture: "naturalist:entity/deer/deer_baby".into(),
            eating: false,
            sprinting: false,
            movement_speed: 0.2,
        }
    }
}

pub(crate) static DEER_APPEARANCE: AttachmentType<DeerAppearance> = AttachmentType {
    namespace: "naturalist",
    name: "deer_appearance",
    default: DeerAppearance::default,
    codec: Some(AttachmentCodec { encode, decode }),
    copy_on_death: false,
    sync: true,
};

fn encode(value: &DeerAppearance) -> Result<Compound, AttachmentError> {
    if !value.movement_speed.is_finite()
        || value.movement_speed < 0.0
        || value.variant.is_empty()
        || value.adult_texture.is_empty()
        || value.baby_texture.is_empty()
    {
        return Err(AttachmentError::Encode);
    }
    Ok([
        ("variant".into(), Tag::String(value.variant.as_str().into())),
        (
            "adult_texture".into(),
            Tag::String(value.adult_texture.as_str().into()),
        ),
        (
            "baby_texture".into(),
            Tag::String(value.baby_texture.as_str().into()),
        ),
        ("eating".into(), Tag::Byte(value.eating as i8)),
        ("sprinting".into(), Tag::Byte(value.sprinting as i8)),
        (
            "movement_speed".into(),
            Tag::Float(value.movement_speed.to_bits()),
        ),
    ]
    .into())
}
fn decode(data: &Compound) -> Result<DeerAppearance, AttachmentError> {
    let string = |name: &str| match data.get(&name.into()) {
        Some(Tag::String(s)) if !s.to_string_lossy().is_empty() => Ok(s.to_string_lossy()),
        _ => Err(AttachmentError::Decode),
    };
    let flag = |name: &str| match data.get(&name.into()) {
        Some(Tag::Byte(0)) => Ok(false),
        Some(Tag::Byte(1)) => Ok(true),
        _ => Err(AttachmentError::Decode),
    };
    let speed = match data.get(&"movement_speed".into()) {
        Some(Tag::Float(bits)) => f32::from_bits(*bits),
        _ => return Err(AttachmentError::Decode),
    };
    if !speed.is_finite() || speed < 0.0 {
        return Err(AttachmentError::Decode);
    }
    Ok(DeerAppearance {
        variant: string("variant")?,
        adult_texture: string("adult_texture")?,
        baby_texture: string("baby_texture")?,
        eating: flag("eating")?,
        sprinting: flag("sprinting")?,
        movement_speed: speed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mod_api::{Attachments, Holder};

    #[test]
    fn synced_appearance_survives_an_encoded_snapshot() {
        let mut server = Attachments::default();
        let holder = Holder::Entity(17);
        assert_eq!(
            DEER_APPEARANCE.get_existing(&mut server, holder).unwrap(),
            None
        );
        let expected = DeerAppearance {
            variant: "example:woodland".into(),
            adult_texture: "naturalist:entity/deer/white_deer".into(),
            baby_texture: "naturalist:entity/deer/reindeer".into(),
            eating: true,
            sprinting: true,
            movement_speed: 0.31,
        };
        DEER_APPEARANCE
            .set(&mut server, holder, expected.clone())
            .unwrap();
        let mut client = Attachments::from_nbt(server.to_nbt());
        assert_eq!(
            DEER_APPEARANCE.get_existing(&mut client, holder).unwrap(),
            Some(expected)
        );
        let mut another_entity = Attachments::default();
        assert_eq!(
            DEER_APPEARANCE
                .get_existing(&mut another_entity, Holder::Entity(18))
                .unwrap(),
            None
        );
        assert!(DEER_APPEARANCE.sync);
    }
    #[test]
    fn invalid_state_is_an_error_and_does_not_replace_existing_value() {
        let mut store = Attachments::default();
        let holder = Holder::Entity(1);
        let original = DeerAppearance::default();
        DEER_APPEARANCE
            .set(&mut store, holder, original.clone())
            .unwrap();
        let bad = DeerAppearance {
            movement_speed: f32::NAN,
            ..original.clone()
        };
        assert!(DEER_APPEARANCE.set(&mut store, holder, bad).is_err());
        assert_eq!(
            DEER_APPEARANCE.get_existing(&mut store, holder).unwrap(),
            Some(original)
        );
    }
}
