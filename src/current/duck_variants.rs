use std::{collections::HashMap,sync::{LazyLock,Mutex}};
static CACHE:LazyLock<Mutex<HashMap<String,Option<(String,String)>>>>=LazyLock::new(||Mutex::new(HashMap::new()));
pub fn clear(){CACHE.lock().unwrap_or_else(|e|e.into_inner()).clear();}
pub fn textures(game:&crate::game::Game<'_>, variant:&str, queso:bool)->(String,String){
 if variant=="naturalist:duck" {return (if queso{"naturalist:entity/duck/queso"}else{"naturalist:entity/duck/duck"}.into(),"naturalist:entity/duck/duck_baby".into());}
 let mut cache=CACHE.lock().unwrap_or_else(|e|e.into_inner());
 cache.entry(variant.to_owned()).or_insert_with(||{
  use mod_api::Host;
  let (namespace,path)=variant.split_once(':')?;
  let id=mod_api::Identifier::parse(&format!("{namespace}:naturalist/duck_variant/{path}.json")).ok()?;
  let bytes=game.resource(mod_api::PackType::ServerData,&id)?;
  let doc:serde_json::Value=serde_json::from_slice(&bytes).ok()?;
  let adult=doc.get("texture")?.as_str()?.to_owned();
  let baby=doc.get("baby_texture").and_then(|x|x.as_str()).unwrap_or(&adult).to_owned();Some((adult,baby))
 }).clone().unwrap_or_else(||("naturalist:entity/duck/duck".into(),"naturalist:entity/duck/duck".into()))
}
