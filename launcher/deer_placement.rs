use std::cell::Cell;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Pass {
    Natural,
    Generation,
    Spawner,
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Substrate {
    Vanilla,
    Bird,
    Deer,
    Duck,
    Scorpion,
    Komodo,
    Vulture,
    Butterfly,
    Dragonfly,
    Firefly,
    Crab,
    Hippo,
    Mammoth,
}

#[derive(Clone, Copy)]
struct Probe {
    pass: Pass,
    substrate: Substrate,
    position: Option<[i32; 3]>,
}

thread_local! {
    static PLACEMENT: Cell<Option<Probe>> = const { Cell::new(None) };
}

pub(super) struct PlacementPass(Option<Probe>);

impl PlacementPass {
    pub(super) fn active(pass: Pass) -> bool {
        PLACEMENT.get().is_some_and(|probe| probe.pass == pass)
    }

    pub(super) fn enter(pass: Pass) -> Option<Self> {
        PLACEMENT.with(|current| match current.get() {
            Some(active) if active.pass == pass => None,
            previous => {
                current.set(Some(Probe { pass, substrate: Substrate::Vanilla, position: None }));
                Some(Self(previous))
            }
        })
    }
}

impl Drop for PlacementPass {
    fn drop(&mut self) {
        PLACEMENT.set(self.0);
    }
}

pub(super) fn select(entity: &str) {
    PLACEMENT.with(|current| {
        if let Some(mut probe) = current.get() {
            let substrate = match entity {
                "naturalist:bird" => Substrate::Bird,
                "naturalist:deer"
                | "naturalist:alligator"
                | "naturalist:snail"
                | "naturalist:giraffe"
                | "naturalist:capybara"
                | "naturalist:lizard"
                | "naturalist:rhino"
                | "naturalist:bear"
                | "naturalist:black_bear"
                | "naturalist:boar"
                | "naturalist:turkey"
                | "naturalist:zebra"
                | "naturalist:lion"
                | "naturalist:mole"
                | "naturalist:snake"
                | "naturalist:rat"
                | "naturalist:tiger"
                | "naturalist:tortoise"
                | "naturalist:ostrich"
                | "naturalist:elephant" => Substrate::Deer,
                "naturalist:mammoth" => Substrate::Mammoth,
                "naturalist:duck" => Substrate::Duck,
                "naturalist:desert_scorpion" | "naturalist:jungle_scorpion" => Substrate::Scorpion,
                "naturalist:komodo_dragon" => Substrate::Komodo,
                "naturalist:vulture" => Substrate::Vulture,
                "naturalist:butterfly" => Substrate::Butterfly,
                "naturalist:dragonfly" => Substrate::Dragonfly,
                "naturalist:firefly" => Substrate::Firefly,
                "naturalist:crab" => Substrate::Crab,
                "naturalist:hippo" => Substrate::Hippo,
                _ => Substrate::Vanilla,
            };
            probe.substrate = substrate;
            probe.position = None;
            current.set(Some(probe));
        }
    });
}

pub(super) fn substrate(tag: &str) -> Substrate {
    let active = PLACEMENT
        .get()
        .map_or(Substrate::Vanilla, |probe| probe.substrate);
    match (active, tag) {
        (
            Substrate::Vulture | Substrate::Butterfly | Substrate::Dragonfly,
            "minecraft:parrots_spawnable_on",
        ) => active,
        (
            Substrate::Vulture | Substrate::Butterfly | Substrate::Dragonfly | Substrate::Firefly,
            _,
        ) => Substrate::Vanilla,
        (_, "minecraft:animals_spawnable_on") => active,
        _ => Substrate::Vanilla,
    }
}

pub(super) fn firefly_active() -> bool {
    PLACEMENT
        .get()
        .is_some_and(|probe| probe.substrate == Substrate::Firefly)
}

pub(super) fn record_hippo_position(pos: [i32; 3]) {
    if let Some(mut probe) = PLACEMENT.get()
        && probe.substrate == Substrate::Hippo
    {
        probe.position = Some(pos);
        PLACEMENT.set(Some(probe));
    }
}

pub(super) fn hippo_position() -> Option<(Pass, [i32; 3])> {
    let probe = PLACEMENT.get()?;
    (probe.substrate == Substrate::Hippo).then_some(())?;
    Some((probe.pass, probe.position?))
}

pub(super) fn is_bird_generation() -> bool {
    PLACEMENT.get().is_some_and(|probe| probe.pass == Pass::Generation && probe.substrate == Substrate::Bird)
}
