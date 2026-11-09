//! Hook points for native mods (docs/native-mods.md, "Mods on the hook points"): empty
//! functions the game calls at fixed places, for horizon-mixins bundles to
//! hook with HEAD. Hooks that answer take an out parameter (`claimed`,
//! `zoom`…) so several mods can hook the same point. A native mod keeps its
//! own state in a module of its bundle; the helpers below read and draw what
//! Horizon's own HUD extras do, and keep a mod's settings where the
//! WebAssembly mod of the same id kept them.
//!
//! API 57 adds `native_worn_tint`: a mod tints the opaque worn models of a piece of gear (Mekanism's Color Modulation Unit).
//! API 56 adds `native_fog`: a mod changes the frame's fog (Mekanism's MekaSuit Vision Enhancement Unit pushes the near plane back and tints it).
//! API 55 adds `SlotRule::Check(fn(&ItemStack) -> bool)`: a slot that takes only stacks in a given state (Mekanism's hohlraum slot wants a full hohlraum).
//! API 50 adds a mod item's continuous use (`native_using_tick`, `native_start_using`) and `native_clip`/`native_eye`: a ray through the world's blocks and the eyes of the player (Mekanism's Flamethrower).
//! API 54 adds `native_gear_attributes`, `native_glider` and `native_glide_wear`: attribute modifiers, gliding and its upkeep for worn gear a mod computes (Mekanism's MekaSuit Hydrostatic Repulsor, Gyroscopic Stabilization and Elytra units).
//! API 53 adds `native_player_damage_rules`: a share of the damage of some types a mod's armor takes, paid for afterwards (Mekanism's MekaSuit Inhalation Purification Unit).
//! API 52 adds `native_player_immunities`: damage types a player takes none of (Mekanism's Gas Mask).
//! API 51 adds looping sounds (`native_loop_sound`, `native_stop_loop_sound`): a loop kept alive by calling it each tick (Mekanism's Industrial Alarm and Flamethrower).
//!
//! API 49 adds `native_gui_model`: a mod's world model drawn turning in a GUI (Mekanism's Robit skin previews).
//! API 46 adds `native_mob_types` (mob types of a mod's own, networked as a vanilla base), `native_mobs_tick`, `native_mob_died`, `native_use_mob`, `native_mob_ids`, `native_with_mob` and `Extra::mod_data`, saved with the mob (Mekanism's Robit).
//! API 45 adds `native_keep_chunks`: a mod keeps chunks loaded and ticking for as long as it asks (Mekanism's dimensional stabilizer and anchor upgrade).
//! API 44 adds `native_open_ui` and `native_ui_draw`, `native_ui_click`, `native_ui_key`, `native_ui_wheel`, `native_ui_closed`: a screen of a mod's own, tied to no block (Mekanism's module tweaker and radial selector).
//! API 43 adds `native_set_block`, `native_block_redirect` and `native_block_removed`: a click or a break aimed at one block acts on another (Mekanism's bounding blocks).
//! API 41 adds `native_world_boxes`: translucent coloured boxes drawn in the world (Mekanism's Digital Miner visuals).
//! API 42 adds `NativeScreen::armor` and `offhand`: the player's worn slots on a mod screen (Mekanism's armor sidebar).
//! API 40 adds `native_screen_wheel`: a mod screen can take the mouse wheel (a scrolling list).
//! API 39 adds `native_fill_quad`: a four-cornered shape in one colour (a mod's lightning).
//! API 38 adds `native_slot_rules`: what each slot of a mod screen's menu takes (an energy slot, an output slot).
//! API 37 adds `native_screen_escape`: a mod screen can take Escape (to close only a window of its own).
//! API 36 adds `native_screen_alt` and `NativeScreen::alt`: a screen that swaps between two layouts.
//!
//! API 35 adds `native_mouse_down`, for windows a mod lets the player drag.
//!
//! API 34 adds creative tabs for mods (`native_creative_tabs`) and the pages
//! the creative screen needs for them.
//!
//! API 33 adds text boxes on mod screens (`native_screen_typing`,
//! `native_screen_key`) and `native_set_clipboard`.
//!
//! API 31 adds `native_sprite_tinted`, a GUI sprite drawn in a colour (a
//! mod's tabs and gauges).
//!
//! API 30 adds `native_entities_in`, `native_hurt_mob` and
//! `native_take_item_entity`: the mobs and dropped items in a box, hurting a
//! mob, and picking a dropped item up (a mod's laser and tractor beam).
//!
//! API 29 adds `native_extract_item_where`, taking only the items a mod's
//! filter wants out of a container (a sorter).
//!
//! API 28 adds `native_player_damage`, a player's damage taken by a mod's
//! armor (a powered suit that spends energy on each hit).
//!
//! API 27 adds `native_player_attack`, the damage of a player's melee hit on
//! a mob (a mod's powered weapon).
//!
//! API 26 runs `native_server_payload` as the sending player, their state
//! swapped in (a worn item's fuel spent on the client's word).
//!
//! API 25 adds `native_destroy_speed`, the held item's mining speed on a
//! block (a mod's powered tool).
//!
//! API 24 adds `native_extract_item`, taking items out of a container
//! through a face as a hopper would: with `native_insert_item`, a mod's
//! item pipe.
//!
//! API 23 adds block-entity client sync: a mod block entity's `client`
//! compound reaches the clients with its chunk and when the mod calls
//! `native_sync_block_entity`; a client reads it with
//! `native_client_block_entity` and hears it in
//! `native_client_block_entity_changed`.
//!
//! API 22 adds mod networking: packets on a mod's own channel from the
//! server to a player's client (`native_send_to_player`, heard by
//! `native_client_payload`) and back (`native_send_to_server`,
//! `native_server_payload`).
//!
//! API 21 adds right clicking a block (`native_right_click_block`), and in
//! `native_server.rs` the acting player (`native_acting_player`), a player's
//! health, the action bar (`native_action_bar`) and commands run as the
//! server (`native_run_command`).
//!
//! API 20 adds buttons, tooltips and item icons on mod screens
//! (`native_screen_click`, `native_screen_tooltip`, `native_menu_button`,
//! and `native_item`, `native_screen_title`, `native_data`, `native_block`,
//! `native_block_property`, `native_insert_item`, `native_cursor`).
//!
//! API 19 adds mod-drawn screens for mod blocks (`native_screen`,
//! `native_screen_draw`, and the `native_sprite` helpers), with the mod's own
//! GUI textures.
//!
//! API 18 adds what a machine's tick reaches: a neighbouring mod block entity
//! (`native_with_mod_block_entity`) and its own block's state
//! (`native_set_block_property`).
//!
//! API 17 adds menus for mod containers (the `menu` of a `ModContainer`: vanilla
//! slot grids and furnaces) and has `native_block_entity_tick` take the whole
//! `ModBlockEntity`, with the menu's data slots.
//!
//! API 16 adds item slots for those block entities
//! (`native_block_entity_container`), which hoppers, comparators and
//! breaking the block use as they use vanilla's containers.
//!
//! API 15 adds block entities for a mod's blocks (`native_block_entity`,
//! `native_block_entity_ticks`, `native_block_entity_tick`): made when the
//! block is placed, ticked on the server and saved with the chunk.
//!
//! API 14 adds breaking and placing a block (`native_destroy_block`,
//! `native_place_block`).
//!
//! API 13 adds using an item (`native_use_item_on`, `native_use_item`).
//!
//! API 12 adds item tooltips (`native_tooltip`).
//!
//! API 11 adds server particle emission (`native_send_particles`).
//!
//! API 10 adds server menus (`native_menu_clicked`, `native_menu_closed`,
//! and `native_open_menu`, `native_exchange_items`... in `native_server.rs`).
//!
//! API 9 adds the acting player's plain items (`native_held`,
//! `native_count_items`, `native_take_items`, `native_give_items`... in
//! `native_server.rs`).
//!
//! API 8 adds server commands and players (`native_register_commands`,
//! `native_perform_command`, and the helpers in `native_server.rs`).
//!
//! API 7 adds block-change observations (`native_block_changed`).
//!
//! API 6 adds neighbour-change observations (`native_neighbor_changed`).
//!
//! API 5 adds read-only server block queries (`native_server_block`).
//!
//! API 4 adds authoritative server tick hooks (`native_server_tick`).
//!
//! API 3 adds the world lifecycle hook (`native_world_lifecycle`).
//!
//! API 2 adds the item panel beside container screens and its recipe pages
//! (`native_panel_items`, `native_panel_recipes`, `native_panel_event`),
//! and the game's items and recipes (`native_recipes`,
//! `native_known_recipes`, `native_place_recipe`, `native_give`): what a
//! recipe viewer needs.
//!
//! Names, parameters and call sites are kept from release to release, and
//! hook points are only ever added: each addition raises [`NATIVE_HOOKS_API`].
//! A bundle that hooks only these names the API it needs (`hooks`), and the
//! launcher builds it on any Horizon whose API is at least that one. A change
//! that would break such a bundle is not made.
//! Bodies have no `cfg` and no macros: horizon-mixins refuses a target with
//! either (`tests::hook_points_stay_hookable`).
use super::native_lifecycle::{NativeLifecycleContext, ServerTickPhase, WorldPhase};
use super::{
    Game, GameLevel, LevelDimension, Screen,
    execution_role::ExecutionRole,
    native_panel_recipes::RecipePage,
    native_server::{NativeCommand, NativeServerPlayer},
};
use horizon_client::{
    block_entities::Pos,
    chat::ChatSource,
    containers::{BlockEntity, MOD_MENUS, ModBlockEntity, ModContainer},
    extras::Corner,
    extras_hud::Corners,
    fast_hash::FxHashMap,
    hud::{HudBatch, rgba},
    items::ItemStack,
    keybinds,
    mod_list::{self, Installed, NativePage},
    particles::{ParticleOptions, ParticleType},
    text::Text,
};
use horizon_core::{geometry::Direction, position::BlockPos};
use horizon_world::{
    level::{BlockId, flags},
    nbt::Compound,
    registry::BlockStateId,
};
use std::{fs, path::PathBuf, sync::Arc, time::{Duration, Instant}};
use winit::{event::MouseButton, keyboard::KeyCode};

/// An item a mod wants drawn in the world, in its GUI form (a bin's contents). API 47.
#[derive(Clone, Debug)]
pub struct NativeWorldItem {
    /// The item id (`minecraft:diamond`).
    pub item: String,
    /// Item space (one unit across, as the GUI draws it) to the world, column major.
    pub matrix: [f64; 16],
    /// The block whose light lights it.
    pub light_at: [i32; 3],
    /// The seed vanilla's item model resolver takes.
    pub seed: i32,
    /// The model's display transform: the GUI one, or the one a dropped item has (`ItemDisplayContext.GROUND`).
    pub ground: bool,
}

/// A line of text a mod wants drawn in the world (`submitText`). API 47.
#[derive(Clone, Debug)]
pub struct NativeWorldText {
    pub text: String,
    /// Text space (one unit a font pixel, y down) to the world, column major.
    pub matrix: [f64; 16],
    pub x: f32,
    pub y: f32,
    /// ARGB.
    pub color: u32,
    pub light_at: [i32; 3],
}

/// A flat quad a mod wants drawn in the world (`native_world_quads`: a lightning bolt's strips, a billboard).
/// API 47.
#[derive(Clone, Debug)]
pub struct NativeWorldQuad {
    /// The four corners, counter-clockwise seen from the front; it is drawn from both sides.
    pub corners: [[f64; 3]; 4],
    /// ARGB, tinting the sprite.
    pub color: u32,
    /// A sprite of the GUI atlas; none for the plain colour.
    pub sprite: Option<String>,
    /// The part of the sprite, `[u0, v0, u1, v1]` from 0 to 1: corner 0 gets (u0, v1), 1 (u1, v1), 2 (u1, v0) and
    /// 3 (u0, v0).
    pub uv: [f32; 4],
}

/// A translucent box a mod has drawn in the world (`native_world_boxes`): its corners in world
/// coordinates and the ARGB colour of each face (east, west, up, down, south, north).
#[derive(Clone, Debug)]
pub struct NativeWorldBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub colors: [u32; 6],
    /// A sprite of the GUI atlas (`block/water_still`, a content mod's `<ns>:gui/...`) laid over every face, one
    /// tile per block as `RenderResizableCuboid` does a fluid; none for the plain colour. API 43.
    pub sprite: Option<String>,
}

/// A model a mod draws in the world (`native_world_models`): `model` is `<ns>:<name>` of a part tree in
/// `assets/<ns>/horizon/world_models/<name>.json` (pixels, y up, as it stands) with its texture in
/// `textures/entity/equipment/horizon_models/<name>.png`. It is placed at `position` (world), turned by `yaw`
/// about y, then `pitch` about x, then `roll` about its own z (degrees), and lit as the block `light_at`.
#[derive(Clone, Debug)]
pub struct NativeWorldModel {
    pub model: String,
    pub position: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub light_at: [i32; 3],
    /// ARGB the model's texture is multiplied by (a tint, a fade); `0xFFFF_FFFF` for none.
    pub color: u32,
    /// A rotation (x, y, z, w of a unit quaternion) used in place of `yaw`, `pitch` and `roll`.
    pub rotation: Option<[f32; 4]>,
    /// Drawn at full brightness whatever the light at `light_at`.
    pub full_bright: bool,
    /// Size factor of the whole model about `position` (1 for none).
    pub scale: f32,
    /// Parts to turn (their x, y and z rotations in radians), by the name the part tree gives them: a chest's
    /// lid. API 48.
    pub parts: Vec<(String, [f32; 3])>,
}

/// The hook points' API version. The launcher reads this line from the
/// source it builds; keep its form.
#[allow(dead_code)]
pub(crate) const NATIVE_HOOKS_API: u32 = 57;

/// The largest mod packet a server sends a client (vanilla's clientbound
/// custom payload limit, 1 MiB). API 22.
pub(super) const NATIVE_PAYLOAD_TO_CLIENT: usize = 1 << 20;
/// The largest mod packet a client sends the server (vanilla's serverbound
/// custom payload limit). API 22.
pub(super) const NATIVE_PAYLOAD_TO_SERVER: usize = 32_767;

/// Whether `channel` is a mod's: `namespace:path` in resource characters,
/// not vanilla's or Horizon's own.
pub(super) fn mod_channel(channel: &str) -> bool {
    let Some((ns, path)) = channel.split_once(':') else {
        return false;
    };
    let ok = |s: &str, extra: &[u8]| {
        !s.is_empty()
            && s.bytes().all(|b| {
                b.is_ascii_lowercase()
                    || b.is_ascii_digit()
                    || b"_-.".contains(&b)
                    || extra.contains(&b)
            })
    };
    ok(ns, b"") && ok(path, b"/") && ns != "minecraft" && ns != "horizon" && channel.len() <= 256
}

impl GameLevel<'_> {
    /// An authoritative server committed a changed state through `Level.setBlock`.
    /// The world contains `new`; block side effects and neighbour updates follow.
    #[inline(always)]
    pub(super) fn native_block_changed(
        &self,
        dimension: LevelDimension,
        pos: BlockPos,
        old: BlockStateId,
        new: BlockStateId,
    ) {
        let _ = (dimension, pos, old, new);
    }

    /// An authoritative server is about to deliver `neighborChanged`.
    /// State and block IDs are valid in this level's registries for this call.
    #[inline(always)]
    pub(super) fn native_neighbor_changed(
        &self,
        dimension: LevelDimension,
        pos: BlockPos,
        state: BlockStateId,
        changed: BlockId,
        moved: bool,
    ) {
        let _ = (dimension, pos, state, changed, moved);
    }
}

/// The zoom a native mod asks for (`native_zoom`): the factor (1 is none),
/// eased through the field of view like the spyglass when `smooth`, and the
/// scale of mouse turning.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct NativeZoom {
    pub(crate) factor: f32,
    pub(crate) smooth: bool,
    pub(crate) turn: f32,
}

/// The minimap a native mod shows (`native_minimap`), drawn by Horizon's
/// minimap: its corner, its side in GUI pixels, blocks per map pixel,
/// turning with the player or north up, round or square.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct NativeMinimap {
    pub(crate) corner: Corner,
    pub(crate) size: f32,
    pub(crate) blocks_per_pixel: f32,
    pub(crate) rotate: bool,
    pub(crate) round: bool,
}

