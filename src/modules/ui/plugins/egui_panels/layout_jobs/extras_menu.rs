/*!
Layout jobs for the Extras menu

A layout job is the custom way to format [`RichText`].
*/

use super::{top_menu_text_format, top_menu_text_format_underline};
use bevy_egui::egui::text::LayoutJob;

/// top_menu_extras()
/// "Extras"
pub fn top_menu_extras() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("xtras", 0.0, top_menu_text_format());

    job
}

/// top_menu_engines()
/// "Engines"
pub fn top_menu_engines() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("ngines", 0.0, top_menu_text_format());

    job
}

/// top_menu_engine_options()
/// "Engine Options..."
pub fn top_menu_engine_options() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Engine ", 0.0, top_menu_text_format());
    job.append("O", 0.0, top_menu_text_format_underline());
    job.append("ptions", 0.0, top_menu_text_format());

    job
}

/// top_menu_hash_tables()
/// "Hash Tables..."
pub fn top_menu_hash_tables() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("H", 0.0, top_menu_text_format_underline());
    job.append("ash Tables...", 0.0, top_menu_text_format());
    job
}

/// top_menu_permanent_brain()
/// "Permanent Brain"
pub fn top_menu_permanent_brain() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("P", 0.0, top_menu_text_format_underline());
    job.append("ermanent Brain", 0.0, top_menu_text_format());

    job
}

/// top_menu_install_engine()
/// "Install Engine..."
pub fn top_menu_install_engine() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("I", 0.0, top_menu_text_format_underline());
    job.append("nstall Engine...", 0.0, top_menu_text_format());

    job
}

/// top_menu_uninstall_engine()
/// "Uninstall Engine..."
pub fn top_menu_uninstall_engine() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("U", 0.0, top_menu_text_format_underline());
    job.append("ninstall Engine...", 0.0, top_menu_text_format());

    job
}

/// top_menu_edit_engine_file()
/// "Edit Engine File..."
pub fn top_menu_edit_engine_file() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format());
    job.append("d", 0.0, top_menu_text_format_underline());
    job.append("it Engine File...", 0.0, top_menu_text_format());

    job
}

/// top_menu_endgame_databases()
/// "Endgame Databases..."
pub fn top_menu_endgame_databases() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Endgame ", 0.0, top_menu_text_format());
    job.append("D", 0.0, top_menu_text_format_underline());
    job.append("atabases...", 0.0, top_menu_text_format());

    job
}

/// top_menu_query_online_database()
/// "Query Online Database"
pub fn top_menu_query_online_database() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Q", 0.0, top_menu_text_format_underline());
    job.append("uery Online Database", 0.0, top_menu_text_format());

    job
}

/// top_menu_publish_game()
/// "Publish Game..."
pub fn top_menu_publish_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("P", 0.0, top_menu_text_format_underline());
    job.append("ublish Game...", 0.0, top_menu_text_format());

    job
}

/// top_menu_endgame_oracle()
/// "Endgame Oracle..."
pub fn top_menu_endgame_oracle() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("ndgame Oracle...", 0.0, top_menu_text_format());

    job
}

/// top_menu_my_results()
/// "My Results..."
pub fn top_menu_my_results() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("M", 0.0, top_menu_text_format_underline());
    job.append("y Results...", 0.0, top_menu_text_format());

    job
}

/// top_menu_options()
/// "Options..."
pub fn top_menu_options() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("O", 0.0, top_menu_text_format_underline());
    job.append("ptions...", 0.0, top_menu_text_format());

    job
}
