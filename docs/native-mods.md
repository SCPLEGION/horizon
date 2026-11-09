# Native mods (experimental)

Native mods are **mixin bundles** that change the game's own Rust source before it is compiled, with
[horizon-mixins](https://github.com/horizon-rust/horizon-mixins) (MIT, by levzzz5154, made for Horizon). A bundle
is a `horizon-mixin.toml` and a few `.rs` helpers that hook into named functions of the game (HEAD, BEFORE_CALL,
RETURN, MODIFY_ARG, OVERWRITE...; the tool's README lists them). A port such as Veinminer is three HEAD hooks and
one module. Start a mod from [horizon-native-mod-template](https://github.com/horizon-rust/horizon-native-mod-template).

Native mods are native code with the player's rights, not sandboxed. The launcher turns them on only when the player
installs one (after a notice) and builds them only behind a red warning; the registry lists one only after a review (its later releases are checked by machine); and
they play in a game of their own, built for the instance they are installed in.

## Pinned to a Nightly

A native mod builds on one exact Horizon commit: `[compatibility] horizon_revision` in its `horizon-mixin.toml`,
and `horizon` in the `horizon-native.json` of its registry file (`.hnmod`, `horizon_mods::index::native`). Mods
pin a **Nightly's commit** (or a release's). Every Nightly ships its source and keeps its commit for good as the
tag `nightly-source/<date>-<short>` ([release-flow.md](release-flow.md#nightly-channel)), so a pinned mod still
builds after its Nightly is replaced. When the mods of one build pin different commits, the launcher says which
mod pins which and the commit the build would use, the newest of them, and builds only after the player chooses
**Build anyway** (`native-build --allow-version-mismatch`): such mods often still apply and compile. A failed build
then says the mods were made for other versions. Their horizon-mixins pins may differ too: the build uses the
newest of them and the launcher's own, never one older than a mod was made with. A bundle with no pin builds on
the commit of the installed version the build starts from.

### Mods on the hook points

A mod that hooks only Horizon's native hook points (`crates/horizon-client/src/game/native_hooks.rs`: empty
functions the game calls at fixed places, plus helpers, kept stable across releases) can name the version it
needs instead of pinning: `"hooks": 1` in its `.hnmod`'s `horizon-native.json`, the `NATIVE_HOOKS_API` of that
file when it was made. Hook points are only ever added: each addition raises `NATIVE_HOOKS_API`, and a mod builds
on any Horizon whose version is the one it needs or a later one. Its `horizon` is then only the commit it was tested on. The launcher builds it on the commit the other mods
pin, else on the installed version's commit (the game the player plays), after checking that commit's
`NATIVE_HOOKS_API` (read from GitHub) is at least the mod's; when it isn't and nothing pins, it builds on the commit the
mod was tested on. In its own copy of the mod it sets `horizon_revision` to the commit it builds on. Horizon's
own native mods (Zoom, FPS counter, Keystrokes, Minimap; horizon-rust/horizon-mods, `native/<name>/`) are made this way, so any of them build together
with no re-release for each Nightly.

API 2 adds the item panel beside container screens and its recipe pages (`native_panel_items`,
`native_panel_recipes`, and the hook `native_panel_event` for the player's clicks, searches and keys over items), and
the game's items and recipes (`native_recipes`, `native_known_recipes`, `native_place_recipe`, `native_give`): what a
recipe viewer needs.

API 11 adds `GameLevel::native_send_particles` on authoritative servers. It sends supported
vanilla particles to nearby players. Block particles require a valid block state. Item, trail and
vibration particles are not supported by the current dispatch. Count zero sends one directional
particle. The return value says whether the call was accepted, not whether any player received it. Bundles that use it need `"hooks": 11`.

API 3 adds authoritative world lifecycle events through `native_world_lifecycle`: open ready, open failed,
and final close. Bundles that use these events need `"hooks": 3`.

API 4 adds authoritative server tick events through `native_server_tick`: start (before any level ticks, like NeoForge's `ServerTickEvent.Pre`) and end of each non-paused
server tick, with `world_running`. A frozen world reports false except during a requested step. Bundles that
use these events need `"hooks": 4`.

API 8 adds server commands and players: `native_register_commands` (the commands a mod adds to every player's
command tree) and `native_perform_command` (a player's command on the server), with helpers to answer
([Server commands and players](#server-commands-and-players)). Bundles that use them need `"hooks": 8`.

API 9 adds the acting player's plain items for server mods (count, take, give; [Server commands and
players](#server-commands-and-players)). Bundles that use them need `"hooks": 9`.

API 10 adds server menus: a vanilla chest menu a server mod opens and fills, whose clicks and closing reach the mod
(`native_menu_clicked`, `native_menu_closed`), and `native_exchange_items`, two players' stacks swapped in one checked
step ([Server menus](#server-menus)). Bundles that use them need `"hooks": 10`.

API 12 adds item tooltips through `native_tooltip`: the hovered stack, with its components, and its tooltip's
lines, which a mod can add to, change or take out of, wherever the game shows an item's tooltip. Bundles that use it
need `"hooks": 12`.

API 13 adds using an item, on the server for every player: `native_use_item_on` hears a right click with an item on a
block once the block let it through (`ItemStack.useOn`), and `native_use_item` hears the item's own use, in the air or
after `native_use_item_on` let it through (`Item.use`). Each hears a click once. A mod that claims one swings the hand
instead of the item's use. Bundles that use them need `"hooks": 13`.

API 14 adds breaking and placing a block, on the server for every player: `native_destroy_block` hears a player break a
block once the game allowed it (`ServerPlayerGameMode.destroyBlock`) and may cancel it, which keeps the block and its
drops; `native_place_block` hears a block placed from an item (`BlockItem.place`). To stop a placement, a mod claims
`native_use_item_on`, which runs before it. Bundles that use them need `"hooks": 14`.

API 15 adds block entities for a mod's blocks (`native_block_entity`, `native_block_entity_ticks`,
`native_block_entity_tick`): made when the block is placed, ticked on the server and saved with the chunk. Bundles that use them need `"hooks": 15`.

API 16 adds item slots for those block entities (`native_block_entity_container`): hoppers, comparators and breaking the
block use them as they use vanilla's containers, and `native_block_entity_tick` also gets the slots. Bundles that use them need `"hooks": 16`.

API 17 adds menus for mod containers: the container names one of vanilla's slot-grid or furnace menus, which opens when the block is used,
and `native_block_entity_tick` takes the whole block entity, with the data slots the menu shows. Bundles that use them need `"hooks": 17`.

API 18 adds what a machine's tick reaches: `native_with_mod_block_entity` (change a neighbouring mod block entity, which marks its chunk
and tells comparators when its items changed) and `native_set_block_property` (a property of the block's own state, through the level's
normal update path). Bundles that use them need `"hooks": 18`.

API 19 adds screens a mod draws for its blocks' menus: `native_screen` lays one out (its height, where each container slot is,
where the inventory starts; a slot it leaves out is not in the menu, so shift-clicks skip it) and `native_screen_draw` draws it with the mod's
own GUI textures (`assets/<namespace>/textures/gui`, drawn with `native_sprite`). Bundles that use them need `"hooks": 19`.

API 20 adds buttons, tooltips and item icons on mod screens: `native_screen_click` and `native_screen_tooltip` (the client), `native_menu_button`
(a button press on the server, on the mod block entity) and the helpers `native_item`, `native_screen_title`, `native_data`, `native_block`,
`native_block_property`, `native_insert_item` and `native_cursor`. Bundles that use them need `"hooks": 20`.

API 21 adds what a server mod needs to guard land and move players. `native_right_click_block` hears a player's right
click on a block before the block's own use (a chest, a door, a lever) and the item's, and may cancel it: nothing
happens and the player sees the block as it is. `native_acting_player` says who acts inside a player's hook (UUID,
name, level, position, game mode, health); `native_run_command` runs a command as the server's console (level 4, no
output), so `execute in <dimension> run tp <player> <x> <y> <z>` moves a player across levels; `native_action_bar`
tells a player why, even inside their own hook; and the players
`native_server_players` lists carry their health. Bundles that use them need `"hooks": 21`.

API 22 adds mod networking: a server mod sends a packet on its own channel (`mymod:sync`, never `minecraft:` or
`horizon:`) to a player's client with `native_send_to_player(uuid, channel, data)` (up to 1 MiB), and that client's
`native_client_payload(channel, data)` hears it; a client sends one to the server with `native_send_to_server(channel,
data)` (up to 32,767 bytes), heard by `native_server_payload(player, channel, data)`. They travel as vanilla's custom
payload packets, so a client without the mod ignores them. The window's own client in single player gets them too. A
server mod never trusts what a client sends. Bundles that use them need `"hooks": 22`.

API 23 adds block-entity client sync. A mod block entity's client view is the `client` compound in its NBT: what a
machine shows in the world, never its whole state. The server sends it to each player with the chunk, again after the
block changes, and when the mod calls `native_sync_block_entity(pos)` after changing it (in the tick too; it goes out
at the end of the tick). A client reads it with `native_client_block_entity(pos)`, which gives the id and the view, and
hears each one in `native_client_block_entity_changed(pos, id, client)`. On a LAN host's own game the helper reads the
block entity itself. Keep the view small and change it only when what players see changes.

API 24 adds `native_extract_item(pos, face, max)`, the other half of `native_insert_item`: it takes up to `max` items of
one stack out of the container at `pos` through its side `face`, from the slots and under the rules a hopper there
would use (a furnace gives its output and empty buckets, a mod container its outputs). Item pipes use the pair.
Bundles that use it need `"hooks": 24`.

API 25 adds `native_destroy_speed(stack, block, hardness, speed)`: the speed the held stack mines a block at, where
`Item.getDestroySpeed` gives it, before enchantments and effects change it. A mod's tool sets `speed` (a powered drill
mines fast while it holds energy and slowly once empty). It runs on the client, which draws the crack, and on the
server, which times the break, so both must answer the same. Bundles that use it need `"hooks": 25`.

API 26 runs `native_server_payload` as the player who sent the packet: their state is swapped in, as for
`native_right_click_block`, so the hook can read and change what they wear and carry, and `native_acting_player` names
them (a jetpack's fuel goes down while its wearer's client says it thrusts). Bundles that rely on it need `"hooks": 26`.

API 27 adds `native_player_attack(target, strength, amount)`: a player's melee hit on a mob, with the damage
`Player.attack` worked out (attack damage, attack strength `strength` from 0 to 1, enchantments) before the mob takes
it. A mod's weapon changes `amount` (a powered sword adds damage and spends its energy). The hitting player's state is
swapped in. Server-side. Bundles that use it need `"hooks": 27`.

API 28 adds `native_player_damage(absorbed, share)`: a mod's armor takes the share `share` (0 to 1) of each hit on a
player before their own armor sees it, except damage tagged `minecraft:bypasses_armor`. `absorbed` is the damage it
took since the last call, for the mod to pay for (a powered suit spends energy on it); the mod sets `share` for the hits
to come. Called right after an absorbed hit and every 10 ticks otherwise, the player's state swapped in. Server-side.
Bundles that use it need `"hooks": 28`.

API 29 adds `native_extract_item_where(pos, face, max, wanted)`: `native_extract_item`, but it skips the stacks
`wanted` turns down and takes from the first one it accepts (a sorter that moves only its filters' items). Bundles
that use it need `"hooks": 29`.

API 30 adds entity access for blocks that act on what stands in front of them. `native_entities_in(min, max, from)` lists
the living mobs and dropped items whose boxes overlap a box, nearest to `from` first (players are not listed).
`native_hurt_mob(id, amount, fire_ticks)` hurts a mob with generic damage, sets it on fire, and returns the health it
lost. `native_take_item_entity(id)` picks a dropped item up. A mod's laser and tractor beam use them. Server-side.
Bundles that use them need `"hooks": 30`.

API 31 adds `native_sprite_tinted(batch, name, src, dst, argb)`: `native_sprite` multiplied by a colour, for a
mod's GUI tabs in their type's colour and gauges in their chemical's. Bundles that use it need `"hooks": 31`.

API 32 adds windows over a mod screen. `native_screen_overlay` draws after the screen's items, labels and the player's
inventory, for a window that covers slots (a side configuration or upgrade window). `native_screen_covers` says
which points that window covers: there the slots under it take no presses, do not light up and show no tooltips,
and a press goes to `native_screen_click` and stops there. API 32 also adds two drawing helpers:
`native_item_stack(batch, stack, at, size)`, which draws a stack's icon using its components (a
`custom_model_data` that picks the model), and `native_text_scaled(batch, text, at, rgb, scale)`. Client-side.
Bundles that use them need `"hooks": 32`.

API 33 adds text boxes on a mod screen (a frequency name, a filter's tag). While `native_screen_typing` says one of
the mod's boxes has focus, every key press but Escape goes to `native_screen_key(block, data, key, text, ctrl,
shift)` instead of the game, as vanilla's anvil rename box takes them: `key` is the key's name, `text` what it types
(the clipboard's text for the paste shortcut). The mod keeps the box's value itself, sends it with
`native_send_to_server`, and copies with the new `native_set_clipboard(text)` helper. Client-side. Bundles that use
them need `"hooks": 33`.

API 34 adds creative tabs for mods. `native_creative_tabs(tabs)` is asked each time the creative inventory opens: push
a `NativeCreativeTab` (`id`, `title`, an `icon` item, `items`) per module. Vanilla's ten ordinary tabs fill the first
page, a mod's follow, ten to a page, and `<` `>` buttons above the screen turn pages (as NeoForge's do; they show only
when a mod added tabs). The title is a language key or plain text; the stacks also join the search tab. Client-side.
Bundles that use it need `"hooks": 34`.

API 35 adds `native_mouse_down()`: whether the left mouse button is down. `native_screen_click` sees only the press, so a mod
that lets the player drag a window of its own (as Mekanism's GUI windows move) follows `native_cursor()` while it holds.
Bundles that use it need `"hooks": 35`.

API 36 adds a second layout for a mod screen. `NativeScreen::alt` holds another `NativeScreen` (another size, slots and
inventory position), and `native_screen_alt(block, alt)`, asked every frame, says whether it shows now: Mekanism's
frequency selector is a screen of its own size that its machines' screens swap to. Bundles that use it need
`"hooks": 36`.

API 37 adds `native_screen_escape(block, consumed)`: Escape on a mod screen first asks the mod, which sets `consumed` when it
closed a window of its own (Mekanism's machine windows); the screen then stays open. Bundles that use it need
`"hooks": 37`.

API 38 adds `native_slot_rules(block, rules)`, on the server: one `SlotRule` per container slot of a mod screen's menu
(`Any`, `Refuse` for an output slot, `Items(ids)` for a slot that takes only those items, such as an energy slot).
Bundles that use it need `"hooks": 38`.

API 39 adds `native_fill_quad(batch, corners, argb, additive)`: a four-cornered shape in one colour, added to what is behind it when `additive` (Mekanism's lightning in the
Antiprotonic Nucleosynthesizer's screen). Bundles that use it need `"hooks": 39`.

API 40 adds `native_screen_wheel(block, at, steps, taken)`: the mouse wheel over a mod screen goes to the mod first, which
sets `taken` when it scrolled a list of its own (Mekanism's filter lists). Bundles that use it need `"hooks": 40`.

API 41 adds `native_world_boxes(boxes)`: a mod pushes `NativeWorldBox { min, max, colors }` (world corners and an ARGB colour per face)
and the game draws each as a translucent box, full bright and seen from both sides, like translucent item quads
(Mekanism's Digital Miner visuals). Bundles that use it need `"hooks": 41`.

API 44 adds mod screens tied to no block: `native_open_ui(id)` (from a client hook such as `native_key`) opens the mod's screen `id` over the game and frees the cursor; the mod draws the whole window in `native_ui_draw(id, batch)` and hears `native_ui_click(id, at, button, pressed)`, `native_ui_key(id, key, text, pressed)` (every key but Escape, both edges), `native_ui_wheel(id, steps)` and `native_ui_closed(id)`. `native_close_ui()` closes it; Escape does too. Mekanism's module tweaker and radial selector use it.
API 45 adds `native_keep_chunks(chunks)`: a mod keeps chunks of the level in place loaded and ticking, like `/forceload`, for 40 ticks after each call (call it every tick from the block entity that loads them; they are let go when it stops), and they are not written to `forced_chunks.dat`. A chunk `/forceload` already holds stays the command's. Mekanism's dimensional stabilizer and anchor upgrade use it.
API 46 adds the hook `native_mob_types(types)`: a mod adds its own mob types when they load, as a clone of a vanilla `MobType` under another id (size, health, speed changed), and declares `horizon_client::mobs::register_mod_mob_base(id, base)`; guests and the window's client are sent `base`, so the mod's own client code draws the difference. `/summon` and saved entities find the type by its id. Also `native_mobs_tick()` (every tick after the mobs ticked), `native_mob_died(kind, at, data)`, `native_use_mob(id, claimed)` (a client's right click on a mob), `native_mob_ids(kind)`, `native_with_mob(id, f)` and `Extra::mod_data`, a compound saved with the mob as `mod_data`. Mekanism's Robit is built on them.

API 43 adds `native_block_redirect(pos, state, to)`: before a right click or a break, a mod sets `to` and the click or break acts on that
position instead (Mekanism's bounding blocks send a click on a filler cell to the machine that owns it). API 43 also adds `native_set_block(pos, block, properties)`: sets a block (`Level.setBlock` with `UPDATE_ALL`; `minecraft:air` removes), which a multi-cell
machine uses to fill and clear its other cells. API 43 also adds `native_block_removed(pos, old, old_properties, new)`: after any change of a block into another (a player, an explosion, a piston,
a command), with the block it was; unlike `native_block_changed` it runs once the change is done, so the mod may set blocks (a machine clearing
its other cells). API 43 also gives `NativeWorldBox` a `sprite` (a GUI-atlas sprite such as `block/water_still`, one tile per block, tinted by the face colours: a tank's fluid).
API 43 also adds `native_world_models(models)`: a mod pushes `NativeWorldModel { model, position, yaw, pitch, roll, light_at, color, rotation, full_bright, scale }` for a part tree
in `assets/<ns>/horizon/world_models/<name>.json` (pixels, y up) textured from `textures/entity/equipment/horizon_models/<name>.png`, and the game
draws it each frame (a wind generator's spinning rotor). Bundles that use any of these need `"hooks": 43`.

API 47 adds `native_world_items(items)` (`NativeWorldItem { item, matrix, light_at, seed, ground }`: an item in its GUI or ground form under a world-space matrix, a bin's
contents or a liquifier's food), `native_world_texts(texts)` (`NativeWorldText { text, matrix, x, y, color, light_at }`: a line of text in font pixels, y down, like a sign's) and
`native_face_sturdy(pos, face)` (`canOcclude` and a full sturdy face). Matrices are column major, in world space. Bundles that use them need `"hooks": 47`.

API 55 adds `SlotRule::Check(fn(&ItemStack) -> bool)` to the rules `native_slot_rules` returns: the slot takes only the stacks the function accepts, so a rule can test an item's components (Mekanism's hohlraum slot takes only a full hohlraum).

API 49 adds `native_gui_model(batch, model, texture, clip, origin, size, yaw)`: a mod's world model (`native_world_models`) drawn in a GUI as a picture-in-picture, its feet at `origin`, `size` pixels to the block, turned `yaw` degrees about its vertical axis, lit like an entity in a UI, wearing another atlas sprite as its skin (Mekanism's Robit skin preview). Call it from `native_ui_draw` or a block screen's draw.

API 50 adds a mod item's continuous use: `native_use_item` claims the click and calls `native_start_using`, then `native_using_tick(stack, ticks, claimed, keep)` runs each tick the use key is held (set `claimed`, and `keep` to go on). `native_clip(from, to)` finds the first block on a segment and `native_eye()` is the player's eyes (Mekanism's Flamethrower).

API 51 adds looping sounds: `native_loop_sound(key, event, at, volume, pitch)` starts (or keeps alive) a loop of a sound event, at a world position or heard everywhere with `at: None`; call it every tick or frame it should sound, and it stops 250 ms after the last call. `native_stop_loop_sound(key)` stops it at once (Mekanism's Industrial Alarm and Flamethrower).

API 60 adds `native_may_fly(may_fly)`: set it to let the player fly as in creative mode (double jump toggles flying) while in survival; asked whenever the game sets the player's abilities, on both sides (Mekanism's Gravitational Modulating Unit).

API 59 adds `native_shears_used(stack)`: the item that acts as shears was used to shear (where shears would take wear: only a real shearing), so the mod charges it, as Mekanism's Meka-Tool does in energy; it takes no wear.

API 58 adds `native_acts_as_shears(stack, shears)`: set `shears` when the stack in hand works as shears; the game then treats it as `minecraft:shears` when it shears mobs, hives, pumpkins and leads (Mekanism's Meka-Tool with a Shearing Unit).

API 57 adds `native_worn_tint(stack, tint)`: the colour (`0xAARRGGBB`) the opaque mod models worn on a piece of gear are drawn in (the glowing and translucent ones keep theirs), asked per worn stack every frame on the client (Mekanism's Color Modulation Unit).

API 56 adds `native_fog(fog_type, fog)`: the frame's fog after the game set it up (`fog_type` 0 atmospheric, 1 water, 2 lava, 3 powder snow), for the mod to change its colour and distances, every frame on the client (Mekanism's Vision Enhancement Unit).

API 54 adds `native_gear_attributes(attributes)` (push `(attribute id, modifier id, amount, operation)` for the worn gear, applied like an enchantment's modifiers), `native_glider(glider)` (set it to let the worn gear glide as an elytra) and `native_glide_wear(wear, claimed)` (the chest gear's wear while gliding, which `claimed` pays another way): Mekanism's MekaSuit Hydrostatic Repulsor, Gyroscopic Stabilization and Elytra units.

API 53 adds `native_player_damage_rules(absorbed, rules)`: pushed `(damage type id or #tag, share)` pairs are the share of such hits a mod's armor takes (1 is all of it), also for damage that bypasses armor; `absorbed` is the damage they took since the last call, for the mod to pay for (Mekanism's Inhalation Purification Unit).

API 52 adds `native_player_immunities(immune)`: pushed damage type ids (`minecraft:magic`) or `#`-tags are damage the player takes none of, asked every tick with the player swapped in (Mekanism's Gas Mask).


API 48 adds `native_mob_hidden(id, kind, hidden)` (the game leaves a mob undrawn because the mod draws it) and `native_mob_pose(id)` (its interpolated position, body yaw, head yaw and pitch); Mekanism's Robit is drawn so, as a part tree under `native_world_models`. API 48 also gives `NativeWorldModel` a `parts` list (the x, y and z rotations of parts by name) and lets a world model's JSON be `{"vanilla": "minecraft:chest#main", "offset": [x, y, z]}`: the part tree of a vanilla entity model under the mod's texture (Mekanism's Personal Chest lid).

API 47 also adds `native_world_quads(quads)` (`NativeWorldQuad { corners, color, sprite, uv }`: a flat full bright quad in world space, drawn from both sides, tinted, with a GUI-atlas sprite or the plain
colour: lightning bolts, billboarding effects) and `native_camera_axes()` (the camera's right and up directions, for a billboard). Bundles that use them need `"hooks": 47`.

API 42 adds `NativeScreen { armor, offhand }`: where the screen shows the player's four armor slots (head first, 18 px apart down) and the off hand slot
(`addArmorSlots`). They are the player's own worn slots; clicks go to the inventory menu (window 0) of a Horizon server. Bundles that use it need `"hooks": 42`.

## What the launcher does

In the window: install a native mod from the Mods page. The first one shows a notice once (the first build downloads
Rust and can take 20 to 30 minutes; later builds are faster) and turns on Settings → Game → **Native mods
(experimental)**, which stays off for players who never install one; `mods install` does the same. Then **Build**. The launcher names the mods, warns in red, and builds only from "I trust these mods: build"; the
row shows each step, the error if one fails, and the build the instance plays. From the command line:
`horizon-launcher native-build [FOLDER] [--instance NAME] --trust-native-code`. FOLDER is a bundle, an unpacked
`.hnmod`, or a folder of them; without it, the native mods turned on in the instance. It is not an MCP tool:
trusting native code is the player's decision, never an agent's.

1. **Rust.** The launcher's own toolchain ([`rust_toolchain`](../launcher/src/rust_toolchain.rs)): Rust 1.99.1's
   official `rustc`, `cargo` and `rust-std` (and `rust-mingw` on Windows), each pinned by SHA-256, downloaded once
   from static.rust-lang.org into `<launcher data>/rust/`, with its own Cargo home `<launcher data>/cargo/`. No
   rustup, nothing on the system; the player's own Rust is never used. Windows uses the `windows-gnu` host, which
   links without Visual Studio. `rust-mingw` only links, so on Windows the toolchain also brings MinGW-w64
   (`rust_toolchain::MINGW`: WinLibs' GCC 16.2.0 on MinGW-w64 14.0.0, MSVCRT like `rust-mingw`, one zip pinned by
   SHA-256, its `mingw64/` without the documentation) on the build's `PATH`: `ring` and `hidapi` compile C,
   and `windows-sys`' `raw-dylib` imports need `dlltool` and the assembler it runs. The linker is the system's on
   macOS (the Command Line Tools: the launcher asks for `xcode-select --install`) and Linux (`cc`).
   `horizon-launcher rust status|install|remove`.
2. **horizon-mixins**, at the commit the mods name (their `.hnmod`'s `mixins`), else `native_mods::MIXINS_REV`,
   built once with that cargo (`cargo install --locked --git … --rev …`) into `native/tools/<commit>/`.
3. **The source** of the pinned commit: GitHub's archive of it, once per commit (`native/source/<commit>/`, about
   63 MB unpacked), with the commit in `.horizon-revision`. Never the player's own checkout. Links in the
   archive are left out: the tool refuses them and nothing needs them to build.
4. **Mixins.** The bundles are copied first, so the build and its key use that copy whatever happens to the
   folder meanwhile. `horizon-mixins apply` stages the modded source; the launcher copies only the files that
   changed into `native/stage/`, so Cargo, which goes by file times, recompiles only the crates the mods touch.
5. **Compile**: `cargo build --locked --profile release --features horizon-client/mods -p horizon-client`, the
   release's own command (game-build.yml), into `native/target/`. Cargo downloads the locked crates from
   crates.io (Horizon has no git dependencies). The log is `native/build.log`; a failure shows its last lines (a
   horizon-mixins diagnostic in one line).
6. **Install**: a new version, `native-<commit>-<key>`, a copy of the starting version's folder (resources
   included, cloned on APFS, Btrfs and ReFS) with the built game in place of its own, and a `native-mods.json`
   saying which commit, mods, horizon-mixins and Rust made it. The instance the mods are installed in plays it (a
   folder built from the command line, or mods of the default home, go to the **Native mods** instance). That
   instance gets each mod's settings from the default home the first time it plays it (its
   `horizon-mods/.horizon-mod-data/<id>/` folder and `key.<id>.` lines of `horizon-mods.txt`, where earlier versions of
   the mod kept them); nothing it already has is overwritten.

## Install from GitHub

An author can ship a native mod without a registry pull request per version: the player installs it from a
GitHub link, on the Mods page ("Add from GitHub") or with `horizon-launcher mods add-github <url>
[--mod ID | --all] [--instance NAME] --trust-native-code`. The link is `https://github.com/<owner>/<repo>`
(its latest release), `.../releases/tag/<tag>`, `.../tree/<ref>` or `.../commit/<sha>`.

- **Resolved to something exact.** GitHub's public API (no sign-in; a rate limit is said plainly, and
  `HORIZON_GITHUB_TOKEN` raises it) turns the link into a release's `.hnmod` file or a **commit SHA**, never a
  branch that moves. A release with a `.hnmod` asset installs that file (its SHA-256 is compared with the one GitHub
  lists). With no release or no such asset, the launcher fetches
  `codeload.github.com/<owner>/<repo>/tar.gz/<sha>` and looks in it for mods by the rules of
  `native-build <folder>`: a folder with `horizon-mixin.toml`, or an unpacked `.hnmod` (`horizon-native.json`).
  Several mods: they are listed, and the player picks one or installs all. A source folder must pin the Horizon commit
  (`[compatibility] horizon_revision`); the launcher packs its Rust, TOML and `data/` files into a `.hnmod`.
  A release that ships only a content mod (`.hmod`) says "not supported yet": installing `.hmod` files from a link
  is not built.
- **Checked like the registry's files.** The `.hnmod` is read as `horizon_mods::index::native::Bundle::read` reads a
  registry file (only Rust, TOML and data under `mixins/<id>/`, size limits, nothing run) before it is installed.
- **Said first.** Before anything is downloaded: "not reviewed by the Horizon registry; native mods run with full
  access to this computer; read the source", with a link to the exact commit. The command needs
  `--trust-native-code`, as `native-build` does. It is not an MCP tool: trusting native code is the player's
  decision, never an agent's.
- **Origin kept.** `horizon-native-mods/<id>.origin.json` beside the `.hnmod`: owner/repo, release tag (for a
  release), commit, asset name and the SHA-256 of the installed file. `mods list` shows "from GitHub: owner/repo @
  tag" (or the commit's first 7 digits).
- **Updates.** `horizon-launcher mods update-github [ID] [--check]` compares a mod from a release with the
  repository's latest release, and a mod from a commit with the head of the default branch; a newer one is
  installed with the same warning and `--trust-native-code`. `mods remove <id>` removes it. A mod already
  installed from the registry (or by hand) is never replaced.

Installing turns Native mods on, as any native mod does; then build as usual.

## Cache and disk

The key hashes the commit, the horizon-mixins commit, Rust's version, the profile and features, and every file of
every bundle. The same set on the same commit is never built twice. A change gives a new build, and
`native/target/` stays, so a rebuild compiles only `horizon-client` and what the mods touched.

Measured 2026-10-28 on the Mac mini (M4 Pro, 14 cores) with Veinminer 0.3.0 on Nightly `83f15fa`,
`CARGO_BUILD_JOBS=8` while other agents' builds ran (upper bounds):

| What | Size, time |
|---|---|
| Rust 1.99.1 (`<launcher data>/rust/`) | 453 MB; 58 s to download and unpack |
| horizon-mixins (`native/tools/`) | 7 MB; built once in 19 s |
| Crates in the launcher's Cargo home (`<launcher data>/cargo/`) | 289 MB |
| Source of one commit, and the stage | 64 MB each |
| First build: release profile, the whole game | 9 min 10 s of compiling, 9 min 41 s in all; `native/target/` 1.5 GB |
| Rebuild after a helper's code changed | 2 min 46 s of compiling (only `horizon-client`), 2 min 53 s in all |
| A change horizon-mixins ignores (a comment) | nothing compiled; 31 s, mostly the version folder's copy |

Measured 2026-10-29 on Windows 11 (Ryzen 7 9800X3D, 16 threads), the same mod and Nightly, an empty `native/target/`
(horizon-mixins, the source and the crates were already downloaded):

| What | Size, time |
|---|---|
| Rust 1.99.1 and MinGW-w64 (`<launcher data>/rust/`) | 1.7 GB, MinGW-w64 875 MB of it |
| Crates in the launcher's Cargo home | 657 MB |
| First build: release profile, the whole game | 6 min 57 s of compiling, 8 min 3 s in all; `native/target/` 2.0 GB |

Measured 2026-10-28 on Linux (an x86_64 server with no Rust of its own), Zoom on Horizon `1149c92`, 3 jobs while CI
ran: `rust install` in 11 s (592 MB), the first build in 12 min 9 s. The built game ran Zoom's proof (under `xvfb-run`:
the game needs a display even with `--offscreen`).

Only the native builds an instance plays are kept (`native_mods::prune`); `native/target/` stays for the next one.

## Trust

- **Off until the player installs one.** Settings → Game → Native mods stays off for a player who never installs a
  native mod; the first install shows the notice and turns it on. The command needs `--trust-native-code`; the window
  shows a warning in red before every build: the mods run with full access to the computer, and their ids and
  versions. The Mods page says who made each one, and "Not made or endorsed by the Horizon project" for any mod not
  Horizon's own.
- **The registry** (horizon-rust/horizon-mods) lists them as their own kind, `native-mod`, only after a maintainer
  has read the source and the repository they come from. Its entry names the GitHub repository (`repo`); each
  release of it with a `.hnmod` asset is a version once the bundle passes the registry's checks, with no further
  review (nothing is built or run by the registry; the player's launcher builds it behind its warning). The `.hnmod` contains `.rs` and `.toml` files and may contain data files under
  `mixins/<id>/data/`. The launcher checks the bundle without running anything. Native code is trusted.
  The file is pinned by SHA-256 like every registry file, and the version, game and pins are the bundle's own
  `horizon-native.json`. `revoked.toml` takes a bad release down.
  A content mod's entry may turn `native-mod` once (never back): its `.hmod` versions stay published and
  are checked as `.hmod`s, the index lists only its `.hnmod` versions, and the launcher's update keeps the
  player's `.hmod`; removing the mod removes both.
- **Isolation of data.** Native mods run only in the instance built for them, with its own worlds, options and logs.
- What this does not do: sandbox the compiled game. horizon-mixins' checks stop mistakes, not a malicious helper
  ([security notes](https://github.com/horizon-rust/horizon-mixins/blob/main/docs/security.md)).

## Multiplayer

- **Single player and LAN host:** the mods run in the host's game and its integrated server, so a server-side
  hook applies to every player of the world.
- **Joining a server:** only client-side hooks do anything; the protocol is unchanged.
- **Dedicated servers:** native mods are not built yet. Lifecycle hook call sites are ready for when the pipeline builds them (`--no-default-features`).

## Left to do

- Rebuild after the mods or the Nightly change (asking first, as it compiles for minutes).
- A prebuilt horizon-mixins from its CI instead of building it on the player's computer.

## Named hook catalog (draft)

`native-hooks.json` maps stable hook names and contract versions to the current
source targets. A contract includes the signature, call timing and behavior.
Keep an existing contract or add a new version when it changes incompatibly.
The catalog does not cover helper methods or arbitrary private game code.
Named injection resolution needs a matching horizon-mixins change; existing raw
targets and the `hooks` metadata continue to work as before.

### Reading server blocks (draft)

A native server tick hook can call `native_server_block` with its context, a dimension id and a block position. It returns the block name and state properties from a loaded chunk. It does not load a chunk or open a dimension. Reads outside that hook, from a stale context, from unavailable dimensions or outside the world bounds return an error.

### Neighbour observations (draft)

`native_neighbor_changed` observes each authoritative server `neighborChanged` before the block handles it. It supplies the dimension, position, current state ID, changed block ID and moved flag. Resolve IDs with the level's registries during the hook; the hook does not load chunks or change the update queue.

### Server block changes (draft)

`native_block_changed` observes a committed authoritative server `Level.setBlock` change before block side effects and neighbour updates. It supplies the dimension, position and old and new state IDs. It does not observe chunk loading or direct storage writes that bypass `Level.setBlock`.

### Server commands and players

For mods that run where the world is simulated: a dedicated server, a LAN host, single player's server thread
(`crates/horizon-client/src/game/native_server.rs`). The SMP mod of the official server is built on them
([smp.md](smp.md)).

- **`native_register_commands(commands)`**: push a `NativeCommand` per usage, asked once when a world opens to
  players. A usage is words, then arguments `<name:kind>`, optional ones last in brackets:
  `NativeCommand::new("pay <player:player> <amount:integer>")`, `NativeCommand::op("eco set <player:player>
  <amount:integer>", 3)` for operators of level 3. Kinds: `word`, `text` (the rest of the line, last), `integer`,
  `number`, `boolean`, `player`, `item`, `block_pos`. The usages of one command merge into one tree, which every
  player receives after vanilla's (filtered by their level, as vanilla's is), so their chat box completes and colours
  it. A name vanilla has, or a usage that doesn't parse, is left out with a warning in the log.
- **`native_perform_command(player, command, ran)`**: a player typed a registered command (without its slash) and
  their level allows it, whether the world allows commands or not. The mod parses the line and sets `ran`. The
  player's state is swapped in for the call, as for their other actions, so `native_player` reads them; a
  registered command no mod takes answers as vanilla's unknown command.
- Helpers: `native_server_players()` (uuid, name, level, dimension, position and game mode of each player on);
  `native_tell(uuid, text)`, a system line to one player; `native_broadcast(text)`, to every player and the server's
  log; `native_feedback(text)`, the command's answer to the player whose command it is; `native_world_dir()`, the
  open world's folder, where a server mod keeps its data in a folder of its own so the world's backups carry it.
- Items (API 9), of the acting player (inside `native_perform_command`, whose player is swapped in): `native_held()`
  (the main hand's item, count, and whether it is plain), `native_count_items(item)`, `native_carried_items()`,
  `native_take_items(item, count)` (the main hand first, then as `Inventory.add` orders slots),
  `native_room_for(item)` and `native_give_items(item, count)` (only where it fits: nothing is dropped). They look at
  the 36 inventory slots and the off hand, never the armour, and only at **plain** stacks: no damage and no
  component (name, enchantment...), so a shop never takes a worn or enchanted item for a new one. Each returns how
  many it counted, took or gave; the player's screen follows as for any change to their inventory.
- API 21: `native_acting_player()`, the player whose state is swapped in (inside `native_perform_command`,
  `native_right_click_block`, `native_use_item_on`, `native_destroy_block`...; None elsewhere);
  `native_run_command(line)`, a command run as the server's console (level 4, no output), only where no player is
  swapped in (`native_tick`, a menu hook): a teleport to another level (`execute in minecraft:the_nether run tp Alex 0
  70 0`) lands at the next tick's start. `native_server_players()` and the acting player carry `health` (0 once dead).
  `native_action_bar(uuid, text)`, a line over a player's hotbar, which also reaches the acting player inside their
  hook (where `native_tell` reaches nobody).
- **`native_right_click_block(pos, face, canceled)`** (API 21): a player's right click on a block, before the block's
  own use and the item's (`ServerPlayerGameMode.useItemOn`), once per hand. A mod that sets `canceled` stops both,
  without a swing; the player's client is sent the block back. With `native_destroy_block` it keeps players out of
  claimed land.

### Server menus

API 10, `crates/horizon-client/src/game/native_menus.rs`: menus a server mod builds from ordinary items, which a
stock client shows as a chest (`generic_9x1` to `generic_9x6`); nothing is installed on the player's side.

- `native_open_menu(uuid, rows, title, items)` opens one in place of the player's open menu (its window id);
  `native_set_menu_items(uuid, items)` changes what it shows; `native_close_menu(uuid)` closes it.
- **No click moves anything.** Each reaches `native_menu_clicked(uuid, window, slot, button, mode)`, and the client
  is sent the menu, its inventory and its cursor as they were. `slot` counts the menu's own slots first, then the
  player's inventory below it: `native_menu_inventory_slot(size, slot)` gives the inventory slot (0..9 the hotbar).
- `native_menu_closed(uuid, window)`: the player closed it, another menu opened, or the player left.
- `native_player_stacks(uuid)`: the player's 36 inventory slots, to show copies or offer them.
- `native_exchange_items(a, a_offer, b, b_offer)`: each offer is inventory slots, each with the exact stack it must
  still hold. Both offers leave their slots and go to the other player where they fit whole (onto matching stacks,
  then empty slots), or nothing moves and the error says why (an offer that changed, no room, a player gone or
  dead). A trade on these menus leaves the offered stacks in their owners' inventories until this one step, so
  closing the menu, leaving or dying moves nothing.

The menu hooks run with no player's state swapped in; `native_player_stacks` and `native_exchange_items` refuse a
player whose state is (inside `native_perform_command`): call them from a menu hook or `native_server_tick`.