/// A mod block's screen over its menu (`native_screen`): its size, where
/// each of the container's slots is (`Slot.x`, `Slot.y`; none hides the
/// slot, and so do slots past the list; the server's menu does not have a
/// hidden slot either, so clicks and shift-clicks skip it), and the player inventory's first
/// row (vanilla's is 84; the hotbar is 58 below it) and left column, from
/// its top left.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NativeScreen {
    /// Vanilla's container screens are 176 wide.
    pub width: f32,
    pub height: f32,
    pub slots: Vec<Option<[f32; 2]>>,
    pub inventory_y: f32,
    /// The inventory's left column (vanilla's is 8).
    pub inventory_x: f32,
    /// The mod draws the title and the "Inventory" label itself
    /// (`native_screen_draw`); Horizon leaves them out.
    pub own_labels: bool,
    /// A second layout the screen swaps to while `native_screen_alt` says so (Mekanism's
    /// frequency selector is a screen of another size). API 36.
    pub alt: Option<Box<NativeScreen>>,
    /// Where the player's four armor slots sit, head first and 18 down each: Mekanism's tanks,
    /// energy cubes and modification station have them (`addArmorSlots`). They are the player's
    /// own, worn pieces, not the menu's. API 42.
    pub armor: Option<[f32; 2]>,
    /// The off-hand slot, with the armor slots (`OffhandSlot`). API 42.
    pub offhand: Option<[f32; 2]>,
}

/// The mod screen open on this client: the block it is for, the key its
/// slots are under, the window id of its menu, and the menu's data slots as
/// the server sent them.
pub(super) struct OpenNativeScreen {
    key: Pos,
    window: i32,
    block: String,
    screen: NativeScreen,
    data: Vec<i32>,
}

impl OpenNativeScreen {
    /// Data slot `id` of window `window`'s menu: false for another window's
    /// (an old or replaced one), which this screen leaves alone.
    fn set_data(&mut self, window: i32, id: i16, value: i16) -> bool {
        if window != self.window {
            return false;
        }
        // A menu's data slot ids are small; a bad one is dropped.
        if let Ok(i @ 0..256) = usize::try_from(id) {
            if self.data.len() <= i {
                self.data.resize(i + 1, 0);
            }
            self.data[i] = i32::from(value);
        }
        true
    }
}

/// The player as a native mod reads it.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativePlayer {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    /// Degrees; 0 faces south, 90 west (`Direction.fromYRot`).
    pub(crate) yaw: f32,
    pub(crate) pitch: f32,
    /// `minecraft:overworld`, `minecraft:the_nether`, `minecraft:the_end`.
    pub(crate) dimension: &'static str,
}

/// A mob or a dropped item as `native_entities_in` lists it. API 30.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeEntity {
    /// Its network id, for `native_hurt_mob` and `native_take_item_entity`.
    pub(crate) id: u32,
    /// The mob's type (`minecraft:zombie`), or `minecraft:item` for a dropped item.
    pub(crate) kind: String,
    /// Its feet.
    pub(crate) position: [f64; 3],
}

/// A native mod as Options > Mods lists it (`native_mods`).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NativeMod {
    /// Its id: its settings folder and its keys' lines.
    pub(crate) id: String,
    /// `Name 1.0`.
    pub(crate) title: String,
    pub(crate) description: String,
    /// Its keys: name, label and default key (`key.keyboard.z`). The page
    /// lists them first; `native_mod_key` reads the player's choice.
    pub(crate) keys: Vec<[String; 3]>,
}

/// An item the panel shows (`native_panel_items`): its id
/// (`minecraft:diamond`), a count (1 draws none) and its components beyond
/// the item's own, in vanilla's item syntax, or "".
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NativeStack {
    pub(crate) item: String,
    pub(crate) count: u32,
    pub(crate) components: String,
}

/// A creative tab a mod adds (`native_creative_tabs`).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NativeCreativeTab {
    /// `mymod:machines`; the tab's identity, kept when the player turns pages.
    pub(crate) id: String,
    /// A language key (`itemGroup.mymod.machines`) or the text itself.
    pub(crate) title: String,
    /// The item drawn on the tab (`mymod:digital_miner`).
    pub(crate) icon: String,
    /// The tab's stacks, in order; items the game does not have are left out.
    pub(crate) items: Vec<NativeStack>,
}

/// How the panel lays out a recipe of `native_panel_recipes`.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativeRecipeLayout {
    /// A crafting grid of `width` x `height` slots, an arrow and the result.
    Grid,
    /// One input over a flame, an arrow and the result (furnaces, smokers,
    /// campfires).
    Cooking,
    /// One input, an arrow and the result (the stonecutter).
    Single,
    /// Template, base and addition, an arrow and the result.
    Smithing,
    /// A potion and an ingredient, an arrow and the result.
    Brewing,
}

/// A recipe the panel shows (`native_panel_recipes`). Each slot, and the
/// result, cycles through its stacks, one a second; an empty slot shows
/// nothing.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeRecipeView {
    pub(crate) layout: NativeRecipeLayout,
    /// The grid's size (`Grid` only), at most 3 x 3.
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) slots: Vec<Vec<NativeStack>>,
    pub(crate) results: Vec<NativeStack>,
    /// A short line under the recipe (a cooking time), or "".
    pub(crate) note: String,
    /// The action button beside it: none, enabled, or shown greyed.
    pub(crate) action: Option<bool>,
}

/// What the player did on a native mod's panel (`native_panel_event`).
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum NativePanelEvent {
    /// A click on an item of the panel, by its index in
    /// `native_panel_items`' list.
    Clicked { index: u32, right: bool },
    /// The search box's text, each time the player changes it.
    Search(String),
    /// A key (`key.keyboard.r`; compare it with `native_mod_key`) went down
    /// while the pointer was on an item: a slot of the screen's, or the
    /// panel's.
    Key { key: String, stack: NativeStack },
    /// A click on a slot of a recipe the panel shows: the recipe's index in
    /// `native_panel_recipes`' list and the slot's in it (the results come
    /// after the slots), and the stack it showed then.
    SlotClicked {
        recipe: u32,
        slot: u32,
        stack: NativeStack,
        right: bool,
    },
    /// The action button of the recipe at this index.
    Action(u32),
    /// The player turned the recipes' page: the index of the first recipe
    /// now shown.
    Page(u32),
    /// The recipe page's back button.
    Back,
    /// The player picked the tab at this index.
    Tab(u32),
}

/// An item of [`NativeRecipes::items`], by its index there, with a count
/// and the components it has beyond the item's own, in vanilla's item
/// syntax (`[minecraft:potion_contents={potion:"minecraft:swiftness"}]`),
/// or "".
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NativeRecipeStack {
    pub(crate) item: u32,
    pub(crate) count: u32,
    pub(crate) components: String,
}

/// The station a recipe is made at.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativeRecipeKind {
    CraftingShaped,
    CraftingShapeless,
    Smelting,
    Blasting,
    Smoking,
    Campfire,
    Stonecutting,
    Smithing,
    Brewing,
}

/// A recipe of [`NativeRecipes`].
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeRecipe {
    /// `minecraft:oak_planks`.
    pub(crate) id: String,
    pub(crate) kind: NativeRecipeKind,
    /// The recipe book's tab (`crafting_building_blocks`, `furnace_food`),
    /// or "".
    pub(crate) category: String,
    /// Recipes the book shows on one button share a group, or "".
    pub(crate) group: String,
    /// A shaped recipe's grid: `slots` holds its cells row by row. 0 for
    /// the other kinds.
    pub(crate) width: u32,
    pub(crate) height: u32,
    /// Each slot's stacks, any one of which fits; an empty list is an empty
    /// cell. Cooking and stonecutting: the input. Smithing: the template,
    /// the base, the addition. Brewing: the bottle, the reagent.
    pub(crate) slots: Vec<Vec<NativeRecipeStack>>,
    /// What it makes; several when that varies (a trim on any armour).
    pub(crate) results: Vec<NativeRecipeStack>,
    /// Cooking: game ticks and experience; 0 for the other kinds.
    pub(crate) cooking_time: u32,
    pub(crate) experience: f32,
    /// The recipe book's id for it (one per way the book shows it), as
    /// `native_known_recipes` lists them; -1 for the kinds the book doesn't
    /// show (campfire, stonecutting, smithing, brewing).
    pub(crate) book_id: i32,
}

/// An item of [`NativeRecipes::items`].
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NativeItem {
    /// `minecraft:diamond`, `<mod>:<path>`.
    pub(crate) id: String,
    /// In the player's language.
    pub(crate) name: String,
    /// Its item tags, without the `#`.
    pub(crate) tags: Vec<String>,
}

/// Every item and recipe the game knows (`native_recipes`): `items` in the
/// registry's order, which each [`NativeRecipeStack::item`] indexes; the
/// recipes are the book's in its order, then campfire, stonecutting,
/// smithing and brewing.
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NativeRecipes {
    pub(crate) items: Vec<NativeItem>,
    pub(crate) recipes: Vec<NativeRecipe>,
}

/// The mappings `native_mapping` answers for, with the key their default
/// binding is (none: a mouse button, attack or use).
#[allow(dead_code)]
const MAPPINGS: [(&str, Option<KeyCode>); 9] = [
    ("key.forward", Some(KeyCode::KeyW)),
    ("key.left", Some(KeyCode::KeyA)),
    ("key.back", Some(KeyCode::KeyS)),
    ("key.right", Some(KeyCode::KeyD)),
    ("key.jump", Some(KeyCode::Space)),
    ("key.sneak", Some(KeyCode::ShiftLeft)),
    ("key.sprint", Some(KeyCode::ControlLeft)),
    ("key.attack", None),
    ("key.use", None),
];

