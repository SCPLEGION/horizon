//! Mods' overlay on container screens (API 87): `native_container_draw` after the screen, and the clicks, keys and
//! wheel it may claim first. The glue between those hooks and the screens; the hooks are in `native_hooks.rs`.
use super::{Game, Screen};
use horizon_client::{hud::HudBatch, keybinds};
use winit::{event::KeyEvent, keyboard::PhysicalKey};

impl Game<'_> {
    /// The container screen's window as the hooks give it.
    fn container_window(&self, batch: &HudBatch) -> [f32; 4] {
        let image = self.container_image(batch);
        [
            image.left as f32,
            image.top as f32,
            image.width as f32,
            image.height as f32,
        ]
    }

    pub(super) fn draw_native_container(&self, batch: &mut HudBatch) {
        let window = self.container_window(batch);
        self.native_container_draw(window, batch);
    }

    /// A mouse press on a container screen: true when a mod claimed it.
    pub(super) fn native_container_press(&mut self, right: bool) -> bool {
        let Some(r) = self.renderer.as_ref() else {
            return false;
        };
        let batch = HudBatch::new([r.config.width, r.config.height]);
        let window = self.container_window(&batch);
        let at = [self.cursor.0, self.cursor.1];
        let mut claimed = false;
        self.native_container_click(window, at, u8::from(right), &mut claimed);
        claimed
    }

    /// A mouse release on a container screen: true when a mod claimed it.
    pub(super) fn native_container_let_go(&mut self, right: bool) -> bool {
        let Some(r) = self.renderer.as_ref() else {
            return false;
        };
        let batch = HudBatch::new([r.config.width, r.config.height]);
        let window = self.container_window(&batch);
        let at = [self.cursor.0, self.cursor.1];
        let mut claimed = false;
        self.native_container_release(window, at, u8::from(right), &mut claimed);
        claimed
    }

    /// A key on a container screen (Escape excluded by the caller): true when a mod claimed it.
    pub(super) fn native_container_key_event(&mut self, event: &KeyEvent) -> bool {
        use horizon_client::clipboard::{Clipboard, EditShortcut};
        if self.screen != Screen::Inventory {
            return false;
        }
        let PhysicalKey::Code(code) = event.physical_key else {
            return false;
        };
        let Some(name) = keybinds::key_name(code) else {
            return false;
        };
        let pressed = event.state == winit::event::ElementState::Pressed;
        let text = if pressed {
            match self.edit_shortcut(event) {
                Some(EditShortcut::Paste) => Some(self.clipboard.get()),
                Some(_) => None,
                None => event
                    .text
                    .as_deref()
                    .map(|t| t.chars().filter(|c| !c.is_control()).collect::<String>())
                    .filter(|t| !t.is_empty()),
            }
        } else {
            None
        };
        let mut claimed = false;
        self.native_container_key(name, text.as_deref(), pressed, &mut claimed);
        claimed
    }

    /// The wheel on a container screen: true when a mod scrolled something.
    pub(super) fn native_container_wheel_event(&mut self, steps: f32) -> bool {
        let at = [self.cursor.0, self.cursor.1];
        let mut claimed = false;
        self.native_container_wheel(at, steps, &mut claimed);
        claimed
    }
}
