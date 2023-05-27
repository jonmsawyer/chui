/*!
Layout jobs for the Opening Book menu

A layout job is the custom way to format [`RichText`].
*/

use super::{top_menu_text_format, top_menu_text_format_underline};
use bevy_egui::egui::text::LayoutJob;

/// "Opening Book"
pub fn top_menu_opening_book() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("O", 0.0, top_menu_text_format_underline());
    job.append("pening Book", 0.0, top_menu_text_format());

    job
}

/// "New Opening Book..."
pub fn top_menu_new_opening_book() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("N", 0.0, top_menu_text_format_underline());
    job.append("ew Opening Book...", 0.0, top_menu_text_format());

    job
}

/// "Load Opening Book..."
pub fn top_menu_load_opening_book() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("L", 0.0, top_menu_text_format_underline());
    job.append("oad Opening Book...", 0.0, top_menu_text_format());

    job
}

/// "Import Opening Book..."
pub fn top_menu_import_opening_book() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("I", 0.0, top_menu_text_format_underline());
    job.append("mport Opening Book...", 0.0, top_menu_text_format());

    job
}

/// "Import Games..."
pub fn top_menu_import_games() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Import ", 0.0, top_menu_text_format());
    job.append("G", 0.0, top_menu_text_format_underline());
    job.append("ames", 0.0, top_menu_text_format());

    job
}

/// "Import Current Game..."
pub fn top_menu_import_current_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Import ", 0.0, top_menu_text_format());
    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("urrent Game...", 0.0, top_menu_text_format());
    job
}

/// "Export Opening Book..."
pub fn top_menu_export_opening_book() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("xport Opening Book", 0.0, top_menu_text_format());

    job
}

/// "Count Book Moves"
pub fn top_menu_count_book_moves() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("ount Book Moves", 0.0, top_menu_text_format());

    job
}

/// "Weed Opening Book..."
pub fn top_menu_weed_opening_book() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("W", 0.0, top_menu_text_format_underline());
    job.append("eed Opening Book...", 0.0, top_menu_text_format());

    job
}