/// The hook points. Each does nothing in Horizon.
impl Game<'_> {
    /// Once a frame, in a world or not, after Horizon's extras.
    pub(super) fn native_frame(&mut self, elapsed: Duration) {
        let _ = elapsed;
    }
    /// Once a tick, in every `Game`: the window's (in single player, on LAN
    /// and on servers), and the server's own (`native_window` is false
    /// there: the single player server thread, a dedicated server).
    pub(super) fn native_tick(&mut self) {}
    /// The world's phases, read-only; `error_text` carries the engine's
    /// own open-failure text, empty otherwise. Server-side; the deprecated
    /// in-frame path delivers no lifecycle events. A server loop or final
    /// save failure fires `WorldCloseFailed` with the error text.
    pub(super) fn native_world_lifecycle(
        &mut self,
        context: &NativeLifecycleContext,
        phase: WorldPhase,
        error_text: &str,
    ) {
        let _ = (context, phase, error_text);
    }
    /// The authoritative server tick's phases: `ServerTickStart` before any
    /// level is ticked (NeoForge's `ServerTickEvent.Pre`), `ServerTickEnd`
    /// after the last (`.Post`). Read blocks with `native_server_block`, by
    /// dimension: which level `self.world` holds here is not defined. A paused
    /// pass fires nothing; a frozen world carries `world_running` false except
    /// during a requested step.
    pub(super) fn native_server_tick(
        &mut self,
        context: &NativeLifecycleContext,
        phase: ServerTickPhase,
    ) {
        let _ = (context, phase);
    }
    /// The HUD (not drawn under F1), after the minimap: place a box with
    /// `corners.place` (boxes stack away from their edge) and draw it with
    /// `native_fill` and `native_text`. Skip a corner `native_corner_shown`
    /// says is taken.
    pub(super) fn native_hud(&mut self, batch: &mut HudBatch, corners: &mut Corners) {
        let _ = (batch, corners);
    }
    /// A key or mouse button went down or up, by its vanilla name
    /// (`key.keyboard.z`, `key.mouse.left`): keys on any screen (check
    /// `native_playing`), mouse buttons while playing. Repeats are left out.
    pub(super) fn native_key(&mut self, key: &str, pressed: bool) {
        let _ = (key, pressed);
    }
    /// Wheel steps while playing: set `claimed` to keep them from the hotbar.
    pub(super) fn native_wheel(&mut self, steps: f32, claimed: &mut bool) {
        let _ = (steps, claimed);
    }
    /// A command typed or clicked in chat, without its slash, before the
    /// server and Horizon's commands see it: set `ran` when the mod ran it.
    pub(super) fn native_command(&mut self, command: &str, ran: &mut bool) {
        let _ = (command, ran);
    }
    /// The zoom a native mod asks for, applied while playing (the spyglass's
    /// own view wins): set `zoom` when it is none yet.
    pub(super) fn native_zoom(&self, zoom: &mut Option<NativeZoom>) {
        let _ = zoom;
    }
    /// The minimap a native mod shows: set `minimap` when it is none yet.
    pub(super) fn native_minimap(&self, minimap: &mut Option<NativeMinimap>) {
        let _ = minimap;
    }
    /// Options > Mods lists the native mods: push yours.
    pub(super) fn native_mods(&mut self, mods: &mut Vec<NativeMod>) {
        let _ = mods;
    }
    /// The buttons of mod `mod_id`'s settings page, under its keys: push
    /// their labels when it is yours. Asked again each time the page opens
    /// and after each click.
    pub(super) fn native_settings(&mut self, mod_id: &str, buttons: &mut Vec<String>) {
        let _ = (mod_id, buttons);
    }
    /// Button `button` of mod `mod_id`'s settings page was clicked.
    pub(super) fn native_settings_click(&mut self, mod_id: &str, button: usize) {
        let _ = (mod_id, button);
    }
    /// The player did `event` on mod `mod_id`'s panel (`native_panel_items`):
    /// act on it when it is yours. API 2.
    pub(super) fn native_panel_event(&mut self, mod_id: &str, event: &NativePanelEvent) {
        let _ = (mod_id, event);
    }
    /// Block `block` (`mymod:machine`) gets a block entity
    /// (`EntityBlock.newBlockEntity`): set `entity` to its id and saved
    /// compound when the block is yours and `entity` is none yet. The block's
    /// template must have a block entity (a chest, a furnace...), so that
    /// breaking the block removes it. API 15.
    pub(super) fn native_block_entity(
        &mut self,
        block: &str,
        entity: &mut Option<(String, Compound)>,
    ) {
        let _ = (block, entity);
    }
    /// Whether block `block`'s block entity ticks (`EntityBlock.getTicker`):
    /// set `ticks` when it is yours. Asked for each of the block's states
    /// when a world opens. API 15.
    pub(super) fn native_block_entity_ticks(&self, block: &str, ticks: &mut bool) {
        let _ = (block, ticks);
    }
    /// Block `block`'s block entity has item slots (a `WorldlyContainer`):
    /// set `container` when the block is yours. Asked once for each mod
    /// block when the ticking table is built; at most 256 slots, and slot
    /// numbers past the count are left out. Its `menu`, one of
    /// `containers::MOD_MENUS` with as many slots, opens when the block is
    /// used (its template must open a menu, as a chest or a furnace does);
    /// another is left out. API 17.
    pub(super) fn native_block_entity_container(
        &self,
        block: &str,
        container: &mut Option<ModContainer>,
    ) {
        let _ = (block, container);
    }
    /// A server tick of the block entity at `pos`, in vanilla's order among
    /// the others (`Level.tickBlockEntities`), when its `id` is yours: read
    /// and change its saved compound `nbt`, its item slots `items` (keep
    /// their count: as many as its container has, none without one) and the
    /// data slots its menu shows (`data`). It is out of the world during the
    /// call (the block entity at `pos` reads as a `Mod` with an empty id, and
    /// a save leaves it out); if the call removes or replaces it, `entity` is
    /// dropped. Clients, the window's own in single player included, don't
    /// get it yet. API 17.
    pub(super) fn native_block_entity_tick(&mut self, pos: [i32; 3], entity: &mut ModBlockEntity) {
        let _ = (pos, entity);
    }
    /// A menu opens on this client for block `block`, a mod's: set `screen`
    /// when the block is yours, to lay its screen out (`native_screen_draw`
    /// draws it). The server asks it too: a slot it hides is not in the menu
    /// there either, so shift-clicks never fill it. API 19.
    pub(super) fn native_screen(&self, block: &str, screen: &mut Option<NativeScreen>) {
        let _ = (block, screen);
    }
    /// What the slots of block `block`'s menu take, on the server: push one
    /// rule per container slot (the missing ones take anything). API 38.
    pub(super) fn native_slot_rules(
        &self,
        block: &str,
        rules: &mut Vec<horizon_client::lan::menu::SlotRule>,
    ) {
        let _ = (block, rules);
    }
    /// The rules of the mod menu at `pos`, from the mod's screen.
    pub(super) fn mod_menu_rules(&self, pos: Pos) -> Vec<horizon_client::lan::menu::SlotRule> {
        if self.mod_menu_at(pos).is_none() {
            return Vec::new();
        }
        let block = self
            .res
            .facts()
            .get(self.world.block(pos.0, pos.1, pos.2))
            .name
            .to_string();
        let mut rules = Vec::new();
        self.native_slot_rules(&block, &mut rules);
        rules
    }
    /// Escape was pressed on block `block`'s screen: set `consumed` when the mod
    /// used it (closed a window of its own), and the screen stays open. API 37.
    pub(super) fn native_screen_escape(&mut self, block: &str, consumed: &mut bool) {
        let _ = (block, consumed);
    }
    /// Escape on the open mod screen: whether the mod took it.
    pub(super) fn escape_taken_by_mod(&mut self) -> bool {
        let Some(block) = self.shown_native_screen().map(|s| s.block.clone()) else {
            return false;
        };
        let mut consumed = false;
        self.native_screen_escape(&block, &mut consumed);
        consumed
    }
    /// Boxes a mod wants drawn in the world this frame: push a [`NativeWorldBox`] for each. They are
    /// drawn like translucent item quads (full bright, no depth writes, seen from both sides). API 41.
    pub(super) fn native_world_boxes(&self, boxes: &mut Vec<NativeWorldBox>) {
        let _ = boxes;
    }
    /// Models a mod wants drawn in the world this frame: push a [`NativeWorldModel`] for each (a wind
    /// generator's spinning rotor). API 43.
    pub(super) fn native_world_models(&self, models: &mut Vec<NativeWorldModel>) {
        let _ = models;
    }
    /// Items a mod wants drawn in the world this frame: push a [`NativeWorldItem`] for each. API 47.
    pub(super) fn native_world_items(&self, items: &mut Vec<NativeWorldItem>) {
        let _ = items;
    }
    /// Text a mod wants drawn in the world this frame: push a [`NativeWorldText`] for each. API 47.
    pub(super) fn native_world_texts(&self, texts: &mut Vec<NativeWorldText>) {
        let _ = texts;
    }
    /// Whether the game should leave mob `id` of type `kind` (the base type a mod's mob is sent as) undrawn because
    /// the mod draws it itself, from [`Self::native_mob_pose`] and `native_world_models`: set `hidden`. API 48.
    pub(super) fn native_mob_hidden(&self, id: u32, kind: &str, hidden: &mut bool) {
        let _ = (id, kind, hidden);
    }
    /// Quads a mod wants drawn in the world this frame: push a [`NativeWorldQuad`] for each. API 47.
    pub(super) fn native_world_quads(&self, quads: &mut Vec<NativeWorldQuad>) {
        let _ = quads;
    }
    /// The vertices of the mods' world quads, camera relative, full bright, appended to the entity quads.
    pub(super) fn world_quad_geometry(
        &self,
        camera: glam::DVec3,
        out: &mut Vec<horizon_client::mesher::Vertex>,
    ) {
        let mut quads = Vec::new();
        self.native_world_quads(&mut quads);
        if quads.is_empty() {
            return;
        }
        let white = self.gui.atlas.font.white();
        let white_uv = horizon_client::gfx::lightning::white_uv(&white);
        for q in &quads {
            let [a, r, g, bl] = q.color.to_be_bytes();
            let color = u32::from(a) << 24 | u32::from(bl) << 16 | u32::from(g) << 8 | u32::from(r);
            let rect = q
                .sprite
                .as_deref()
                .and_then(|name| self.gui.atlas.sprites.get(name))
                .map(|r| [r.u0, r.v0, r.u1, r.v1]);
            let corner_uv = |i: usize| match rect {
                Some(r) => {
                    let [u0, v0, u1, v1] = q.uv;
                    let (tu, tv) = [(u0, v1), (u1, v1), (u1, v0), (u0, v0)][i];
                    [r[0] + (r[2] - r[0]) * tu, r[1] + (r[3] - r[1]) * tv]
                }
                None => white_uv,
            };
            for order in [[0, 1, 2, 3], [3, 2, 1, 0]] {
                for i in order {
                    out.push(horizon_client::mesher::Vertex {
                        pos: std::array::from_fn(|k| (q.corners[i][k] - camera[k]) as f32),
                        uv: corner_uv(i),
                        layer: horizon_client::gfx::TRANSLUCENT_ITEM_OVERLAY,
                        color,
                        light: 240 | 240 << 8,
                        normal: [0; 4],
                        glint_uv: [0.0, 0.0],
                    });
                }
            }
        }
    }
    /// The vertices of the mods' world items and texts, camera relative, appended to the entity quads.
    pub(super) fn world_item_geometry(
        &self,
        camera: glam::DVec3,
        out: &mut Vec<horizon_client::mesher::Vertex>,
    ) {
        let to_camera = |m: &[f64; 16]| {
            (glam::DMat4::from_translation(-camera) * glam::DMat4::from_cols_array(m)).as_mat4()
        };
        let mut items = Vec::new();
        self.native_world_items(&mut items);
        for i in &items {
            let rel = to_camera(&i.matrix);
            if rel.w_axis.truncate().length_squared() > 64.0 * 64.0 {
                continue;
            }
            let stack = ItemStack::new(&i.item, 1);
            if self.res.items().get(&i.item).is_none() {
                continue;
            }
            let start = out.len();
            let state = self.resolve_item(
                &stack,
                if i.ground {
                    horizon_assets::item_model::DisplayContext::Ground
                } else {
                    horizon_assets::item_model::DisplayContext::Gui
                },
                None,
                i.seed,
            );
            let light = self
                .world
                .light(i.light_at[0], i.light_at[1], i.light_at[2]);
            self.submit_item_world(
                &state,
                rel,
                horizon_client::block_entity_render::vertex_light(light),
                out,
            );
            self.cull_entity_vertices(out, start);
        }
        let mut texts = Vec::new();
        self.native_world_texts(&mut texts);
        for t in &texts {
            let rel = to_camera(&t.matrix);
            if rel.w_axis.truncate().length_squared() > 64.0 * 64.0 {
                continue;
            }
            let light = self
                .world
                .light(t.light_at[0], t.light_at[1], t.light_at[2]);
            let call = horizon_client::block_entity_render::TextCall {
                matrix: rel,
                x: t.x,
                y: t.y,
                text: t.text.clone(),
                color: t.color,
                outline: 0,
                light: horizon_client::block_entity_render::vertex_light(light),
            };
            horizon_client::block_entity_render::draw_text(
                &|c| self.gui.atlas.glyph_cell(c),
                &call,
                out,
            );
        }
    }
    /// The vertices of the mods' world models, camera relative, appended to the entity quads.
    pub(super) fn world_model_geometry(
        &self,
        camera: glam::DVec3,
        out: &mut Vec<horizon_client::mesher::Vertex>,
    ) {
        let mut models = Vec::new();
        self.native_world_models(&mut models);
        for m in &models {
            let Some((part, sprite)) = self.armor_stand_assets.world_models.get(&m.model) else {
                continue;
            };
            let Some(r) = self.gui.atlas.sprites.get(sprite.as_str()) else {
                continue;
            };
            let rel = glam::DVec3::from_array(m.position) - camera;
            if rel.length_squared() > 128.0 * 128.0 {
                continue;
            }
            let light = self
                .world
                .light(m.light_at[0], m.light_at[1], m.light_at[2]);
            let packed = if m.full_bright {
                240 | 240 << 8
            } else {
                (u32::from(light >> 4) * 16) | ((u32::from(light & 15) * 16) << 8)
            };
            let [ca, cr, cg, cb] = m.color.to_be_bytes();
            let color =
                u32::from(ca) << 24 | u32::from(cb) << 16 | u32::from(cg) << 8 | u32::from(cr);
            let turn = match m.rotation {
                Some([x, y, z, w]) => glam::Quat::from_xyzw(x, y, z, w).normalize(),
                None => glam::Quat::from_euler(
                    glam::EulerRot::YXZ,
                    m.yaw.to_radians(),
                    m.pitch.to_radians(),
                    m.roll.to_radians(),
                ),
            };
            let base = glam::Mat4::from_translation(rel.as_vec3())
                * glam::Mat4::from_quat(turn)
                * glam::Mat4::from_scale(glam::Vec3::splat(m.scale));
            let mut poses = horizon_client::mobs::anim::Poses::new(part);
            for (name, [x, y, z]) in &m.parts {
                poses.xrot(name, *x);
                poses.yrot(name, *y);
                poses.zrot(name, *z);
            }
            horizon_client::mobs::render::draw(
                part,
                base,
                &poses,
                [r.u0, r.v0, r.u1, r.v1],
                color,
                packed,
                0,
                out,
            );
        }
    }
    /// The vertices of the mods' world boxes, camera relative, appended to the entity quads.
    pub(super) fn world_box_geometry(
        &self,
        camera: glam::DVec3,
        out: &mut Vec<horizon_client::mesher::Vertex>,
    ) {
        let mut boxes = Vec::new();
        self.native_world_boxes(&mut boxes);
        if boxes.is_empty() {
            return;
        }
        let white = self.gui.atlas.font.white();
        let uv = horizon_client::gfx::lightning::white_uv(&white);
        for b in &boxes {
            if let Some(rect) = b
                .sprite
                .as_deref()
                .and_then(|name| self.gui.atlas.sprites.get(name))
            {
                sprite_box(b, [rect.u0, rect.v0, rect.u1, rect.v1], camera, out);
                continue;
            }
            let lo: [f32; 3] = std::array::from_fn(|i| (b.min[i] - camera[i]) as f32);
            let hi: [f32; 3] = std::array::from_fn(|i| (b.max[i] - camera[i]) as f32);
            let at = |x: bool, y: bool, z: bool| {
                [
                    if x { hi[0] } else { lo[0] },
                    if y { hi[1] } else { lo[1] },
                    if z { hi[2] } else { lo[2] },
                ]
            };
            // East, west, up, down, south, north: each face's four corners.
            let faces = [
                [
                    at(true, false, false),
                    at(true, true, false),
                    at(true, true, true),
                    at(true, false, true),
                ],
                [
                    at(false, false, false),
                    at(false, false, true),
                    at(false, true, true),
                    at(false, true, false),
                ],
                [
                    at(false, true, false),
                    at(false, true, true),
                    at(true, true, true),
                    at(true, true, false),
                ],
                [
                    at(false, false, false),
                    at(true, false, false),
                    at(true, false, true),
                    at(false, false, true),
                ],
                [
                    at(false, false, true),
                    at(true, false, true),
                    at(true, true, true),
                    at(false, true, true),
                ],
                [
                    at(false, false, false),
                    at(false, true, false),
                    at(true, true, false),
                    at(true, false, false),
                ],
            ];
            for (corners, argb) in faces.into_iter().zip(b.colors) {
                let [a, r, g, bl] = argb.to_be_bytes();
                let color =
                    u32::from(a) << 24 | u32::from(bl) << 16 | u32::from(g) << 8 | u32::from(r);
                for p in corners {
                    out.push(horizon_client::mesher::Vertex {
                        pos: p,
                        uv,
                        layer: horizon_client::gfx::TRANSLUCENT_ITEM_OVERLAY,
                        color,
                        // Full bright: sky and block light 15 (16x fixed point).
                        light: 240 | 240 << 8,
                        normal: [0; 4],
                        glint_uv: [0.0, 0.0],
                    });
                }
            }
        }
    }
    /// The wheel turned `steps` (positive up) with the cursor at `at` (as in
    /// `native_screen_click`) on block `block`'s screen: set `taken` when the
    /// mod scrolled something of its own. API 40.
    pub(super) fn native_screen_wheel(
        &mut self,
        block: &str,
        at: [f32; 2],
        steps: f32,
        taken: &mut bool,
    ) {
        let _ = (block, at, steps, taken);
    }
    /// The wheel on the open mod screen: whether the mod took it.
    pub(super) fn wheel_taken_by_mod(&mut self, steps: f32) -> bool {
        let Some((block, _, at)) = self.native_screen_at() else {
            return false;
        };
        let mut taken = false;
        self.native_screen_wheel(&block, at, steps, &mut taken);
        taken
    }
    /// The mob types are loaded: a mod adds its own to `types` (a clone of a vanilla type with another
    /// `id`, size, health or speed) and declares it with `horizon_client::mobs::register_mod_mob_base(id, base)`;
    /// networking sends it as `base`. `/summon` and the saved entities find it by `id`. API 46.
    pub(super) fn native_mob_types(
        types: &mut std::collections::HashMap<String, horizon_client::mobs::MobType>,
    ) {
        let _ = types;
    }
    /// Every tick on a level (client and server alike), after its mobs ticked: a mod steers the mobs of
    /// its own types (`native_mob_ids`, `native_with_mob`). API 46.
    pub(super) fn native_mobs_tick(&mut self) {}
    /// A mob of `kind` died at `at` (before its loot drops), carrying the mod's `data` if any. API 46.
    pub(super) fn native_mob_died(
        &mut self,
        kind: &str,
        at: [f64; 3],
        data: Option<&horizon_world::nbt::Compound>,
    ) {
        let _ = (kind, at, data);
    }
    /// The player right clicked mob `id` on a window that is a client (a server's player, or the single
    /// player's window): set `claimed` to take the click. The mob is the base type to this client;
    /// the mod knows its own by id. API 46.
    pub(super) fn native_use_mob(&mut self, id: u32, claimed: &mut bool) {
        let _ = (id, claimed);
    }
    /// The mod's own screen `id` (opened by `native_open_ui`) is drawn: fill the
    /// whole window (`batch.width` by `batch.height`, GUI pixels) with `native_fill`,
    /// `native_sprite`, `native_text`. The cursor is `native_cursor`. API 44.
    pub(super) fn native_ui_draw(&self, id: &str, batch: &mut HudBatch) {
        let _ = (id, batch);
    }
    /// A mouse button (0 left, 1 right, 2 middle or other) went down or up on
    /// the mod's screen `id`, with the cursor at `at`. API 44.
    pub(super) fn native_ui_click(&mut self, id: &str, at: [f32; 2], button: u8, pressed: bool) {
        let _ = (id, at, button, pressed);
    }
    /// A key (`key.keyboard.*`) went down or up on the mod's screen `id`, with
    /// the text it types (when it types any; a paste gives the clipboard).
    /// Escape closes the screen and is not sent. API 44.
    pub(super) fn native_ui_key(&mut self, id: &str, key: &str, text: Option<&str>, pressed: bool) {
        let _ = (id, key, text, pressed);
    }
    /// The wheel turned `steps` (positive up) on the mod's screen `id`. API 44.
    pub(super) fn native_ui_wheel(&mut self, id: &str, steps: f32) {
        let _ = (id, steps);
    }
    /// The mod's screen `id` was closed (Escape, or `native_close_ui`). API 44.
    pub(super) fn native_ui_closed(&mut self, id: &str) {
        let _ = id;
    }
    /// Whether block `block`'s screen shows its `NativeScreen::alt` layout now: set `alt`.
    /// Asked every frame, so keep it a flag read. API 36.
    pub(super) fn native_screen_alt(&self, block: &str, alt: &mut bool) {
        let _ = (block, alt);
    }
    /// Draw block `block`'s screen at `origin` (its top left, GUI pixels),
    /// over a plain panel with its slots' frames and under the items and
    /// labels, from the menu's data slots `data` (the block entity's `data`
    /// on the server): data slots are 16-bit on the wire as in vanilla (a
    /// bigger value wraps), and `data` is empty until the server's first
    /// values arrive, so read it with `get`. Draw with `native_sprite`, `native_fill` and
    /// `native_text`. API 19.
    pub(super) fn native_screen_draw(
        &self,
        block: &str,
        data: &[i32],
        batch: &mut HudBatch,
        origin: [f32; 2],
    ) {
        let _ = (block, data, batch, origin);
    }
    /// A left or right (`right`) press at `at` (GUI pixels from the screen's
    /// top left, so outside it too) on block `block`'s screen, a mod's, over
    /// no slot: set `button` to a menu button id when it is on one of yours.
    /// Horizon sends it to the server (`ContainerButtonClick`,
    /// `native_menu_button` there) and the press goes no further. API 20.
    pub(super) fn native_screen_click(
        &self,
        block: &str,
        data: &[i32],
        at: [f32; 2],
        right: bool,
        button: &mut Option<i32>,
    ) {
        let _ = (block, data, at, right, button);
    }
    /// The tooltip at `at` (as in `native_screen_click`) on block `block`'s
    /// screen, a mod's, over no item: push its lines when it is over
    /// something of yours. Legacy colour codes (§) work. API 20.
    pub(super) fn native_screen_tooltip(
        &self,
        block: &str,
        data: &[i32],
        at: [f32; 2],
        lines: &mut Vec<String>,
    ) {
        let _ = (block, data, at, lines);
    }
    /// Draw over block `block`'s screen, a mod's, after its items, labels
    /// and the player's inventory: a window of the mod's own that covers
    /// slots (`native_screen_draw` is under them). Same `data` and `origin`
    /// as there. API 32.
    pub(super) fn native_screen_overlay(
        &self,
        block: &str,
        data: &[i32],
        batch: &mut HudBatch,
        origin: [f32; 2],
    ) {
        let _ = (block, data, batch, origin);
    }
    /// Whether the mod's own drawing covers `at` (as in
    /// `native_screen_click`) on block `block`'s screen: set `covered` and
    /// the slots there neither take presses nor light up or show their
    /// tooltips; the press goes to `native_screen_click` and stops there. API 32.
    pub(super) fn native_screen_covers(
        &self,
        block: &str,
        data: &[i32],
        at: [f32; 2],
        covered: &mut bool,
    ) {
        let _ = (block, data, at, covered);
    }
    /// Whether a text box of the mod's own has focus on block `block`'s
    /// screen, a mod's: set `typing` and every key press but Escape goes to
    /// `native_screen_key` instead of the game (the inventory key, hotbar
    /// keys and drop no longer act), as vanilla's anvil rename box takes
    /// them. Same `data` as in `native_screen_draw`. API 33.
    pub(super) fn native_screen_typing(&self, block: &str, data: &[i32], typing: &mut bool) {
        let _ = (block, data, typing);
    }
    /// A key press (or repeat) while `native_screen_typing` holds on block
    /// `block`'s screen: `key` is its name as in `native_key`
    /// (`key.keyboard.backspace`...), `text` what it types, if anything
    /// (control characters left out), with `ctrl` and `shift` held. The
    /// edit shortcut with V comes as `key.keyboard.v` with the clipboard's
    /// text in `text`; with A, C or X the mod answers it itself
    /// (`native_set_clipboard` for copy and cut). Send what the box holds
    /// with `native_send_to_server`. API 33.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn native_screen_key(
        &mut self,
        block: &str,
        data: &[i32],
        key: &str,
        text: Option<&str>,
        ctrl: bool,
        shift: bool,
    ) {
        let _ = (block, data, key, text, ctrl, shift);
    }
    /// A player pressed menu button `id` on the menu of the mod block entity
    /// at `pos` (`AbstractContainerMenu.clickMenuButton`), on the server:
    /// change `entity` when its id is yours. It is out of the world during
    /// the call, as in `native_block_entity_tick`. API 20.
    pub(super) fn native_menu_button(
        &mut self,
        pos: [i32; 3],
        entity: &mut ModBlockEntity,
        id: i32,
    ) {
        let _ = (pos, entity, id);
    }
    /// The commands native mods add to the server's command tree: push
    /// yours. Asked once when a world opens to players (a dedicated
    /// server, a LAN host, single player's server thread); every player's
    /// chat box then completes them as vanilla's, within their level. A
    /// name vanilla has is left out. API 8.
    pub(super) fn native_register_commands(&mut self, commands: &mut Vec<NativeCommand>) {
        let _ = commands;
    }
    /// A player's command on the server (`Commands.performCommand`),
    /// without its slash, for a command a mod registered and the player's
    /// level allows: set `ran` when it is yours. The player's state is
    /// swapped in for the call, so `native_player` and the other helpers
    /// read that player; `native_feedback` answers them. API 8.
    pub(super) fn native_perform_command(
        &mut self,
        player: &NativeServerPlayer,
        command: &str,
        ran: &mut bool,
    ) {
        let _ = (player, command, ran);
    }
    /// A click in a menu a mod opened (`native_open_menu`), which moved
    /// nothing: the menu's `slot` (its own first, then the player's
    /// inventory, `native_menu_inventory_slot`; -999 outside), the `button`
    /// and the click's `mode` (`ClickType`'s id) of the player `uuid`.
    /// No player's state is swapped in: the menu helpers take the player's
    /// UUID. API 10.
    pub(super) fn native_menu_clicked(
        &mut self,
        uuid: u128,
        window: i32,
        slot: i32,
        button: i32,
        mode: i32,
    ) {
        let _ = (uuid, window, slot, button, mode);
    }
    /// A menu a mod opened closed: the player closed it, the server or a mod
    /// opened another, or the player left. API 10.
    pub(super) fn native_menu_closed(&mut self, uuid: u128, window: i32) {
        let _ = (uuid, window);
    }
    /// The creative tabs mods add, asked each time the creative inventory
    /// opens: push a tab per module (Mekanism, Generators...). They follow
    /// vanilla's, ten to a page, with `<` and `>` buttons to turn pages as
    /// NeoForge has them; their stacks join the search tab. API 34.
    pub(super) fn native_creative_tabs(&self, tabs: &mut Vec<NativeCreativeTab>) {
        let _ = tabs;
    }
    /// An item's tooltip, wherever the game shows one (slots, the creative
    /// tabs, the recipe book, chat's hover): the stack, with its components
    /// (`stack.data()`: a shulker box's contents...), and its lines, the name
    /// first, each with a colour as `rgba` packs it (`u32::MAX` keeps the
    /// text's own). Add, change or take out lines; `Text::legacy` reads `§`
    /// codes. Asked each frame the tooltip shows, never for one the stack
    /// hides. API 12.
    pub(super) fn native_tooltip(&self, stack: &ItemStack, lines: &mut Vec<(Text, u32)>) {
        let _ = (stack, lines);
    }
    /// The speed `stack` (the acting player's main hand, `None` when empty)
    /// mines the block `block` of hardness `hardness` at, before enchantments
    /// and effects: `Item.getDestroySpeed`, as `speed` holds it from the
    /// item's tool rules. A mod's tool sets its own (a powered drill that is
    /// fast while charged). Both sides: the client predicts the crack, the
    /// server times the break. API 25.
    pub(super) fn native_destroy_speed(
        &self,
        stack: Option<&ItemStack>,
        block: &str,
        hardness: f32,
        speed: &mut f32,
    ) {
        let _ = (stack, block, hardness, speed);
    }
    /// A player's melee hit on the mob of type `target` (`minecraft:zombie`):
    /// `amount` is its damage as `Player.attack` worked it out from the
    /// attack damage attribute, the attack strength `strength` (0 to 1, the
    /// cooldown) and the enchantments, before the mob takes it. A mod's weapon
    /// adds to it or takes from it (a powered sword that spends energy on
    /// each hit). The acting player's state is swapped in. Server-side, for
    /// every player of the world. API 27.
    pub(super) fn native_player_attack(&mut self, target: &str, strength: f32, amount: &mut f32) {
        let _ = (target, strength, amount);
    }
    /// A player's damage taken by a mod's armor: `absorbed` is the damage
    /// that `share` (0 to 1, of each hit not tagged
    /// `minecraft:bypasses_armor`) kept off them since the last call, to pay
    /// for; set `share` for the hits to come (a suit with no energy left sets
    /// it to 0). Called at once after an absorbed hit and every 10 ticks
    /// otherwise, with the player's state swapped in. Server-side, for every
    /// player of the world. API 28.
    pub(super) fn native_player_damage(&mut self, absorbed: f32, share: &mut f32) {
        let _ = (absorbed, share);
    }
    /// The damage a player takes none of, asked every tick with the player's state swapped in: push damage type
    /// ids (`minecraft:magic`) or tags with a leading `#` (`#minecraft:is_fire`) (a gas mask against magic damage).
    /// Server-side, for every player of the world. API 52.
    pub(super) fn native_player_immunities(&mut self, immune: &mut Vec<String>) {
        let _ = immune;
    }
    /// A share of the damage of some types that a mod's armor takes, asked every tick with the player's state
    /// swapped in: `absorbed` is the damage the rules took since the last call, for the mod to pay for; push
    /// `(type or #tag, share)` for the hits to come (a share of 1 takes the whole hit, and it is no hit at all). Unlike
    /// `native_player_damage` it applies to sources that bypass armor too. Server-side, for every player of the
    /// world. API 53.
    pub(super) fn native_player_damage_rules(&mut self, absorbed: f32, rules: &mut Vec<(String, f32)>) {
        let _ = (absorbed, rules);
    }
    /// Attribute modifiers the player's worn gear gives beyond what its items list, asked every tick with the player
    /// swapped in: push `(attribute id, modifier id, amount, operation)` (`minecraft:water_movement_efficiency`,
    /// `add_value`). Applied like an enchantment's modifiers. Server-side, for every player of the world. API 54.
    pub(super) fn native_gear_attributes(&mut self, attributes: &mut Vec<(String, String, f64, String)>) {
        let _ = attributes;
    }
    /// The frame's fog (`FogRenderer.setupFog`, and Neo's `ViewportEvent.RenderFog` and `ComputeFogColor`) after the game
    /// set it up, for a mod to change: `fog_type` is 0 for atmospheric fog, 1 water, 2 lava, 3 powder snow. Client-side,
    /// every frame. API 56.
    pub(super) fn native_fog(&self, fog_type: u8, fog: &mut horizon_client::fog::FogData) {
        let _ = (fog_type, fog);
    }
    /// The colour (`0xAARRGGBB`) a worn piece's opaque mod models (not the glowing or translucent ones) are drawn in,
    /// asked for each worn piece when its layers are collected: set `tint` for the stack, leave it `None` for white.
    /// Client-side, every frame. API 57.
    pub(super) fn native_worn_tint(&self, stack: &ItemStack, tint: &mut Option<u32>) {
        let _ = (stack, tint);
    }
    /// Set `glider` to let the worn gear glide as an elytra does (`GLIDING_FLIGHT`), asked every tick with the player
    /// swapped in. Server-side, for every player of the world. API 54.
    pub(super) fn native_glider(&mut self, glider: &mut bool) {
        let _ = glider;
    }
    /// The gliding player's chest gear takes `wear` points of wear (`Equippable.onGlideDamage`, one a second): set
    /// `claimed` to pay for it another way (energy) so the item takes none. Server-side, for every player of the
    /// world. API 54.
    pub(super) fn native_glide_wear(&mut self, wear: u32, claimed: &mut bool) {
        let _ = (wear, claimed);
    }
    /// A player uses `stack` (right click) with `self.hand` on the block at
    /// `pos`, `face` the face clicked (`[0, 1, 0]` is the top), once the
    /// block itself let the click through (a chest still opens): where
    /// `ItemStack.useOn` starts. Set `claimed` to take the use: the hand
    /// swings and neither the item's `useOn` nor its `use` runs.
    /// Server-side, for every player of the world (the helpers read the
    /// acting one); an item on cooldown is never used. API 13.
    pub(super) fn native_use_item_on(
        &mut self,
        stack: &ItemStack,
        pos: [i32; 3],
        face: [i32; 3],
        claimed: &mut bool,
    ) {
        let _ = (stack, pos, face, claimed);
    }
    /// A player uses `stack` (right click) with `self.hand`: in the air, or
    /// on a block once `native_use_item_on` let it through, where `Item.use`
    /// starts. Each click reaches it once. Set `claimed` to take the use: the
    /// hand swings and the item's own use doesn't run. Server-side, for every
    /// player of the world; an item on cooldown is never used. API 13.
    pub(super) fn native_use_item(&mut self, stack: &ItemStack, claimed: &mut bool) {
        let _ = (stack, claimed);
    }
    /// Every tick of a use that `native_start_using` began, for the item in use (`Item.onUseTick`): set
    /// `claimed` to own the use, and `keep` to go on with it next tick (left false, the use ends, as
    /// `releaseUsingItem`). Server-side, for every player of the world. API 50.
    pub(super) fn native_using_tick(
        &mut self,
        stack: &ItemStack,
        ticks: u32,
        claimed: &mut bool,
        keep: &mut bool,
    ) {
        let _ = (stack, ticks, claimed, keep);
    }
    /// A player breaks the block `state` at `pos`, once the game allowed it
    /// (the held item can break blocks, the debug stick didn't take the
    /// click): where `ServerPlayerGameMode.destroyBlock` starts. Set
    /// `canceled` to keep the block: nothing is destroyed or dropped, and a
    /// guest sees it back. Server-side, for every player of the world (the
    /// helpers read the acting one). API 14.
    pub(super) fn native_destroy_block(
        &mut self,
        pos: [i32; 3],
        state: BlockStateId,
        canceled: &mut bool,
    ) {
        let _ = (pos, state, canceled);
    }
    /// A player placed `stack` as the block `state` at `pos`: where
    /// `BlockItem.place` set it, before the stack shrinks. It only hears; a
    /// mod stops a placement by claiming `native_use_item_on`, which runs
    /// before it. Server-side, for every player of the world. API 14.
    pub(super) fn native_place_block(
        &mut self,
        stack: &ItemStack,
        pos: [i32; 3],
        state: BlockStateId,
    ) {
        let _ = (stack, pos, state);
    }
    /// The block at `pos` stopped being `old` (name and properties as it
    /// was) and is `new` (a name) now, whatever did it: a player, an
    /// explosion, a piston, a command, a fluid. Called once the change and
    /// its updates are done, so the hook may set blocks. A mod's multi-cell
    /// machine clears its other cells here. Server-side. API 43.
    pub(super) fn native_block_removed(
        &mut self,
        pos: [i32; 3],
        old: &str,
        old_properties: &std::collections::BTreeMap<String, String>,
        new: &str,
    ) {
        let _ = (pos, old, old_properties, new);
    }
    /// A player right clicks or breaks the block `state` at `pos`: set `to`
    /// to another position and the click or break acts there instead (the
    /// block, its use, its drops and its break all belong to that position).
    /// A mod's multi-cell machine answers for its filler blocks with the
    /// position of the block that owns them (Mekanism's bounding blocks).
    /// Called before `native_right_click_block` and `native_destroy_block`,
    /// server-side, for every player of the world. API 43.
    pub(super) fn native_block_redirect(
        &self,
        pos: [i32; 3],
        state: BlockStateId,
        to: &mut Option<[i32; 3]>,
    ) {
        let _ = (pos, state, to);
    }
    /// A player right clicks the block at `pos` with `self.hand`, `face` the
    /// face clicked: where `ServerPlayerGameMode.useItemOn` starts, before
    /// the block's own use (a chest, a door, a lever) and before
    /// `native_use_item_on`. Set `canceled` to stop the click: neither the
    /// block nor the item acts, the hand doesn't swing, and a guest sees the
    /// block as it is. Server-side, for every player of the world
    /// (`native_acting_player` says who), once per hand. API 21.
    pub(super) fn native_right_click_block(
        &mut self,
        pos: [i32; 3],
        face: [i32; 3],
        canceled: &mut bool,
    ) {
        let _ = (pos, face, canceled);
    }
    /// A mod's packet from the server reached this client: `channel` is the
    /// mod's own (`mymod:sync`), `data` what the server's
    /// `native_send_to_player` sent. On every client of a server, the
    /// window's own in single player included. API 22.
    pub(super) fn native_client_payload(&mut self, channel: &str, data: &[u8]) {
        let _ = (channel, data);
    }
    /// A mod's packet from player `player` (UUID) reached the server:
    /// `channel` is the mod's own, `data` what that player's client sent with
    /// `native_send_to_server`, at most `NATIVE_PAYLOAD_TO_SERVER` bytes.
    /// Server-side; trust nothing in it. API 22. Since API 26 the player's
    /// state is swapped in, as for `native_right_click_block`: the hook can
    /// read and change their inventory and armor, and `native_acting_player`
    /// names them.
    pub(super) fn native_server_payload(&mut self, player: u128, channel: &str, data: &[u8]) {
        let _ = (player, channel, data);
    }
    /// A mod block entity's client view reached this client: `client` is
    /// the `client` compound of the block entity `id` at `pos` on the
    /// server, sent with its chunk, after its block changed, or when the
    /// mod called `native_sync_block_entity`. Not called on the server's own
    /// `Game`. API 23.
    pub(super) fn native_client_block_entity_changed(
        &mut self,
        pos: [i32; 3],
        id: &str,
        client: &horizon_world::nbt::Compound,
    ) {
        let _ = (pos, id, client);
    }
}

