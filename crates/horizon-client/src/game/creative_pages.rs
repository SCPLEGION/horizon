//! The creative screen's tabs from mods (`native_creative_tabs`, API 34) and
//! the pages they need: vanilla's ten ordinary tabs fill the first page, a
//! mod's go on the next, ten to a page, with `<` and `>` buttons above the
//! screen as NeoForge has them.
use super::{Game, native_hooks::NativeCreativeTab, native_panel::browser_stack};
use horizon_client::hud::HudBatch;
use horizon_client::{creative::ModTab, text::Text, ui};

/// A page button's size, and its place above the screen's panel.
const BUTTON: f32 = 20.0;
const BUTTON_Y: f32 = -50.0;

impl Game<'_> {
    /// Asks the mods for their tabs (opening the screen).
    pub(super) fn load_mod_creative_tabs(&mut self) {
        let mut asked = Vec::new();
        self.native_creative_tabs(&mut asked);
        let items = self.res.items();
        let tabs = asked
            .into_iter()
            .take(40)
            .filter_map(|t: NativeCreativeTab| {
                let stacks: Vec<_> = t
                    .items
                    .iter()
                    .filter_map(|s| browser_stack(items, s))
                    .collect();
                (!stacks.is_empty()).then_some(ModTab {
                    id: t.id,
                    title: t.title,
                    icon: t.icon,
                    items: stacks,
                })
            })
            .collect();
        self.creative_screen.set_mod_tabs(tabs);
    }

    /// The `<` and `>` buttons, when there is more than one page.
    fn creative_page_buttons(&self, ox: f32, oy: f32) -> Option<[[f32; 4]; 2]> {
        (self.creative_screen.page_count() > 1).then(|| {
            let w = horizon_client::creative::Creative::WIDTH;
            [
                [ox, oy + BUTTON_Y, BUTTON, BUTTON],
                [ox + w - BUTTON, oy + BUTTON_Y, BUTTON, BUTTON],
            ]
        })
    }

    pub(super) fn draw_creative_pages(&self, batch: &mut HudBatch, ox: f32, oy: f32) {
        let Some(buttons) = self.creative_page_buttons(ox, oy) else {
            return;
        };
        let atlas = &self.gui.atlas;
        for (b, label) in buttons.iter().zip(["<", ">"]) {
            let hover = ui::hovered(self.cursor, b[0], b[1], b[2], b[3]);
            ui::button(
                batch,
                atlas,
                b[0],
                b[1],
                b[2],
                b[3],
                &Text::literal(label),
                true,
                hover,
            );
        }
        let label = format!(
            "{} / {}",
            self.creative_screen.page + 1,
            self.creative_screen.page_count()
        );
        let w = horizon_client::creative::Creative::WIDTH;
        let tw = HudBatch::text_width(atlas, &label);
        batch.text(
            atlas,
            &label,
            ox + (w - tw) / 2.0,
            oy + BUTTON_Y + 6.0,
            u32::MAX,
            true,
        );
    }

    /// A click on a page button; true when it was one.
    pub(super) fn click_creative_pages(&mut self, ox: f32, oy: f32) -> bool {
        let Some(buttons) = self.creative_page_buttons(ox, oy) else {
            return false;
        };
        let (cx, cy) = self.cursor;
        for (n, b) in buttons.iter().enumerate() {
            if ui::hovered((cx, cy), b[0], b[1], b[2], b[3]) {
                let page = self.creative_screen.page as isize + if n == 0 { -1 } else { 1 };
                self.creative_screen.set_page(page);
                self.inv_clicks = Default::default();
                self.inv_clicks.skip_release();
                self.inv_drag = None;
                self.last_inv_click = None;
                self.audio.play_button_click();
                return true;
            }
        }
        false
    }
}
