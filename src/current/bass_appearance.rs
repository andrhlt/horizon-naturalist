//! Authoritative appearance data shared with clients tracking a bass.
use mod_api::{AttachmentCodec, AttachmentError, AttachmentType, Compound, Tag};

#[derive(Clone, Debug, PartialEq)]
pub struct BassAppearance {
    pub variant: String,
    pub texture: String,
    pub in_water: bool,
    pub tilt: f32,
    pub pitch: f32,
}
impl Default for BassAppearance {
    fn default() -> Self {
        Self {
            variant: "naturalist:bass".into(),
            texture: "naturalist:entity/bass".into(),
            in_water: false,
            tilt: 0.0,
            pitch: 0.0,
        }
    }
}
pub static BASS_APPEARANCE: AttachmentType<BassAppearance> = AttachmentType {
    namespace: "naturalist",
    name: "bass_appearance",
    default: BassAppearance::default,
    codec: Some(AttachmentCodec { encode, decode }),
    copy_on_death: false,
    sync: true,
};
fn encode(value: &BassAppearance) -> Result<Compound, AttachmentError> {
    if value.variant.is_empty()
        || value.texture.is_empty()
        || !value.tilt.is_finite()
        || !value.pitch.is_finite()
    {
        return Err(AttachmentError::Encode);
    }
    Ok([
        ("variant".into(), Tag::String(value.variant.as_str().into())),
        ("texture".into(), Tag::String(value.texture.as_str().into())),
        ("in_water".into(), Tag::Byte(value.in_water as i8)),
        ("tilt".into(), Tag::Float(value.tilt.to_bits())),
        ("pitch".into(), Tag::Float(value.pitch.to_bits())),
    ]
    .into())
}
fn decode(data: &Compound) -> Result<BassAppearance, AttachmentError> {
    let string = |name: &str| match data.get(&name.into()) {
        Some(Tag::String(s)) if !s.to_string_lossy().is_empty() => Ok(s.to_string_lossy()),
        _ => Err(AttachmentError::Decode),
    };
    let number = |name: &str| match data.get(&name.into()) {
        Some(Tag::Float(bits)) if f32::from_bits(*bits).is_finite() => Ok(f32::from_bits(*bits)),
        _ => Err(AttachmentError::Decode),
    };
    let in_water = match data.get(&"in_water".into()) {
        Some(Tag::Byte(0)) => false,
        Some(Tag::Byte(1)) => true,
        _ => return Err(AttachmentError::Decode),
    };
    Ok(BassAppearance {
        variant: string("variant")?,
        texture: string("texture")?,
        in_water,
        tilt: number("tilt")?,
        pitch: number("pitch")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mod_api::{Attachments, Holder};
    #[test]
    fn tracked_state_round_trips_and_rejects_invalid_values() {
        let mut server = Attachments::default();
        let holder = Holder::Entity(7);
        let expected = BassAppearance {
            variant: "naturalist:bass_large".into(),
            texture: "naturalist:entity/bass_large".into(),
            in_water: true,
            tilt: 4.0,
            pitch: -2.0,
        };
        BASS_APPEARANCE
            .set(&mut server, holder, expected.clone())
            .unwrap();
        let mut client = Attachments::from_nbt(server.to_nbt());
        assert_eq!(
            BASS_APPEARANCE.get_existing(&mut client, holder).unwrap(),
            Some(expected.clone())
        );
        assert!(
            BASS_APPEARANCE
                .set(
                    &mut server,
                    holder,
                    BassAppearance {
                        pitch: f32::NAN,
                        ..expected
                    }
                )
                .is_err()
        );
        assert!(BASS_APPEARANCE.sync);
    }
}