/// Helpers for native mods, kept as stable as the hook points.
/// Server-level helpers for native mods.
#[allow(dead_code)]
impl GameLevel<'_> {
    /// Queue particles in this authoritative level. A zero count sends one
    /// directional particle. The return value means accepted, not delivered.
    /// Block particle types require their block state; item, trail and
    /// vibration particles are not supported by the current network dispatch.
    pub(super) fn native_send_particles(
        &mut self,
        options: &ParticleOptions,
        state: Option<BlockStateId>,
        at: [f64; 3],
        spread: [f64; 3],
        speed: f64,
        count: i32,
    ) -> bool {
        use ParticleType as T;
        let supported = match options.ty {
            T::Block | T::BlockMarker | T::FallingDust | T::DustPillar | T::BlockCrumble => {
                state.is_some_and(|id| self.registry.state(id).is_some())
            }
            T::Item | T::Trail | T::Vibration => false,
            T::Geyser | T::GeyserPlume | T::GeyserBase | T::GeyserPoof => options
                .geyser
                .is_some_and(|(_, impulse)| impulse.is_finite()),
            _ => true,
        };
        if !self.native_authoritative
            || !supported
            || !at.into_iter().all(f64::is_finite)
            || !spread
                .into_iter()
                .all(|v| v.is_finite() && v.abs() <= f32::MAX as f64)
            || !speed.is_finite()
            || speed.abs() > f32::MAX as f64
            || !options.color.into_iter().all(f32::is_finite)
            || !options.to_color.into_iter().all(f32::is_finite)
            || !options.scale.is_finite()
        {
            return false;
        }
        let dimension = match self.dimension {
            LevelDimension::Overworld => "minecraft:overworld",
            LevelDimension::Nether => "minecraft:the_nether",
            LevelDimension::End => "minecraft:the_end",
        };
        self.native_particles
            .push((dimension, options.clone(), state, at, spread, speed, count));
        true
    }
}

