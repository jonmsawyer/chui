/*!
Layout jobs for the Windows menu

A layout job is the custom way to format [`RichText`].
*/

use super::{top_menu_text_format, top_menu_text_format_underline};
use bevy_egui::egui::text::LayoutJob;

/// "Windows"
pub fn top_menu_windows() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("W", 0.0, top_menu_text_format_underline());
    job.append("indows", 0.0, top_menu_text_format());

    job
}

/// "Board"
pub fn top_menu_board() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("B", 0.0, top_menu_text_format_underline());
    job.append("oard", 0.0, top_menu_text_format());

    job
}

/// "Clock"
pub fn top_menu_clock() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("lock", 0.0, top_menu_text_format());

    job
}

/// "Notation"
pub fn top_menu_notation() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("N", 0.0, top_menu_text_format_underline());
    job.append("otation", 0.0, top_menu_text_format());

    job
}

/// "Engine"
pub fn top_menu_engine() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("ngine", 0.0, top_menu_text_format());

    job
}

/// "Moves"
pub fn top_menu_moves() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("M", 0.0, top_menu_text_format_underline());
    job.append("oves", 0.0, top_menu_text_format());
    job
}

/// "Histogram"
pub fn top_menu_histogram() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("H", 0.0, top_menu_text_format_underline());
    job.append("istogram", 0.0, top_menu_text_format());

    job
}
