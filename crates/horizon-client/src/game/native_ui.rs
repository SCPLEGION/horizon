//! A native mod's own screens (`Screen::ModUi`), tied to no block or menu: the mod draws the whole
//! window and hears the mouse, the keys and the wheel (Mekanism's module tweaker and radial
//! selector). API 44.
use super::{Game, Screen};
use horizon_client::keybinds;
use horizon_client::hud::HudBatch;
use winit::event::{KeyEvent, MouseButton};

#[allow(dead_code)]
impl Game<'_> {
    /// Opens the mod's screen `id` over the game, freeing the cursor. A mod calls this from a hook
    /// that runs on the client (`native_key`...); nothing happens while another screen is open. API 44.
    pub(super) fn native_open_ui(&mut self, id: &str) {
        if self.screen != Screen::Playing || self.player.is_none() {
            return;
        }
        self.mod_ui = Some(id.to_owned());
        self.screen = Screen::ModUi;
        self.capture(false);
    }
    /// Closes the mod's open screen, if any. API 44.
    pub(super) fn native_close_ui(&mut self) {
        if self.screen == Screen::ModUi {
            self.close_native_ui();
        }
    }
    /// The id of the mod screen that is open. API 44.
    pub(super) fn native_ui_open(&self) -> Option<&str> {
        self.mod_ui.as_deref().filter(|_| self.screen == Screen::ModUi)
    }
    pub(super) fn close_native_ui(&mut self) {
        let Some(id) = self.mod_ui.take() else {
            return;
        };
        self.screen = Screen::Playing;
        self.capture(true);
        self.native_ui_closed(&id);
    }
    pub(super) fn draw_native_ui(&self, batch: &mut HudBatch) {
        if let Some(id) = &self.mod_ui {
            self.native_ui_draw(id, batch);
        }
    }
    pub(super) fn native_ui_mouse(&mut self, button: MouseButton, pressed: bool) {
        let Some(id) = self.mod_ui.clone() else {
            return;
        };
        let code = match button {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            _ => 2,
        };
        let at = self.native_cursor();
        self.native_ui_click(&id, at, code, pressed);
    }
    pub(super) fn native_ui_wheel_event(&mut self, steps: f32) {
        if let Some(id) = self.mod_ui.clone() {
            self.native_ui_wheel(&id, steps);
        }
    }
    pub(super) fn native_ui_key_event(&mut self, event: &KeyEvent) {
        use horizon_client::clipboard::{Clipboard, EditShortcut};
        let winit::keyboard::PhysicalKey::Code(code) = event.physical_key else {
            return;
        };
        let (Some(name), Some(id)) = (keybinds::key_name(code), self.mod_ui.clone()) else {
            return;
        };
        let pressed = event.state == winit::event::ElementState::Pressed;
        let text = if pressed {
            match self.edit_shortcut(event) {
                Some(EditShortcut::Paste) => Some(self.clipboard.get()),
                Some(_) => None,
                None if self.types_text() => event
                    .text
                    .as_deref()
                    .map(|t| t.chars().filter(|c| !c.is_control()).collect::<String>())
                    .filter(|t| !t.is_empty()),
                None => None,
            }
        } else {
            None
        };
        self.native_ui_key(&id, name, text.as_deref(), pressed);
    }
}