// Native mods use these; nothing in Horizon does.
#[allow(dead_code)]
impl Game<'_> {
    /// Sends a mod's packet on `channel` to player `uuid`'s client, where
    /// `native_client_payload` hears it: a server mod telling a client what a
    /// machine shows. False when the channel is not a mod's (namespaced, not
    /// `minecraft:` or `horizon:`), `data` is over `NATIVE_PAYLOAD_TO_CLIENT`
    /// bytes, or the player isn't on. Inside that player's own hook it goes
    /// with the hook's other packets. API 22.
    pub(super) fn native_send_to_player(&mut self, uuid: u128, channel: &str, data: &[u8]) -> bool {
        if !mod_channel(channel) || data.len() > NATIVE_PAYLOAD_TO_CLIENT {
            return false;
        }
        let packet = horizon_protocol::packets::play::Clientbound::CustomPayload(
            horizon_protocol::packets::configuration::CustomPayload {
                channel: channel.into(),
                data: data.to_vec(),
            },
        );
        if self.lan_acting_uuid == Some(uuid) {
            self.guest_packets.push(packet);
            return true;
        }
        let Some(host) = &self.lan else {
            return false;
        };
        let Some(g) = host.hub.guests.iter().find(|g| g.placed && g.uuid == uuid) else {
            return false;
        };
        host.hub.send_to(g.conn, &packet);
        true
    }
    /// Sets the system clipboard (a mod text box's copy and cut). API 33.
    pub(super) fn native_set_clipboard(&mut self, text: &str) {
        use horizon_client::clipboard::Clipboard;
        self.clipboard.set(text);
    }
    /// Sends a mod's packet on `channel` from this client to the server it
    /// plays on, where `native_server_payload` hears it: a button with a
    /// value, a key held. False with no server, a channel that is not a mod's,
    /// or `data` over `NATIVE_PAYLOAD_TO_SERVER` bytes. API 22.
    pub(super) fn native_send_to_server(&self, channel: &str, data: &[u8]) -> bool {
        if !mod_channel(channel) || data.len() > NATIVE_PAYLOAD_TO_SERVER || self.remote.is_none() {
            return false;
        }
        self.send(horizon_protocol::packets::play::Serverbound::CustomPayload(
            horizon_protocol::packets::configuration::CustomPayload {
                channel: channel.into(),
                data: data.to_vec(),
            },
        ));
        true
    }
    /// Sends the `client` compound of the mod block entity at `pos` to the
    /// players who have its chunk, at the end of this tick: what a machine
    /// shows in the world (lit, a tank's fill). Call it after changing that
    /// compound, in the block entity's tick too. Keep it small and change it
    /// only when what players see changes: it travels to every player near
    /// the block. Clients get it with the chunk anyway. API 23.
    pub(super) fn native_sync_block_entity(&mut self, pos: [i32; 3]) {
        if self.world.block_log.is_some() {
            self.world
                .mod_block_entity_log
                .push((pos[0], pos[1], pos[2]));
        }
    }
    /// The id and client view (`client` compound) of the mod block entity at
    /// `pos`, as this client last heard it from the server; on the server's
    /// own `Game` (a LAN host), the block entity's own. API 23.
    pub(super) fn native_client_block_entity(
        &self,
        pos: [i32; 3],
    ) -> Option<(&str, &horizon_world::nbt::Compound)> {
        let Some(BlockEntity::Mod(m)) = self.world.block_entities.get(&(pos[0], pos[1], pos[2]))
        else {
            return None;
        };
        if m.id.is_empty() {
            return None;
        }
        if self.remote.is_some() {
            return Some((&m.id, &m.nbt));
        }
        match m.nbt.get(&"client".into()) {
            Some(horizon_world::nbt::Tag::Compound(client)) => Some((&m.id, client)),
            _ => None,
        }
    }
    /// Whether the player left mod `mod_id` on in Options > Mods. A mod
    /// turned off does nothing (check this first in each hook) until the
    /// launcher's next build leaves it out.
    pub(super) fn native_on(&self, mod_id: &str) -> bool {
        !self.settings.mods.disabled.contains(mod_id)
    }
    /// Sets property `key` of the block at `pos` to `value`, as a furnace's
    /// tick sets its `lit` (`Level.setBlock` with `UPDATE_ALL`: neighbours
    /// and clients are told); false when the block has no such property or
    /// value. The block entity stays. API 18.
    pub(super) fn native_set_block_property(
        &mut self,
        pos: [i32; 3],
        key: &str,
        value: &str,
    ) -> bool {
        let registry = self.pack.assets.registry;
        let state = self.world.block(pos[0], pos[1], pos[2]);
        let Some(mut s) = registry.state(state).cloned() else {
            return false;
        };
        match s.properties.get(key) {
            Some(v) if v == value => return true,
            Some(_) => {}
            None => return false,
        }
        s.properties.insert(key.into(), value.into());
        let Ok(id) = registry.resolve(&s) else {
            return false;
        };
        let at = BlockPos::new(pos[0], pos[1], pos[2]);
        self.with_level(|b, level, ticks| b.set_block(level, ticks, at, id, flags::UPDATE_ALL))
    }
    /// Sets the block at `pos` to `block` with the given properties (every
    /// other property at its default), as `Level.setBlock` with
    /// `UPDATE_ALL`: neighbours and clients are told. False when the block
    /// or a property value does not exist. A block entity at `pos` goes with
    /// the old block. `block` of `minecraft:air` removes the block. A mod's
    /// multi-cell machine fills its other cells this way. API 43.
    pub(super) fn native_set_block(
        &mut self,
        pos: [i32; 3],
        block: &str,
        properties: &[(&str, &str)],
    ) -> bool {
        let registry = self.pack.assets.registry;
        let Some(mut state) = registry
            .default_state(block)
            .and_then(|d| registry.state(d).cloned())
        else {
            return false;
        };
        for (k, v) in properties {
            state.properties.insert((*k).into(), (*v).into());
        }
        let Ok(id) = registry.resolve(&state) else {
            return false;
        };
        let at = BlockPos::new(pos[0], pos[1], pos[2]);
        self.with_level(|b, level, ticks| b.set_block(level, ticks, at, id, flags::UPDATE_ALL))
    }
    /// Runs `change` on the mod block entity at `pos` (a generator charging
    /// the machine beside it) and returns what it returns; none for any
    /// other block entity, and for the one ticking now, which is out of the
    /// world. Afterwards, as its `setChanged` would, its chunk is saved next
    /// time and, when its items changed, comparators read it again. API 18.
    pub(super) fn native_with_mod_block_entity<R>(
        &mut self,
        pos: [i32; 3],
        change: impl FnOnce(&mut ModBlockEntity) -> R,
    ) -> Option<R> {
        let key = (pos[0], pos[1], pos[2]);
        let Some(BlockEntity::Mod(m)) = self.world.block_entities.get_mut(&key) else {
            return None;
        };
        if m.id.is_empty() {
            return None;
        }
        let items = m.items.clone();
        let out = change(m);
        let moved = m.items != items;
        self.world.modified.insert((key.0 >> 4, key.2 >> 4));
        if moved {
            self.container_changed(key);
        }
        Some(out)
    }
    /// Playing, with nothing over the world: keys and zoom apply.
    pub(super) fn native_playing(&self) -> bool {
        self.screen == Screen::Playing
            && self.title.is_none()
            && self.sign_edit.is_none()
            && self.book.is_none()
    }
    /// This `Game` is the window's, not a server's.
    pub(super) fn native_window(&self) -> bool {
        self.options.execution_role == ExecutionRole::Client
    }
    /// Whether a box in `corner` shows: F3's debug screen takes both top
    /// corners (its left and right columns), and a bottom corner while one
    /// of its charts is drawn there.
    pub(super) fn native_corner_shown(&self, corner: Corner) -> bool {
        let charts = self.debug_charts.corners(!self.has_singleplayer_server());
        !debug_takes(corner, self.debug, charts)
    }
    /// The width of `text` in GUI pixels.
    pub(super) fn native_text_width(&self, text: &str) -> f32 {
        HudBatch::text_width(&self.gui.atlas, text)
    }
    /// A rectangle in GUI pixels, `argb` (0x78000000 is the extras' box).
    pub(super) fn native_fill(&self, batch: &mut HudBatch, rect: [f32; 4], argb: u32) {
        let [x, y, w, h] = rect;
        let [a, r, g, b] = argb.to_be_bytes();
        batch.fill(&self.gui.atlas, x, y, w, h, rgba(r, g, b, a));
    }
    /// Any four-cornered shape in GUI pixels in one colour, `argb`, corners in
    /// order round it; `additive` adds the colour to what is behind it
    /// (vanilla's lightning pipeline). API 39.
    pub(super) fn native_fill_quad(
        &self,
        batch: &mut HudBatch,
        corners: [[f32; 2]; 4],
        argb: u32,
        additive: bool,
    ) {
        let [a, r, g, b] = argb.to_be_bytes();
        let start = batch.vertices.len();
        batch.fill_quad(&self.gui.atlas, corners, rgba(r, g, b, a));
        if additive {
            batch.blend_since(start, horizon_client::hud::Blend::Additive);
        }
    }
    /// Part `src` (`[x, y, w, h]` in the sprite's own pixels) of sprite
    /// `name` (vanilla's `gui/...`, or a content mod's GUI texture
    /// `mymod:gui/...`), drawn stretched into `dst` (`[x, y, w, h]` in GUI
    /// pixels). False when there is no such sprite. API 19.
    pub(super) fn native_sprite(
        &self,
        batch: &mut HudBatch,
        name: &str,
        src: [f32; 4],
        dst: [f32; 4],
    ) -> bool {
        let Some(mut r) = self.gui.atlas.sprites.get(name).copied() else {
            return false;
        };
        let (du, dv) = ((r.u1 - r.u0) / r.w as f32, (r.v1 - r.v0) / r.h as f32);
        r.u0 += src[0] * du;
        r.v0 += src[1] * dv;
        r.u1 = r.u0 + src[2] * du;
        r.v1 = r.v0 + src[3] * dv;
        batch.rect_uv(dst[0], dst[1], dst[2], dst[3], r, u32::MAX);
        true
    }
    /// `native_sprite` multiplied by the colour `argb` (white draws it as
    /// it is): a tab tinted its type's colour, a gauge's chemical. API 31.
    pub(super) fn native_sprite_tinted(
        &self,
        batch: &mut HudBatch,
        name: &str,
        src: [f32; 4],
        dst: [f32; 4],
        argb: u32,
    ) -> bool {
        let Some(mut r) = self.gui.atlas.sprites.get(name).copied() else {
            return false;
        };
        let (du, dv) = ((r.u1 - r.u0) / r.w as f32, (r.v1 - r.v0) / r.h as f32);
        r.u0 += src[0] * du;
        r.v0 += src[1] * dv;
        r.u1 = r.u0 + src[2] * du;
        r.v1 = r.v0 + src[3] * dv;
        let [a, red, g, b] = argb.to_be_bytes();
        batch.rect_uv(dst[0], dst[1], dst[2], dst[3], r, rgba(red, g, b, a));
        true
    }
    /// Item `item`'s icon (`mymod:machine`) with its top left at `at`,
    /// `size` GUI pixels square (16 in a slot); false for an unknown item.
    /// API 20.
    pub(super) fn native_item(
        &self,
        batch: &mut HudBatch,
        item: &str,
        at: [f32; 2],
        size: f32,
    ) -> bool {
        if self.res.items().get(item).is_none() {
            return false;
        }
        let start = batch.vertices.len();
        self.draw_item_model(batch, &ItemStack::new(item, 1), at[0], at[1]);
        batch.scale_since(start, at[0], at[1], size / 16.0, size / 16.0);
        true
    }
    /// Stack `stack`'s icon, with its components (a `custom_model_data`
    /// that picks the model), as `native_item` draws an item's. API 32.
    pub(super) fn native_item_stack(
        &self,
        batch: &mut HudBatch,
        stack: &ItemStack,
        at: [f32; 2],
        size: f32,
    ) -> bool {
        if self.res.items().get(&stack.item).is_none() {
            return false;
        }
        let start = batch.vertices.len();
        self.draw_item_model(batch, stack, at[0], at[1]);
        batch.scale_since(start, at[0], at[1], size / 16.0, size / 16.0);
        true
    }
    /// The open menu's title (the block's name, or the one it was given),
    /// as plain text. API 20.
    pub(super) fn native_screen_title(&self) -> Option<String> {
        self.remote_menu_title().map(|t| t.plain())
    }
    /// A data file of the game's resources, content mods' included
    /// (`data/mymod/machines.json`), when there is one: what a mod's content
    /// says, read at runtime. Only relative paths of plain names. API 20.
    pub(super) fn native_data(&self, path: &str) -> Option<Vec<u8>> {
        let plain = |p: &str| !p.is_empty() && p != "." && p != ".." && !p.contains('\\');
        if !path.starts_with("data/") || !path.split('/').all(plain) {
            return None;
        }
        fs::read(self.options.resources.join(path)).ok()
    }
    /// The block at `pos` (`mymod:cable`; air where nothing is loaded).
    /// API 20.
    pub(super) fn native_block(&self, pos: [i32; 3]) -> String {
        let state = self.world.block(pos[0], pos[1], pos[2]);
        self.res.facts().get(state).name.to_string()
    }
    /// Property `key` of the block at `pos` (a machine's `facing`), if it
    /// has one. API 20.
    pub(super) fn native_block_property(&self, pos: [i32; 3], key: &str) -> Option<String> {
        let state = self.world.block(pos[0], pos[1], pos[2]);
        self.pack
            .assets
            .registry
            .state(state)?
            .properties
            .get(key)
            .cloned()
    }
    /// Whether the side `face` of the block at `pos` is full and sturdy
    /// (`canOcclude` and `isFaceSturdy(.., FULL)`, a bin's cover test). API 47.
    pub(super) fn native_face_sturdy(&self, pos: [i32; 3], face: Direction) -> bool {
        let state = self.world.block(pos[0], pos[1], pos[2]);
        let facts = self.res.facts().get(state);
        facts.can_occlude && facts.sturdy_full & (1 << face as u32) != 0
    }
    /// Starts using the held item (`startUsingItem`): `native_using_tick` follows while the use key is
    /// held. Call it from `native_use_item`, which should claim the click. API 50.
    #[allow(dead_code)]
    pub(super) fn native_start_using(&mut self) {
        if self.using.is_none() {
            self.using = Some((self.held_slot(), 0));
        }
    }
    /// The first block hit along the segment `from`..`to`: the point, the block and the face's normal.
    /// API 50.
    #[allow(dead_code)]
    pub(super) fn native_clip(
        &self,
        from: [f64; 3],
        to: [f64; 3],
    ) -> Option<([f64; 3], [i32; 3], [i32; 3])> {
        let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if length < 1e-9 {
            return None;
        }
        let hit = self.clip_ray(from, d.map(|v| v / length), length)?;
        Some((hit.point, hit.pos, hit.face))
    }
    /// The eyes of the player, in a world. API 50.
    #[allow(dead_code)]
    pub(super) fn native_eye(&self) -> Option<[f64; 3]> {
        self.eye_position()
    }
    /// Keeps a looping sound going: `key` names it, `event` is a sound event (`mekanism:tile.machine.industrial_alarm`),
    /// `at` its place in the world (`None`: heard everywhere, unattenuated). Call it every tick or frame it should
    /// sound; it stops a quarter second after the last call, or when `native_stop_loop_sound` is called. The first
    /// call for a key starts it; later calls keep it (position and volume of the first call stay). API 51.
    #[allow(dead_code)]
    pub(super) fn native_loop_sound(
        &mut self,
        key: &str,
        event: &str,
        at: Option<[f64; 3]>,
        volume: f32,
        pitch: f32,
    ) {
        let now = Instant::now();
        if let Some(entry) = self.native_loops.get_mut(key) {
            entry.1 = now;
            return;
        }
        let id = self.audio.play_loop(event, at, volume, pitch);
        self.native_loops.insert(key.to_owned(), (id, now));
    }
    /// Stops the looping sound `key` now. API 51.
    #[allow(dead_code)]
    pub(super) fn native_stop_loop_sound(&mut self, key: &str) {
        if let Some((id, _)) = self.native_loops.remove(key) {
            self.audio.stop_and_release(id);
        }
    }
    /// Stops the loops nobody has refreshed lately (run every game tick).
    pub(super) fn sweep_native_loops(&mut self) {
        if self.native_loops.is_empty() {
            return;
        }
        let now = Instant::now();
        let stale: Vec<String> = self
            .native_loops
            .iter()
            .filter(|(_, (_, seen))| now.duration_since(*seen) > Duration::from_millis(250))
            .map(|(k, _)| k.clone())
            .collect();
        for key in stale {
            self.native_stop_loop_sound(&key);
        }
    }
    /// The block id of the mod screen showing now, if any. API 48.
    #[allow(dead_code)]
    pub(super) fn native_open_screen_block(&self) -> Option<String> {
        self.shown_native_screen().map(|s| s.block.clone())
    }
    /// Where mob `id` is drawn this frame: its interpolated position (feet), body yaw and head yaw in degrees and
    /// head pitch. API 48.
    #[allow(dead_code)]
    pub(super) fn native_mob_pose(&self, id: u32) -> Option<([f64; 3], f32, f32, f32)> {
        let m = self.mobs.iter().find(|m| m.id == id && !m.removed)?;
        let a = self.clock.fraction();
        let lerp = |p: f64, q: f64| p + (q - p) * a;
        let turn = |p: f32, q: f32| {
            let d = (q - p).rem_euclid(360.0);
            p + (if d > 180.0 { d - 360.0 } else { d }) * a as f32
        };
        Some((
            [
                lerp(m.previous.x, m.position.x),
                lerp(m.previous.y, m.position.y),
                lerp(m.previous.z, m.position.z),
            ],
            turn(m.previous_body_yaw, m.body_yaw),
            turn(m.previous_head_yaw, m.head_yaw),
            m.previous_head_pitch + (m.head_pitch - m.previous_head_pitch) * a as f32,
        ))
    }
    /// The camera's right and up directions in the world, for a billboard. API 47.
    #[allow(dead_code)]
    pub(super) fn native_camera_axes(&self) -> ([f64; 3], [f64; 3]) {
        let (yaw, pitch) = (
            f64::from(self.yaw).to_radians(),
            f64::from(self.pitch).to_radians(),
        );
        let right = glam::DVec3::new(-yaw.cos(), 0.0, -yaw.sin());
        let forward = glam::DVec3::new(
            -yaw.sin() * pitch.cos(),
            -pitch.sin(),
            yaw.cos() * pitch.cos(),
        );
        (right.to_array(), right.cross(forward).to_array())
    }
    /// Puts `stack` into the container at `pos` (a chest, a furnace, a mod
    /// container) through its side `face`, as a hopper would, and returns
    /// what did not fit: a machine's auto-output. API 20.
    pub(super) fn native_insert_item(
        &mut self,
        pos: [i32; 3],
        face: Direction,
        stack: ItemStack,
    ) -> Option<ItemStack> {
        self.insert_into_container((pos[0], pos[1], pos[2]), stack, face)
    }
    /// Takes up to `max` items of one stack out of the container at `pos`
    /// (a chest, a furnace's output, a mod container's outputs) through its
    /// side `face`, as a hopper there would: a mod's item pipe. `None` when
    /// nothing can be taken. API 24.
    pub(super) fn native_extract_item(
        &mut self,
        pos: [i32; 3],
        face: Direction,
        max: u32,
    ) -> Option<ItemStack> {
        self.extract_from_container((pos[0], pos[1], pos[2]), face, max)
    }
    /// `native_extract_item`, taking only from a stack `wanted` accepts: the
    /// first such stack the side gives out. API 29.
    pub(super) fn native_extract_item_where(
        &mut self,
        pos: [i32; 3],
        face: Direction,
        max: u32,
        wanted: &dyn Fn(&ItemStack) -> bool,
    ) -> Option<ItemStack> {
        self.extract_from_container_where((pos[0], pos[1], pos[2]), face, max, wanted)
    }
    /// The ids of the living mobs of type `kind` in the level in place. API 46.
    pub(super) fn native_mob_ids(&self, kind: &str) -> Vec<u32> {
        self.mobs
            .iter()
            .filter(|m| !m.removed && m.kind.id == kind)
            .map(|m| m.id)
            .collect()
    }
    /// Runs `f` on mob `id` of the level in place (its position, motion, yaw, health, `extra.mod_data`...). API 46.
    pub(super) fn native_with_mob<R>(
        &mut self,
        id: u32,
        f: impl FnOnce(&mut horizon_client::mobs::Mob) -> R,
    ) -> Option<R> {
        self.mobs
            .iter_mut()
            .find(|m| m.id == id && !m.removed)
            .map(f)
    }
    /// Where the mouse is, in GUI pixels. API 20.
    pub(super) fn native_cursor(&self) -> [f32; 2] {
        [self.cursor.0, self.cursor.1]
    }
    /// Whether the left mouse button is down: a window of the mod's own follows the
    /// cursor while it is (`native_screen_click` sees the press only). API 35.
    pub(super) fn native_mouse_down(&self) -> bool {
        self.left_down
    }
    /// The size of sprite `name` in its own pixels, if there is one. API 19.
    pub(super) fn native_sprite_size(&self, name: &str) -> Option<[f32; 2]> {
        let r = self.gui.atlas.sprites.get(name)?;
        Some([r.w as f32, r.h as f32])
    }
    /// A line of text at `(x, y)` in GUI pixels, opaque `rgb`.
    pub(super) fn native_text(
        &self,
        batch: &mut HudBatch,
        text: &str,
        at: [f32; 2],
        rgb: u32,
        shadow: bool,
    ) {
        let [x, y] = at;
        let [_, r, g, b] = rgb.to_be_bytes();
        let color = rgba(r, g, b, 0xFF);
        batch.text_styled(&self.gui.atlas, &Text::literal(text), x, y, color, shadow);
    }
    /// `native_text` scaled by `scale` from its top left (Mekanism's
    /// 0.6-scale screen text). API 32.
    pub(super) fn native_text_scaled(
        &self,
        batch: &mut HudBatch,
        text: &str,
        at: [f32; 2],
        rgb: u32,
        scale: f32,
    ) {
        let start = batch.vertices.len();
        self.native_text(batch, text, at, rgb, false);
        batch.scale_since(start, at[0], at[1], scale, scale);
    }
    /// The living mobs and the dropped items whose boxes overlap the box
    /// `min`..`max`, nearest to `from` first. Players are not listed. API 30.
    pub(super) fn native_entities_in(
        &self,
        min: [f64; 3],
        max: [f64; 3],
        from: [f64; 3],
    ) -> Vec<NativeEntity> {
        let overlaps = |feet: [f64; 3], half: f64, height: f64| {
            feet[0] + half > min[0]
                && feet[0] - half < max[0]
                && feet[1] + height > min[1]
                && feet[1] < max[1]
                && feet[2] + half > min[2]
                && feet[2] - half < max[2]
        };
        let mut found: Vec<NativeEntity> = Vec::new();
        for m in self.mobs.iter().filter(|m| m.alive()) {
            let feet = [m.position.x, m.position.y, m.position.z];
            if overlaps(feet, m.kind.width / 2.0, m.kind.height) {
                found.push(NativeEntity {
                    id: m.id,
                    kind: m.kind.id.clone(),
                    position: feet,
                });
            }
        }
        for item in &self.dropped {
            let feet = [item.position.x, item.position.y, item.position.z];
            if overlaps(feet, 0.125, 0.25) {
                found.push(NativeEntity {
                    id: item.id,
                    kind: "minecraft:item".into(),
                    position: feet,
                });
            }
        }
        let distance = |p: [f64; 3]| (0..3).map(|i| (p[i] - from[i]).powi(2)).sum::<f64>();
        found.sort_by(|a, b| distance(a.position).total_cmp(&distance(b.position)));
        found
    }
    /// Hurts mob `id` by `amount` (generic damage, no attacker) and sets it on
    /// fire for `fire_ticks` unless it is fire immune. The health it lost, 0
    /// when the hit did nothing. API 30.
    pub(super) fn native_hurt_mob(&mut self, id: u32, amount: f32, fire_ticks: i32) -> f32 {
        let Some(m) = self.mobs.iter_mut().find(|m| m.id == id && m.alive()) else {
            return 0.0;
        };
        if !m.kind.flags.fire_immune {
            m.fire_ticks = m.fire_ticks.max(fire_ticks);
        }
        let before = m.health;
        let source = horizon_client::mobs::Damage {
            kind: horizon_client::mobs::DamageKind::Generic,
            attacker: None,
            from: None,
        };
        if !m.damage(amount, source) {
            return 0.0;
        }
        (before - m.health.max(0.0)).max(0.0)
    }
    /// Takes the dropped item `id` out of the world, giving its stack. API 30.
    pub(super) fn native_take_item_entity(&mut self, id: u32) -> Option<ItemStack> {
        let at = self.dropped.iter().position(|i| i.id == id)?;
        Some(self.dropped.remove(at).stack)
    }
    /// The player, in a world.
    pub(super) fn native_player(&self) -> Option<NativePlayer> {
        let p = self.player.as_ref()?;
        Some(NativePlayer {
            x: p.position.x,
            y: p.position.y,
            z: p.position.z,
            yaw: self.yaw,
            pitch: self.pitch,
            dimension: self.dim.id(),
        })
    }
    /// The frame counter: frames in the last second, the slowest frame and
    /// the 1% low of the last 5 seconds, in frames per second.
    pub(super) fn native_frame_rate(&self) -> (u32, u32, u32) {
        self.frame_reading()
    }
    /// Attack and use clicks in the last second.
    pub(super) fn native_clicks(&mut self) -> (u32, u32) {
        self.mouse_reading().0
    }
    /// A vanilla mapping (`key.forward`, `key.jump`, `key.attack`…): whether
    /// it is held, and its key's label as Controls shows it.
    pub(super) fn native_mapping(&mut self, name: &str) -> Option<(bool, String)> {
        let &(_, key) = MAPPINGS.iter().find(|m| m.0 == name)?;
        let label = keybinds::display_name(self.settings.keys.get(name)).plain();
        let down = match key {
            Some(k) => self.keys.contains(&k),
            None => self.mouse_reading().1[usize::from(name == "key.use")],
        };
        Some((down, label))
    }
    /// The key the player chose for a mod's key `name`, else `default`
    /// (`key.keyboard.z`), as the WebAssembly mods' keys were kept
    /// (`key.<mod id>.<name>` in `horizon-mods.txt`).
    pub(super) fn native_mod_key(&self, mod_id: &str, name: &str, default: &str) -> String {
        let chosen = self
            .settings
            .mods
            .keys
            .get(mod_id)
            .and_then(|k| k.get(name));
        chosen.map_or(default, String::as_str).to_owned()
    }
    /// A line in chat from the client.
    pub(super) fn native_chat(&mut self, text: &str) {
        let focused = self.screen == Screen::Chat;
        let line = Text::literal(text);
        let atlas = &self.gui.atlas;
        self.ui
            .chat
            .add(atlas, line, None, ChatSource::Client, self.ticks, focused);
    }
    /// A native mod's own folder: `horizon-mods/.horizon-mod-data/<id>/`,
    /// where the WebAssembly mod of the same id kept its storage, so a
    /// player's settings carry over. Not created here.
    pub(super) fn native_data_dir(&self, mod_id: &str) -> PathBuf {
        let mods = Self::options_path().with_file_name(mod_list::DIR);
        mods.join(".horizon-mod-data").join(mod_id)
    }
    /// A mod's setting `key` (one file per key in `native_data_dir`).
    pub(super) fn native_setting(&self, mod_id: &str, key: &str) -> Option<String> {
        let path = self.native_data_dir(mod_id).join(key);
        fs::read_to_string(path).ok().map(|v| v.trim().to_owned())
    }
    /// Keeps a mod's setting `key`.
    pub(super) fn native_set_setting(&self, mod_id: &str, key: &str, value: &str) {
        let dir = self.native_data_dir(mod_id);
        let _ = fs::create_dir_all(&dir).and_then(|()| fs::write(dir.join(key), value));
    }
    /// Shows the panel beside container screens with these items (at most
    /// 8192), as the game's own slots in pages, with a search box; the
    /// player's clicks, searches and keys over items come back as
    /// `native_panel_event`. The panel is the last mod's to show one; empty
    /// items and no recipe page take yours away.
    pub(super) fn native_panel_items(&mut self, mod_id: &str, items: Vec<NativeStack>) {
        self.panel.show_items(mod_id, items);
    }
    /// Shows these recipes in the panel instead of its items, as many as
    /// fit a page, under `title` and a back button, with a row of `tabs`
    /// (each an icon; `tab` is the selected one) between them. At most 256
    /// recipes and 16 tabs; no recipes shows the items again. The same title
    /// and tab again keep the page the player turned to.
    pub(super) fn native_panel_recipes(
        &mut self,
        mod_id: &str,
        title: &str,
        tabs: Vec<NativeStack>,
        tab: u32,
        views: Vec<NativeRecipeView>,
    ) {
        let page = RecipePage::new(title, tabs, tab, views);
        self.panel.show_recipes(mod_id, page);
    }
    /// Every item and recipe the game knows, made once (the same for every
    /// call); none outside a world. They are the window's own: the world's
    /// in single player, vanilla's on a server, which sends a client no
    /// others.
    pub(super) fn native_recipes(&mut self) -> Option<Arc<NativeRecipes>> {
        self.player.as_ref()?;
        self.made_recipes()
    }
    /// The `book_id`s of the recipes the player's recipe book holds now
    /// (the server's book on a server); empty outside a world.
    pub(super) fn native_known_recipes(&mut self) -> Arc<[i32]> {
        if self.player.is_none() {
            return Arc::from([]);
        }
        self.known_displays()
    }
    /// Places a recipe in the open screen's grid as a click in the recipe
    /// book does (`ServerboundPlaceRecipePacket`): the inventory's grid or a
    /// crafting table's. `all` places as many as the items allow, as with
    /// Shift. Refused while no such screen is open and for a recipe the
    /// player's book doesn't hold; one missing items shows as a ghost.
    pub(super) fn native_place_recipe(&mut self, book_id: i32, all: bool) -> Result<(), String> {
        self.recipe_place(book_id, all)
    }
    /// Creative mode only: gives the player `count` of `item`
    /// (`minecraft:diamond`) as taking it from the creative inventory does
    /// (`ServerboundSetCreativeModeSlot`): the selected slot first, then the
    /// hotbar, then the inventory. The count is cut to one stack.
    pub(super) fn native_give(&mut self, item: &str, count: u32) -> Result<(), String> {
        self.recipe_give(item, count)
    }
}

