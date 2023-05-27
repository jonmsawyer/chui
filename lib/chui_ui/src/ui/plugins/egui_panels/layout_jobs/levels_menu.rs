/*!
Layout jobs for the Levels menu

A layout job is the custom way to format [`RichText`].
*/

use super::{top_menu_text_format, top_menu_text_format_underline};
use bevy_egui::egui::text::LayoutJob;

/// "Levels"
pub fn top_menu_levels() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("L", 0.0, top_menu_text_format_underline());
    job.append("evels", 0.0, top_menu_text_format());

    job
}

/// "Playing Strength..."
pub fn top_menu_playing_strength() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("P", 0.0, top_menu_text_format_underline());
    job.append("laying Strength", 0.0, top_menu_text_format());

    job
}

/// "Blitz..."
pub fn top_menu_blitz() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("B", 0.0, top_menu_text_format());
    job.append("l", 0.0, top_menu_text_format_underline());
    job.append("itz...", 0.0, top_menu_text_format());

    job
}

/// "Time Per Move..."
pub fn top_menu_time_per_move() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("T", 0.0, top_menu_text_format_underline());
    job.append("ime Per Move...", 0.0, top_menu_text_format());

    job
}

/// "Time Controls..."
pub fn top_menu_time_controls() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Time ", 0.0, top_menu_text_format());
    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("ontrols...", 0.0, top_menu_text_format());

    job
}

/// "Fixed Search Depth..."
pub fn top_menu_fixed_search_depth() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("F", 0.0, top_menu_text_format_underline());
    job.append("ixed Search Depth...", 0.0, top_menu_text_format());

    job
}
