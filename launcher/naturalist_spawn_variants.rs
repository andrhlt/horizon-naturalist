use horizon_core::random::RandomSource;
use horizon_world::loot::PackOverlay;
use serde::Deserialize;
use std::{cell::RefCell, collections::BTreeMap, path::Path};

#[derive(Deserialize)]
#[serde(untagged)]
enum BiomeSelector {
    One(String),
    Many(Vec<String>),
}

impl BiomeSelector {
    fn matches(&self, matches_biome: &impl Fn(&str) -> bool) -> bool {
        match self {
            Self::One(id) => matches_biome(id),
            Self::Many(ids) => ids.iter().any(|id| matches_biome(id)),
        }
    }
}

#[derive(Deserialize)]
pub struct Variant {
    pub texture: String,
    pub baby_texture: Option<String>,
    pub tooltip: Option<serde_json::Value>,
    biomes: Option<BiomeSelector>,
    #[serde(default = "unit_weight")]
    weight: u32,
    #[serde(default)]
    priority: i32,
}

fn unit_weight() -> u32 {
    1
}

#[derive(Default)]
pub struct Variants(pub BTreeMap<String, Variant>);

impl Variants {
    pub fn load(root: &Path, packs: &PackOverlay<'_>, registry: &str) -> Self {
        let mut variants = BTreeMap::new();
        for namespace in crate::mod_content::namespaces(root) {
            let path = root.join("data").join(&namespace).join(registry);
            if let Ok(folder) = horizon_world::json_folder::read(&path) {
                for file in &folder.files {
                    if let Ok(bytes) = file.bytes()
                        && let Ok(variant) = serde_json::from_slice(&bytes)
                    {
                        variants.insert(format!("{namespace}:{}", file.name), variant);
                    }
                }
            }
        }
        for (id, files) in packs.kind(registry) {
            match serde_json::from_slice(PackOverlay::top(files)) {
                Ok(variant) => {
                    variants.insert(id.to_owned(), variant);
                }
                Err(e) => log::warn!("Naturalist {registry} {id}: {e}"),
            }
        }
        Self(variants)
    }

    pub fn candidates(&self, matches_biome: impl Fn(&str) -> bool) -> Vec<(&String, &Variant)> {
        let mut pool: Vec<_> = self
            .0
            .iter()
            .filter(|(_, v)| {
                v.weight > 0 && v.biomes.as_ref().is_some_and(|b| b.matches(&matches_biome))
            })
            .collect();
        if pool.is_empty() {
            pool.extend(
                self.0
                    .iter()
                    .filter(|(_, v)| v.weight > 0 && v.biomes.is_none()),
            );
        }
        if let Some(priority) = pool.iter().map(|(_, v)| v.priority).max() {
            pool.retain(|(_, v)| v.priority == priority);
        }
        pool
    }
}

pub fn choose(pool: Vec<(&String, &Variant)>, random: &mut impl RandomSource) -> Option<String> {
    let total = pool.iter().try_fold(0i32, |total, (_, v)| {
        total.checked_add(i32::try_from(v.weight).ok()?)
    })?;
    let mut roll = u32::try_from(random.next_int_bound(total).ok()?).ok()?;
    for (id, variant) in pool {
        if roll < variant.weight {
            return Some(id.clone());
        }
        roll -= variant.weight;
    }
    None
}

type PackFiles = Vec<(String, Vec<Vec<u8>>)>;

thread_local! {
    static GENERATION_FILES: RefCell<PackFiles> = const { RefCell::new(Vec::new()) };
    static GENERATION_BIRD: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

pub fn generation_files(pack: PackOverlay<'_>) -> PackFiles {
    pack.files.iter().filter(|(key, _)| {
        if key.starts_with("naturalist/deer_variant/") || key.starts_with("naturalist/bird_variant/") {
            return true;
        }
        key.strip_prefix("tags/").and_then(|tag| tag.split_once(':'))
            .is_some_and(|(_, tag)| tag.starts_with("worldgen/biome/"))
    }).cloned().collect()
}

pub fn with_generation_files(files: PackFiles, generate: impl FnOnce()) {
    struct Restore(PackFiles, Option<bool>);
    impl Drop for Restore {
        fn drop(&mut self) {
            GENERATION_FILES.replace(std::mem::take(&mut self.0));
            GENERATION_BIRD.set(self.1);
        }
    }
    let _restore = Restore(GENERATION_FILES.replace(files), GENERATION_BIRD.replace(Some(false)));
    generate();
}

pub fn generation_variant(
    root: &Path,
    registry: &str,
    biome: &str,
    base_tags: &std::collections::HashMap<String, std::collections::HashSet<String>>,
    random: &mut impl RandomSource,
) -> Option<String> {
    GENERATION_FILES.with_borrow(|files| {
        let pack = PackOverlay { files };
        let variants = Variants::load(root, &pack, registry);
        let override_tags = pack.tag_kind("worldgen/biome").next().and_then(|_| {
            crate::items::load_tags(root, "worldgen/biome", &pack).ok()
        });
        let matches_biome = |selector: &str| {
            let Some(tag) = selector.strip_prefix('#') else { return selector == biome };
            override_tags.as_ref().and_then(|tags| tags.get(tag)).filter(|entries| !entries.is_empty())
                .or_else(|| base_tags.get(tag)).is_some_and(|entries| entries.contains(biome))
        };
        choose(variants.candidates(matches_biome), random)
    })
}

pub fn select_generation_bird(entity: &str) {
    if GENERATION_BIRD.get().is_some() { GENERATION_BIRD.set(Some(entity == "naturalist:bird")); }
}
pub fn generation_is_bird() -> bool { GENERATION_BIRD.get() == Some(true) }