/// Where the game calls the hook points.
impl Game<'_> {
    /// A mod block entity's client view from the server
    /// (`chunks::mod_block_entity_packet`): kept as the client's block
    /// entity there, which the block's next change drops.
    pub(super) fn remote_mod_block_entity(&mut self, data: &[u8]) {
        let Some((pos, id, client)) = horizon_client::lan::chunks::read_mod_block_entity(data)
        else {
            return;
        };
        let key = (pos[0], pos[1], pos[2]);
        match self.world.block_entities.get_mut(&key) {
            Some(BlockEntity::Mod(m)) => {
                m.id.clone_from(&id);
                m.nbt.clone_from(&client);
            }
            _ => {
                self.world.block_entities.insert(
                    key,
                    BlockEntity::Mod(Box::new(ModBlockEntity {
                        id: id.clone(),
                        nbt: client.clone(),
                        ..ModBlockEntity::default()
                    })),
                );
            }
        }
        self.native_client_block_entity_changed(pos, &id, &client);
    }
    /// `EntityBlock.newBlockEntity` of block `name`: Horizon's own, else a
    /// native mod's.
    pub(super) fn new_block_entity(&mut self, name: &str) -> Option<BlockEntity> {
        if let Some(entity) = BlockEntity::for_block(name) {
            return Some(entity);
        }
        let mut entity = None;
        self.native_block_entity(name, &mut entity);
        let (id, nbt) = entity?;
        Some(BlockEntity::Mod(Box::new(ModBlockEntity {
            id,
            nbt,
            ..ModBlockEntity::default()
        })))
    }
    /// The containers of the mod blocks' block entities, by block state
    /// (`native_block_entity_container`), checked.
    pub(super) fn native_containers(&self) -> FxHashMap<u32, Arc<ModContainer>> {
        let registry = self.pack.assets.registry;
        let mut by_block: FxHashMap<String, Option<Arc<ModContainer>>> = FxHashMap::default();
        let mut out = FxHashMap::default();
        for i in 0..registry.len() as u32 {
            let Some(state) = registry.state(horizon_world::registry::BlockStateId(i)) else {
                continue;
            };
            if state.name.starts_with("minecraft:") {
                continue;
            }
            let container = by_block.entry(state.name.clone()).or_insert_with(|| {
                let mut container = None;
                self.native_block_entity_container(&state.name, &mut container);
                container.map(|mut c| {
                    c.slots = c.slots.min(256);
                    let n = c.slots;
                    for list in c.faces.iter_mut().chain([&mut c.inputs, &mut c.outputs]) {
                        list.retain(|&s| s < n);
                    }
                    c.menu = c
                        .menu
                        .filter(|m| mod_menu(m).is_some_and(|(_, size)| size == n));
                    Arc::new(c)
                })
            });
            if let Some(c) = container {
                out.insert(i, Arc::clone(c));
            }
        }
        out
    }
    /// Whether a native mod's block `name` has a ticking block entity.
    pub(super) fn native_ticks(&self, name: &str) -> bool {
        let mut ticks = false;
        self.native_block_entity_ticks(name, &mut ticks);
        ticks
    }
    /// A mod block's container at `key` opens its menu: its slots grown to
    /// the menu's. False for any other block.
    pub(super) fn open_mod_menu(&mut self, key: Pos) -> bool {
        let state = self.world.block(key.0, key.1, key.2).0;
        let Some(c) = self.world.block_entities.mod_container(state) else {
            return false;
        };
        let slots = c.slots;
        if c.menu.is_none() {
            return false;
        }
        let Some(BlockEntity::Mod(m)) = self.world.block_entities.get_mut(&key) else {
            return false;
        };
        if m.items.len() < slots {
            m.items.resize(slots, None);
        }
        true
    }
    /// The menu type of the mod container at `key`, if it has one.
    pub(super) fn mod_menu_at(&self, key: Pos) -> Option<&'static str> {
        let state = self.world.block(key.0, key.1, key.2).0;
        let c = self.world.block_entities.mod_container(state)?;
        mod_menu(c.menu.as_deref()?).map(|(menu, _)| menu)
    }
    /// The container slots (of `size`) that the mod's screen for the block at
    /// `pos` does not show: the server's menu does not have them either, so a
    /// shift-click never moves an item where the player cannot see it.
    pub(super) fn native_hidden_slots(&self, pos: Pos, size: usize) -> Vec<usize> {
        if self.mod_menu_at(pos).is_none() {
            return Vec::new();
        }
        let block = self
            .res
            .facts()
            .get(self.world.block(pos.0, pos.1, pos.2))
            .name
            .to_string();
        let mut screen = None;
        self.native_screen(&block, &mut screen);
        let Some(screen) = screen else {
            return Vec::new();
        };
        (0..size)
            .filter(|&i| screen.slots.get(i).is_none_or(Option::is_none))
            .collect()
    }
    /// A menu opened on this client with its slots under `key`, on block
    /// `target`: a mod block's is laid out by its mod (`native_screen`).
    pub(super) fn open_native_screen(&mut self, key: Pos, target: Option<Pos>, window: i32) {
        self.mod_screen = None;
        let Some(t) = target else {
            return;
        };
        let block = self
            .res
            .facts()
            .get(self.world.block(t.0, t.1, t.2))
            .name
            .to_string();
        if block.starts_with("minecraft:") {
            return;
        }
        let mut screen = None;
        self.native_screen(&block, &mut screen);
        if let Some(screen) = screen {
            self.mod_screen = Some(OpenNativeScreen {
                key,
                window,
                block,
                screen,
                data: Vec::new(),
            });
        }
    }
    /// The layout of the open screen: its alternative while the mod says so.
    fn layout_of<'a>(&self, s: &'a OpenNativeScreen) -> &'a NativeScreen {
        let mut alt = false;
        self.native_screen_alt(&s.block, &mut alt);
        match (alt, s.screen.alt.as_deref()) {
            (true, Some(other)) => other,
            _ => &s.screen,
        }
    }
    /// The mod screen showing now, if any.
    fn shown_native_screen(&self) -> Option<&OpenNativeScreen> {
        self.mod_screen
            .as_ref()
            .filter(|s| self.screen == Screen::Inventory && self.container == Some(s.key))
    }
    /// A data slot of window `window`'s menu (`ContainerSetData`); false
    /// when no mod screen is showing for that window.
    pub(super) fn native_screen_data(&mut self, window: i32, id: i16, value: i16) -> bool {
        if self.shown_native_screen().is_none() {
            return false;
        }
        self.mod_screen
            .as_mut()
            .is_some_and(|s| s.set_data(window, id, value))
    }
    /// The open mod screen's height.
    pub(super) fn native_screen_height(&self) -> Option<f32> {
        self.shown_native_screen().map(|s| self.layout_of(s).height)
    }
    /// Where the open mod screen puts container slot `slot`.
    pub(super) fn native_screen_slot(&self, slot: usize) -> Option<(f32, f32)> {
        let s = self.shown_native_screen()?;
        self.layout_of(s)
            .slots
            .get(slot)
            .copied()
            .flatten()
            .map(|p| (p[0], p[1]))
    }
    /// The container slots the open mod screen shows (300.. ids).
    pub(super) fn native_screen_slots(&self) -> Option<Vec<usize>> {
        let s = self.shown_native_screen()?;
        let shown = self.layout_of(s).slots.iter().enumerate();
        Some(
            shown
                .filter(|(_, p)| p.is_some())
                .map(|(i, _)| 300 + i)
                .collect(),
        )
    }
    /// The player's worn slots the open mod screen shows (armor 500.., the off hand 504).
    pub(super) fn native_screen_worn_slots(&self) -> Vec<usize> {
        let Some(s) = self.shown_native_screen() else {
            return Vec::new();
        };
        let layout = self.layout_of(s);
        let mut v = Vec::new();
        if layout.armor.is_some() {
            v.extend(500..504);
        }
        if layout.offhand.is_some() {
            v.push(504);
        }
        v
    }
    /// Where the open mod screen puts worn slot `slot` (500..504 armor, 504 the off hand).
    pub(super) fn native_screen_armor_slot(&self, slot: usize) -> Option<(f32, f32)> {
        let s = self.shown_native_screen()?;
        let layout = self.layout_of(s);
        if slot == 504 {
            return layout.offhand.map(|p| (p[0], p[1]));
        }
        layout
            .armor
            .map(|p| (p[0], p[1] + (slot - 500) as f32 * 18.0))
    }
    /// The first row of the player's inventory on the open mod screen.
    pub(super) fn native_screen_inventory_y(&self) -> Option<f32> {
        self.shown_native_screen()
            .map(|s| self.layout_of(s).inventory_y)
    }
    /// The open mod screen's inventory's left column.
    pub(super) fn native_screen_inventory_x(&self) -> Option<f32> {
        self.shown_native_screen()
            .map(|s| self.layout_of(s).inventory_x)
    }
    /// The open mod screen's width.
    pub(super) fn native_screen_width(&self) -> Option<f32> {
        self.shown_native_screen().map(|s| self.layout_of(s).width)
    }
    /// The open mod screen: a plain panel with its slots' frames, the mod's
    /// drawing over it, then the title and the inventory's label. False when
    /// none is showing.
    pub(super) fn draw_native_screen(&self, batch: &mut HudBatch, ox: f32, oy: f32) -> bool {
        let Some(s) = self.shown_native_screen() else {
            return false;
        };
        let atlas = &self.gui.atlas;
        let layout = self.layout_of(s);
        let (w, h) = (layout.width, layout.height);
        batch.fill(atlas, ox, oy, w, h, rgba(0, 0, 0, 255));
        batch.fill(
            atlas,
            ox + 1.0,
            oy + 1.0,
            w - 2.0,
            h - 2.0,
            rgba(198, 198, 198, 255),
        );
        let container = self.native_screen_slots().unwrap_or_default();
        for i in (0..36).chain(container) {
            let (x, y) = self.slot_position(i);
            let (x, y) = (ox + x - 1.0, oy + y - 1.0);
            batch.fill(atlas, x, y, 18.0, 18.0, rgba(55, 55, 55, 255));
            batch.fill(
                atlas,
                x + 1.0,
                y + 1.0,
                17.0,
                17.0,
                rgba(255, 255, 255, 255),
            );
            batch.fill(
                atlas,
                x + 1.0,
                y + 1.0,
                16.0,
                16.0,
                rgba(139, 139, 139, 255),
            );
        }
        self.native_screen_draw(&s.block, &s.data, batch, [ox, oy]);
        if layout.own_labels {
            return true;
        }
        let grey = rgba(64, 64, 64, 255);
        if let Some(title) = self.remote_menu_title() {
            batch.text_styled(atlas, &title, ox + 8.0, oy + 6.0, grey, false);
        }
        let inventory = self.translate("container.inventory", "Inventory");
        let label_y = oy + layout.inventory_y - 11.0;
        batch.text(atlas, &inventory, ox + 8.0, label_y, grey, false);
        true
    }
    /// The tick of the mod block entity at `pos`.
    pub(super) fn tick_mod_block_entity(&mut self, pos: Pos) {
        let Some(BlockEntity::Mod(m)) = self.world.block_entities.get_mut(&pos) else {
            return;
        };
        // An empty id marks its place while the mod has it.
        let mut taken = std::mem::take(&mut **m);
        let items = taken.items.clone();
        let state = self.world.block(pos.0, pos.1, pos.2).0;
        let slots = self
            .world
            .block_entities
            .mod_container(state)
            .map_or(0, |c| c.slots);
        // Grown only: items a container no longer has room for stay saved.
        if taken.items.len() < slots {
            taken.items.resize(slots, None);
        }
        // Without a container, an `Items` item list is the mod's own field.
        if slots == 0 && !taken.items.is_empty() {
            let items = horizon_client::save::encode_items(&std::mem::take(&mut taken.items));
            taken.nbt.insert("Items".into(), items);
        }
        self.native_block_entity_tick([pos.0, pos.1, pos.2], &mut taken);
        self.put_back_mod_block_entity(pos, taken, &items);
    }
    /// Puts back a mod block entity taken out for a call, unless the call
    /// removed or replaced it, with what its `setChanged` does.
    fn put_back_mod_block_entity(
        &mut self,
        pos: Pos,
        mut taken: ModBlockEntity,
        items: &[Option<ItemStack>],
    ) {
        if let Some(BlockEntity::Mod(m)) = self.world.block_entities.get_mut(&pos)
            && m.id.is_empty()
        {
            let moved = taken.items != items;
            **m = taken;
            // BlockEntity.setChanged: the next save writes its chunk, and
            // comparators read a container whose items changed.
            self.world.modified.insert((pos.0 >> 4, pos.2 >> 4));
            if moved {
                self.container_changed(pos);
            }
        } else if taken.items.iter().any(Option::is_some) {
            // The mod removed or replaced its own block during the call. Vanilla
            // ticks it in the world and drops a container's contents on removal.
            let lost = std::mem::take(&mut taken.items);
            let at = BlockPos {
                x: pos.0,
                y: pos.1,
                z: pos.2,
            };
            self.with_level(|_, level, _| level.drop_lost_contents(at, lost));
            self.world.modified.insert((pos.0 >> 4, pos.2 >> 4));
        }
    }
    /// Menu button `id` on the mod container at `pos` (`clickMenuButton`).
    pub(super) fn mod_menu_button(&mut self, pos: Pos, id: i32) {
        if self.mod_menu_at(pos).is_none() {
            return;
        }
        let Some(BlockEntity::Mod(m)) = self.world.block_entities.get_mut(&pos) else {
            return;
        };
        if m.id.is_empty() {
            return;
        }
        let mut taken = std::mem::take(&mut **m);
        let items = taken.items.clone();
        self.native_menu_button([pos.0, pos.1, pos.2], &mut taken, id);
        self.put_back_mod_block_entity(pos, taken, &items);
    }
    /// The cursor from the open mod screen's top left, with its block and
    /// data, for its press and tooltip hooks.
    fn native_screen_at(&self) -> Option<(String, Vec<i32>, [f32; 2])> {
        let s = self.shown_native_screen()?;
        let r = self.renderer.as_ref()?;
        let batch = HudBatch::new([r.config.width, r.config.height]);
        let (ox, oy) = self.inventory_origin(&batch);
        let at = [self.cursor.0 - ox, self.cursor.1 - oy];
        Some((s.block.clone(), s.data.clone(), at))
    }
    /// A press on the open mod screen's own button (`native_screen_click`)
    /// goes to the server as a menu button click (true).
    pub(super) fn native_screen_press(&mut self, right: bool) -> bool {
        let covered = self.native_screen_covered();
        if !covered && self.inventory_target().0.is_some() {
            return false;
        }
        let Some((block, data, at)) = self.native_screen_at() else {
            return false;
        };
        let mut button = None;
        self.native_screen_click(&block, &data, at, right, &mut button);
        let Some(button) = button else {
            // A press on the mod's own window stops there.
            return covered;
        };
        let window = self
            .remote
            .as_ref()
            .and_then(|r| r.menu.as_ref())
            .map(|(w, _)| *w);
        if let Some(container) = window {
            self.send(
                horizon_protocol::packets::play::Serverbound::ContainerButtonClick(
                    horizon_protocol::packets::play::ContainerButtonClick { container, button },
                ),
            );
        }
        self.audio.play_button_click();
        true
    }
    /// Whether a text box of the open mod screen has focus
    /// (`native_screen_typing`): it takes the keys but Escape.
    pub(super) fn native_screen_typing_now(&self) -> bool {
        let Some(s) = self.shown_native_screen() else {
            return false;
        };
        let mut typing = false;
        self.native_screen_typing(&s.block, &s.data, &mut typing);
        typing
    }
    /// A key press while the open mod screen's text box has focus
    /// (`native_screen_key`).
    pub(super) fn native_screen_key_press(&mut self, event: &winit::event::KeyEvent) {
        use horizon_client::clipboard::{Clipboard, EditShortcut};
        let winit::keyboard::PhysicalKey::Code(code) = event.physical_key else {
            return;
        };
        let Some(name) = keybinds::key_name(code) else {
            return;
        };
        let shortcut = self.edit_shortcut(event);
        let (ctrl, shift) = (self.has_control_down(), self.has_shift_down());
        let text = match shortcut {
            Some(EditShortcut::Paste) => Some(self.clipboard.get()),
            Some(_) => None,
            None if self.types_text() => event
                .text
                .as_deref()
                .map(|t| t.chars().filter(|c| !c.is_control()).collect::<String>())
                .filter(|t| !t.is_empty()),
            None => None,
        };
        let Some((block, data)) = self
            .shown_native_screen()
            .map(|s| (s.block.clone(), s.data.clone()))
        else {
            return;
        };
        self.native_screen_key(&block, &data, name, text.as_deref(), ctrl, shift);
    }
    /// Whether the open mod screen's own drawing covers the cursor
    /// (`native_screen_covers`): its slots there are hidden from the player.
    pub(super) fn native_screen_covered(&self) -> bool {
        let Some((block, data, at)) = self.native_screen_at() else {
            return false;
        };
        let mut covered = false;
        self.native_screen_covers(&block, &data, at, &mut covered);
        covered
    }
    /// The open mod screen's overlay (`native_screen_overlay`), over its
    /// items and labels.
    pub(super) fn draw_native_screen_overlay(&self, batch: &mut HudBatch) {
        let Some(s) = self.shown_native_screen() else {
            return;
        };
        let (ox, oy) = self.inventory_origin(batch);
        self.native_screen_overlay(&s.block, &s.data, batch, [ox, oy]);
    }
    /// The open mod screen's tooltip under the cursor (`native_screen_tooltip`).
    pub(super) fn draw_native_screen_tooltip(&self, batch: &mut HudBatch) {
        let over_slot = self.slot_at(batch, self.cursor.0, self.cursor.1).is_some();
        if self.carried.is_some() || (over_slot && !self.native_screen_covered()) {
            return;
        }
        let Some((block, data, at)) = self.native_screen_at() else {
            return;
        };
        let mut lines = Vec::new();
        self.native_screen_tooltip(&block, &data, at, &mut lines);
        let lines: Vec<(horizon_client::text::Text, u32)> = lines
            .into_iter()
            .map(|l| (horizon_client::text::Text::literal(l), u32::MAX))
            .collect();
        self.draw_tooltip_lines(batch, &lines, self.cursor.0, self.cursor.1);
    }
    /// Options > Mods' rows for the native mods, after the others.
    pub(super) fn list_native_mods(&mut self) {
        let mut mods = Vec::new();
        self.native_mods(&mut mods);
        // A WebAssembly mod of the same id: the native row stands for it,
        // and alone toggles its `mod.<id>` line.
        self.settings
            .mods
            .installed
            .retain(|row| row.native.is_none() && !mods.iter().any(|m| m.id == row.id));
        for m in mods {
            let mut buttons = Vec::new();
            self.native_settings(&m.id, &mut buttons);
            let page = NativePage {
                keys: m.keys,
                buttons,
            };
            self.settings.mods.installed.push(Installed {
                id: m.id,
                title: m.title,
                description: m.description,
                problem: None,
                dev: false,
                native: Some(page),
            });
        }
    }
    /// A click on button `button` of native mod `i`'s page (its index in
    /// Options > Mods), and the page's labels after it.
    pub(super) fn native_settings_event(&mut self, i: usize, button: usize) {
        let Some(id) = self.settings.mods.installed.get(i).map(|m| m.id.clone()) else {
            return;
        };
        self.native_settings_click(&id, button);
        self.native_settings_refresh(i);
    }
    /// Native mod `i`'s page opened: its labels asked again, so that they
    /// follow the language (and the mod's state) since the list was made.
    pub(super) fn native_settings_refresh(&mut self, i: usize) {
        let Some(id) = self.settings.mods.installed.get(i).map(|m| m.id.clone()) else {
            return;
        };
        let mut buttons = Vec::new();
        self.native_settings(&id, &mut buttons);
        let row = self.settings.mods.installed.get_mut(i);
        if let Some(page) = row.and_then(|m| m.native.as_mut()) {
            page.buttons = buttons;
        }
    }
    /// A key event for `native_key`.
    pub(super) fn native_key_event(&mut self, key: KeyCode, pressed: bool, repeat: bool) {
        if repeat {
            return;
        }
        if let Some(name) = keybinds::key_name(key) {
            if pressed {
                self.native_panel_key(name);
            }
            self.native_key(name, pressed);
        }
    }
    /// A mouse button while playing, for `native_key`.
    pub(super) fn native_mouse_event(&mut self, button: MouseButton, pressed: bool) {
        let name = keybinds::mouse_name(button);
        self.native_key(&name, pressed);
    }
    /// Wheel steps while playing: whether a native mod took them.
    pub(super) fn native_wheel_event(&mut self, steps: f32) -> bool {
        let mut claimed = false;
        self.native_wheel(steps, &mut claimed);
        claimed
    }
    /// A native mod's command.
    pub(super) fn client_mod_command(&mut self, command: &str) -> bool {
        let mut ran = false;
        self.native_command(command, &mut ran);
        ran
    }
    /// The zoom in effect while playing (a native mod's); the spyglass's own
    /// view wins.
    pub(super) fn native_zoom_now(&self) -> Option<NativeZoom> {
        if !self.native_playing() || self.scoping() {
            return None;
        }
        let mut zoom = None;
        self.native_zoom(&mut zoom);
        // As the WebAssembly runtime clamped a mod's zoom.
        let z = zoom?;
        let factor = finite(z.factor, 1.0, 10.0);
        (factor > 1.0).then_some(NativeZoom {
            factor,
            smooth: z.smooth,
            turn: if z.turn.is_finite() {
                z.turn.clamp(0.05, 1.0)
            } else {
                1.0
            },
        })
    }
    /// The minimap a native mod shows.
    pub(super) fn native_minimap_now(&self) -> Option<NativeMinimap> {
        let mut minimap = None;
        self.native_minimap(&mut minimap);
        minimap.map(|m| NativeMinimap {
            size: finite(m.size, 16.0, 256.0),
            blocks_per_pixel: finite(m.blocks_per_pixel, 0.5, 4.0),
            ..m
        })
    }
}

