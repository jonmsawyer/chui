/*!
Layout jobs for the File menu

A layout job is the custom way to format [`RichText`].
*/

use super::{top_menu_text_format, top_menu_text_format_underline};
use bevy_egui::egui::text::LayoutJob;

/// "File"
pub fn top_menu_file() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("F", 0.0, top_menu_text_format_underline());
    job.append("ile", 0.0, top_menu_text_format());

    job
}

/// "New Game..."
pub fn top_menu_new_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("N", 0.0, top_menu_text_format_underline());
    job.append("ew Game...", 0.0, top_menu_text_format());

    job
}

/// "New Chess960 Game..."
pub fn top_menu_new_chess960_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("N", 0.0, top_menu_text_format());
    job.append("e", 0.0, top_menu_text_format_underline());
    job.append("w Chess960 Game...", 0.0, top_menu_text_format());

    job
}

/// "Load Game..."
pub fn top_menu_load_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("L", 0.0, top_menu_text_format_underline());
    job.append("oad Game...", 0.0, top_menu_text_format());

    job
}

/// "Save Game..."
pub fn top_menu_save_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("S", 0.0, top_menu_text_format_underline());
    job.append("ave Game...", 0.0, top_menu_text_format());

    job
}

/// "Delete Game..."
pub fn top_menu_delete_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("D", 0.0, top_menu_text_format_underline());
    job.append("elete Game...", 0.0, top_menu_text_format());

    job
}

/// "Load Position..."
pub fn top_menu_load_position() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("L", 0.0, top_menu_text_format());
    job.append("o", 0.0, top_menu_text_format_underline());
    job.append("ad Position...", 0.0, top_menu_text_format());

    job
}

/// "Save Position..."
pub fn top_menu_save_position() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("S", 0.0, top_menu_text_format());
    job.append("a", 0.0, top_menu_text_format_underline());
    job.append("ve Position...", 0.0, top_menu_text_format());

    job
}

/// "Recent Files..."
pub fn top_menu_recent_files() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("R", 0.0, top_menu_text_format_underline());
    job.append("ecent Files...", 0.0, top_menu_text_format());

    job
}

/// "Edit Position..."
pub fn top_menu_edit_position() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("dit Position...", 0.0, top_menu_text_format());

    job
}

/// "Print Game..."
pub fn top_menu_print_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("P", 0.0, top_menu_text_format_underline());
    job.append("rint Game...", 0.0, top_menu_text_format());

    job
}

/// "Exit"
pub fn top_menu_exit() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format());
    job.append("x", 0.0, top_menu_text_format_underline());
    job.append("it", 0.0, top_menu_text_format());

    job
}
