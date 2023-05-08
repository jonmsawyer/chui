/*!
Layout jobs for the Design menu

A layout job is the custom way to format [`RichText`].
*/

use super::{top_menu_text_format, top_menu_text_format_underline};
use bevy_egui::egui::text::LayoutJob;

/// "Design"
pub fn top_menu_design() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("D", 0.0, top_menu_text_format_underline());
    job.append("esign", 0.0, top_menu_text_format());

    job
}

/// "Default"
pub fn top_menu_default() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("D", 0.0, top_menu_text_format_underline());
    job.append("efault", 0.0, top_menu_text_format());

    job
}

/// "Change Design"
pub fn top_menu_change_design() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("hange Design", 0.0, top_menu_text_format());

    job
}

/// "Load Design"
pub fn top_menu_load_design() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("L", 0.0, top_menu_text_format_underline());
    job.append("oad Design", 0.0, top_menu_text_format());

    job
}

/// "Save Design"
pub fn top_menu_save_design() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("S", 0.0, top_menu_text_format_underline());
    job.append("ave Design", 0.0, top_menu_text_format());

    job
}

/// "Background..."
pub fn top_menu_background() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("B", 0.0, top_menu_text_format_underline());
    job.append("ackground...", 0.0, top_menu_text_format());
    job
}

/// "Colors..."
pub fn top_menu_colors() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("olors", 0.0, top_menu_text_format());

    job
}

/// "Window Title Bars"
pub fn top_menu_window_title_bars() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("W", 0.0, top_menu_text_format_underline());
    job.append("indow Title Bars", 0.0, top_menu_text_format());

    job
}