/// Whether F3's debug screen, open when `debug`, covers `corner`
/// (`DebugScreenOverlay.extractRenderState`): its left and right columns
/// the top corners, and its charts the bottom corners `charts` (bottom left,
/// bottom right) says they are drawn in.
fn debug_takes(corner: Corner, debug: bool, charts: [bool; 2]) -> bool {
    debug
        && match corner {
            Corner::TopLeft | Corner::TopRight => true,
            Corner::BottomLeft => charts[0],
            Corner::BottomRight => charts[1],
        }
}

/// A menu of `MOD_MENUS`, by its name.
fn mod_menu(name: &str) -> Option<(&'static str, usize)> {
    MOD_MENUS.into_iter().find(|(menu, _)| *menu == name)
}

/// `v` within `lo..=hi`; `lo` when it is not a number.
fn finite(v: f32, lo: f32, hi: f32) -> f32 {
    if v.is_finite() { v.clamp(lo, hi) } else { lo }
}

#[cfg(test)]
mod tests {
    /// horizon-mixins refuses targets whose body has a `cfg` or a macro:
    /// the hook points and helpers must stay free of both. The launcher
    /// reads the API line as it is.
    /// A data packet for an old window leaves the open mod screen alone.
    #[test]
    fn mod_screen_data_is_its_windows_only() {
        let mut s = super::OpenNativeScreen {
            key: (0, 0, 0),
            window: 4,
            block: "gtmod:machine".into(),
            screen: super::NativeScreen::default(),
            data: Vec::new(),
        };
        assert!(s.set_data(4, 1, 7));
        assert!(!s.set_data(3, 1, 99));
        assert_eq!(s.data, [0, 7]);
    }

