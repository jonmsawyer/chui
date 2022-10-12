/*!
Layout jobs for the UI

A layout job is the custom way to format [`RichText`].
*/

use bevy_egui::egui::{Color32, Stroke, FontFamily, FontId, text::TextFormat};

mod file_menu;
pub use file_menu::*;

mod commands_menu;
pub use commands_menu::*;

mod levels_menu;
pub use levels_menu::*;

mod mode_menu;
pub use mode_menu::*;

mod opening_book_menu;
pub use opening_book_menu::*;

mod extras_menu;
pub use extras_menu::*;

mod windows_menu;
pub use windows_menu::*;

mod design_menu;
pub use design_menu::*;

const MENU_FONT_SIZE: f32 = 14.0; // 14.0 pt
const MENU_COLOR: Color32 = Color32::GRAY;

/// top_menu_font()
fn top_menu_font() -> FontId {
    FontId::new(MENU_FONT_SIZE, FontFamily::Proportional)
}

/// top_menu_underline()
fn top_menu_underline() -> Stroke {
    Stroke {
        width: 1.0,
        color: MENU_COLOR,
    }
}

/// top_menu_text_format()
fn top_menu_text_format() -> TextFormat {
    TextFormat {
        font_id: top_menu_font(),
        color: MENU_COLOR,
        ..Default::default()
    }
}

/// top_menu_text_format_underline()
fn top_menu_text_format_underline() -> TextFormat {
    TextFormat {
        font_id: top_menu_font(),
        color: MENU_COLOR,
        underline: top_menu_underline(),
        ..Default::default()
    }
}

