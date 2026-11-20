use super::{BlockEntity, Game, NativeScreen, OpenNativeScreen, Pos, Screen};
use horizon_client::{
    containers::ModBlockEntityFields,
    mobs::nav::naturalist_elephants::{self, CHEST_SLOTS, INVENTORY_SLOTS},
};
use horizon_protocol::menu_extra::Message;

pub(in crate::game) type GameView<'a> = &'a Game<'a>;

pub(in crate::game) const MENU: &str = "naturalist:elephant";
pub(in crate::game) const UNCHESTED: &str = "naturalist:elephant_unchested";

pub(in crate::game) fn menu_kind(chested: bool) -> &'static str {
    if chested { MENU } else { UNCHESTED }
}

const LOCAL_KEY: Pos = (i32::MIN, 0, 0);

fn description(chested: bool) -> NativeScreen {
    let mut slots: Vec<Option<[f32; 2]>> = (0..CHEST_SLOTS)
        .map(|slot| chested.then_some([80.0 + (slot % 5) as f32 * 18.0,
                                      18.0 + (slot / 5) as f32 * 18.0]))
        .collect();
    slots.extend([Some([8.0, 90.0]), Some([26.0, 90.0])]);
    NativeScreen {
        width: 176.0,
        height: 204.0,
        slots,
        inventory_y: 122.0,
        inventory_x: 8.0,
        own_background: true,
        own_labels: true,
        ..Default::default()
    }
}

impl Game<'_> {
    pub(in crate::game) fn show_naturalist_elephant_inventory(&mut self, key: Pos, extra: Message) {
        let [a, b, c, d, state] = extra.data.as_slice() else { return };
        let mob_id = u32::from_be_bytes([*a, *b, *c, *d]);
        if !self.mobs.iter().any(|mob| mob.id == mob_id && naturalist_elephants::includes(&mob.kind.id)) {
            return;
        }
        self.mod_screen = Some(OpenNativeScreen {
            key,
            window: extra.window,
            block: MENU.to_owned(),
            screen: description(*state != 0),
            data: Vec::new(),
            extra: Some(extra),
        });
    }

    pub(in crate::game) fn open_local_naturalist_elephant_inventory(&mut self, id: u32) {
        let Some(mob) = self.mobs.iter().find(|mob| mob.id == id) else { return };
        let mut items = mob.extra.items.clone();
        items.resize(INVENTORY_SLOTS, None);
        let state = u8::from(mob.extra.chested);
        let [a, b, c, d] = id.to_be_bytes();
        let pos = mob.position.components().map(|n| n.floor() as i32);
        self.close_container();
        self.world.block_entities.insert(LOCAL_KEY, BlockEntity::Mod(Box::new(
            ModBlockEntityFields { items, ..Default::default() }.into()
        )));
        self.container = Some(LOCAL_KEY);
        self.show_naturalist_elephant_inventory(LOCAL_KEY, Message {
            window: 0,
            pos,
            data: vec![a, b, c, d, state],
        });
        self.screen = Screen::Inventory;
        self.capture(false);
    }

    pub(in crate::game) fn naturalist_elephant_inventory(&self) -> Option<(u32, bool)> {
        let screen = self.shown_native_screen().filter(|s| s.block == MENU)?;
        let [a, b, c, d, state] = screen.extra.as_ref()?.data.as_slice() else { return None };
        Some((u32::from_be_bytes([*a, *b, *c, *d]), *state != 0))
    }
}

impl Game<'_> {
    pub(in crate::game) fn sync_local_naturalist_elephant_inventory(&mut self) {
        if self.remote.is_some() || self.container != Some(LOCAL_KEY) {
            return;
        }
        let Some((id, _)) = self.naturalist_elephant_inventory() else { return };
        let Some(BlockEntity::Mod(view)) = self.world.block_entities.get(&LOCAL_KEY) else { return };
        let Some(mob) = self.mobs.iter_mut().find(|mob| mob.id == id) else { return };
        if mob.extra.items != view.items {
            naturalist_elephant_contents(mob, view.items.clone());
            self.world.modified.insert((mob.position.x.floor() as i32 >> 4,
                                        mob.position.z.floor() as i32 >> 4));
        }
    }

    pub(in crate::game) fn close_local_naturalist_elephant_inventory(&mut self) {
        if self.remote.is_none() && self.container == Some(LOCAL_KEY)
            && self.naturalist_elephant_inventory().is_some()
        {
            self.sync_local_naturalist_elephant_inventory();
            self.container = None;
            self.mod_screen = None;
            self.world.block_entities.remove(&LOCAL_KEY);
        }
    }
}

pub(in crate::game) fn naturalist_elephant_contents(
    mob: &mut horizon_client::mobs::Mob,
    mut items: Vec<Option<horizon_client::items::ItemStack>>,
) {
    use horizon_client::mobs::nav::naturalist_elephants::SADDLE_SLOT;
    items.resize(INVENTORY_SLOTS, None);
    mob.set_saddle(items[SADDLE_SLOT].clone());
    mob.extra.items = items;
}