    #[test]
    fn mod_channels_are_namespaced_and_not_vanillas() {
        for ok in ["mekanism:sync", "my_mod:a/b-c.d"] {
            assert!(super::mod_channel(ok), "{ok}");
        }
        for bad in [
            "sync",
            "minecraft:brand",
            "horizon:quiet",
            "Mek:sync",
            ":x",
            "m:",
            "m:a b",
        ] {
            assert!(!super::mod_channel(bad), "{bad}");
        }
    }

    #[test]
    fn hook_points_stay_hookable() {
        let source = include_str!("native_hooks.rs");
        let code = source.split("#[cfg(test)]").next().unwrap_or_default();
        assert!(code.contains("\npub(crate) const NATIVE_HOOKS_API: u32 = "));
        let lines = code.lines().filter(|l| !l.trim_start().starts_with("//"));
        for line in lines {
            assert!(!line.contains("cfg"), "a cfg in the hook points: {line}");
            for mac in ["!(", "![", "! {", "!{"] {
                assert!(!line.contains(mac), "a macro in the hook points: {line}");
            }
        }
    }

    #[test]
    fn lifecycle_source_stays_hookable() {
        let source = include_str!("native_lifecycle.rs");
        let code = source.split("#[cfg(test)]").next().unwrap_or_default();
        for line in code.lines().filter(|l| !l.trim_start().starts_with("//")) {
            assert!(!line.contains("cfg"), "a cfg in lifecycle source: {line}");
            for mac in ["!(", "![", "! {", "!{"] {
                assert!(!line.contains(mac), "a macro in lifecycle source: {line}");
            }
        }
    }

    #[test]
    fn catalog_matches_hook_points() {
        use std::collections::BTreeSet;

        let catalog: serde_json::Value =
            serde_json::from_str(include_str!("../../../../native-hooks.json")).unwrap();
        let hooks = catalog["hooks"].as_array().unwrap();
        let source = include_str!("native_hooks.rs");
        let section = source
            .split("/// The hook points. Each does nothing in Horizon.")
            .nth(1)
            .unwrap()
            .split("/// Helpers for native mods")
            .next()
            .unwrap();
        let level_section = source
            .split("impl GameLevel<'_> {")
            .nth(1)
            .unwrap()
            .split("\n}")
            .next()
            .unwrap();
        let level_hooks: BTreeSet<_> = level_section
            .lines()
            .filter_map(|line| {
                line.trim()
                    .strip_prefix("pub(super) fn native_")?
                    .split('(')
                    .next()
            })
            .collect();
        let declared: BTreeSet<_> = section
            .lines()
            .chain(level_section.lines())
            .filter_map(|line| {
                line.trim()
                    .strip_prefix("pub(super) fn native_")?
                    .split('(')
                    .next()
            })
            .collect();
        let mut names = BTreeSet::new();
        let mut contracts = BTreeSet::new();
        for hook in hooks {
            let name = hook["name"].as_str().unwrap();
            let version = hook["version"].as_u64().unwrap();
            assert!(
                version > 0 && contracts.insert((name, version)),
                "invalid or duplicate hook: {name} v{version}"
            );
            names.insert(name);
            let owner = if level_hooks.contains(name) {
                "GameLevel"
            } else {
                "Game"
            };
            assert_eq!(
                hook["target"].as_str().unwrap(),
                format!("horizon_client::game::native_hooks::{owner}<'_>::native_{name}")
            );
        }
        assert_eq!(names, declared);
    }

    /// A mod container opens a slot grid or a furnace's menu, by its slot
    /// count; no menu with rules of its own (a brewing stand's, a crafter's).
    #[test]
    fn mod_menus_are_slot_grids_and_furnaces() {
        use super::mod_menu;
        assert_eq!(mod_menu("minecraft:hopper"), Some(("minecraft:hopper", 5)));
        assert_eq!(mod_menu("minecraft:generic_9x3").map(|m| m.1), Some(27));
        assert_eq!(mod_menu("minecraft:furnace").map(|m| m.1), Some(3));
        assert_eq!(mod_menu("minecraft:brewing_stand"), None);
        assert_eq!(mod_menu("hopper"), None);
    }

    /// Vanilla's F3 draws a column in each top corner (`extractLines`, left
    /// and right aligned), and its charts along the bottom.
    #[test]
    fn the_debug_screen_takes_both_top_corners_and_its_charts_corners() {
        use super::{Corner, debug_takes};
        for corner in Corner::ALL {
            assert!(!debug_takes(corner, false, [true, true]), "{corner:?}");
        }
        assert!(debug_takes(Corner::TopLeft, true, [false, false]));
        assert!(debug_takes(Corner::TopRight, true, [false, false]));
        assert!(!debug_takes(Corner::BottomLeft, true, [false, false]));
        assert!(!debug_takes(Corner::BottomRight, true, [false, false]));
        assert!(debug_takes(Corner::BottomLeft, true, [true, false]));
        assert!(!debug_takes(Corner::BottomRight, true, [true, false]));
        assert!(debug_takes(Corner::BottomRight, true, [false, true]));
    }
}

/// A [`NativeWorldBox`] with a sprite: each face in tiles of one block, the tile's part of the sprite on it.
fn sprite_box(
    b: &NativeWorldBox,
    rect: [f32; 4],
    camera: glam::DVec3,
    out: &mut Vec<horizon_client::mesher::Vertex>,
) {
    // Per face (east, west, up, down, south, north): the fixed axis and which end of it, the axes the sprite's
    // u and v run along, and whether v runs against the axis (the sprite's top is the face's top).
    const FACES: [(usize, bool, usize, usize, bool); 6] = [
        (0, true, 2, 1, true),
        (0, false, 2, 1, true),
        (1, true, 0, 2, false),
        (1, false, 0, 2, false),
        (2, true, 0, 1, true),
        (2, false, 0, 1, true),
    ];
    for (face, (axis, high, ua, va, flip_v)) in FACES.into_iter().enumerate() {
        let [a, r, g, bl] = b.colors[face].to_be_bytes();
        let color = u32::from(a) << 24 | u32::from(bl) << 16 | u32::from(g) << 8 | u32::from(r);
        let fixed = if high { b.max[axis] } else { b.min[axis] };
        let (mut u, u_end) = (b.min[ua], b.max[ua]);
        while u < u_end - 1e-9 {
            let u1 = (u.floor() + 1.0).min(u_end);
            let mut v = b.min[va];
            while v < b.max[va] - 1e-9 {
                let v1 = (v.floor() + 1.0).min(b.max[va]);
                let (ou, ov) = (u.floor(), v.floor());
                // The four corners: (u, v), (u, v1), (u1, v1), (u1, v), as seen from outside.
                let mut corners = [(u, v), (u, v1), (u1, v1), (u1, v)];
                if high != (axis != 1) {
                    corners.reverse();
                }
                for (cu, cv) in corners {
                    let mut p = [0.0; 3];
                    p[axis] = fixed;
                    p[ua] = cu;
                    p[va] = cv;
                    let tu = (cu - ou) as f32;
                    let tv = if flip_v {
                        1.0 - (cv - ov) as f32
                    } else {
                        (cv - ov) as f32
                    };
                    out.push(horizon_client::mesher::Vertex {
                        pos: std::array::from_fn(|i| (p[i] - camera[i]) as f32),
                        uv: [
                            rect[0] + (rect[2] - rect[0]) * tu,
                            rect[1] + (rect[3] - rect[1]) * tv,
                        ],
                        layer: horizon_client::gfx::TRANSLUCENT_ITEM_OVERLAY,
                        color,
                        light: 240 | 240 << 8,
                        normal: [0; 4],
                        glint_uv: [0.0, 0.0],
                    });
                }
                v = v1;
            }
            u = u1;
        }
    }
}
