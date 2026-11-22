//! Duck appearance exchanged between the authoritative server and tracking clients.
use mod_api::{AttachmentCodec, AttachmentError, AttachmentType, Compound, Tag};
#[derive(Clone, Debug, PartialEq)]
pub struct DuckAppearance { pub color: i32, pub sitting: bool, pub in_water: bool, pub sprinting: bool, pub on_ground: bool, pub speed: f32, pub adult_texture: String, pub baby_texture: String }
impl Default for DuckAppearance { fn default()->Self { Self { color:-1, sitting:false, in_water:false, sprinting:false, on_ground:true, speed:0.25, adult_texture:"naturalist:entity/duck/duck".into(), baby_texture:"naturalist:entity/duck/duck_baby".into() } } }
pub static DUCK_APPEARANCE: AttachmentType<DuckAppearance> = AttachmentType { namespace:"naturalist", name:"duck_appearance", default:DuckAppearance::default, codec:Some(AttachmentCodec { encode, decode }), copy_on_death:false, sync:true };
fn encode(a:&DuckAppearance)->Result<Compound,AttachmentError>{
 if !(-1..16).contains(&a.color) || !a.speed.is_finite() || a.speed<0.0 || a.adult_texture.is_empty() || a.baby_texture.is_empty() { return Err(AttachmentError::Encode); }
 Ok([("adult".into(),Tag::String(a.adult_texture.as_str().into())),("baby".into(),Tag::String(a.baby_texture.as_str().into())),("color".into(),Tag::Int(a.color)),("sitting".into(),Tag::Byte(a.sitting as i8)),("water".into(),Tag::Byte(a.in_water as i8)),("sprinting".into(),Tag::Byte(a.sprinting as i8)),("ground".into(),Tag::Byte(a.on_ground as i8)),("speed".into(),Tag::Float(a.speed.to_bits()))].into())
}
fn decode(c:&Compound)->Result<DuckAppearance,AttachmentError>{
 let flag=|k:&str|match c.get(&k.into()){Some(Tag::Byte(0))=>Ok(false),Some(Tag::Byte(1))=>Ok(true),_=>Err(AttachmentError::Decode)};
 let color=match c.get(&"color".into()){Some(Tag::Int(v)) if (-1..16).contains(v)=>*v,_=>return Err(AttachmentError::Decode)};
 let speed=match c.get(&"speed".into()){Some(Tag::Float(v))=>f32::from_bits(*v),_=>return Err(AttachmentError::Decode)};
 if !speed.is_finite() || speed<0.0 {return Err(AttachmentError::Decode)}
 let string=|k:&str|match c.get(&k.into()){Some(Tag::String(v)) if !v.to_string_lossy().is_empty()=>Ok(v.to_string_lossy()),_=>Err(AttachmentError::Decode)};
 Ok(DuckAppearance{color,adult_texture:string("adult")?,baby_texture:string("baby")?,sitting:flag("sitting")?,in_water:flag("water")?,sprinting:flag("sprinting")?,on_ground:flag("ground")?,speed})
}
#[cfg(test)] mod tests { use super::*; #[test] fn color_and_pose_roundtrip(){ let a=DuckAppearance{color:14,sitting:true,in_water:true,sprinting:false,on_ground:false,speed:0.25,..Default::default()}; assert_eq!(decode(&encode(&a).unwrap()).unwrap(),a); } }
