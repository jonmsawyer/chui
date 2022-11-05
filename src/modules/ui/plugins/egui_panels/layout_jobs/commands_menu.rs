/*!
Layout jobs for the File menu

A layout job is the custom way to format [`RichText`].
*/

use super::{top_menu_text_format, top_menu_text_format_underline};
use bevy_egui::egui::text::LayoutJob;

/// top_menu_file()
/// "File"
pub fn top_menu_commands() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("ommands", 0.0, top_menu_text_format());

    job
}

/// top_menu_new_game()
/// "New Game..."
pub fn top_menu_compute() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("ompute / Switch Sides", 0.0, top_menu_text_format());

    job
}

/// top_menu_interrupt()
/// "Interrupt"
pub fn top_menu_interrupt() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Interrupt", 0.0, top_menu_text_format());

    job
}

/// top_menu_copy()
/// "Copy"
pub fn top_menu_copy() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("opy", 0.0, top_menu_text_format());

    job
}

/// top_menu_copy_pgn()
/// "Game (PGN)"
pub fn top_menu_copy_pgn() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Game (", 0.0, top_menu_text_format());
    job.append("P", 0.0, top_menu_text_format_underline());
    job.append("GN)", 0.0, top_menu_text_format());

    job
}

/// top_menu_copy_epd()
/// "Position (EPD)"
pub fn top_menu_copy_epd() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Position (", 0.0, top_menu_text_format());
    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("PD)", 0.0, top_menu_text_format());

    job
}

/// top_menu_copy_analysis_window()
/// "Analysis Window"
pub fn top_menu_copy_analysis_window() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("A", 0.0, top_menu_text_format_underline());
    job.append("nalysis Window", 0.0, top_menu_text_format());

    job
}

/// top_menu_copy_last_analysis()
/// "Last Analysis"
pub fn top_menu_copy_last_analysis() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("L", 0.0, top_menu_text_format_underline());
    job.append("ast Analysis", 0.0, top_menu_text_format());

    job
}

/// top_menu_copy_notation()
/// "Notation"
pub fn top_menu_copy_notation() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("N", 0.0, top_menu_text_format_underline());
    job.append("otation", 0.0, top_menu_text_format());

    job
}

/// top_menu_paste()
/// "Paste"
pub fn top_menu_paste() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("P", 0.0, top_menu_text_format_underline());
    job.append("aste", 0.0, top_menu_text_format());

    job
}

/// top_menu_offer_draw()
/// "Offer Draw"
pub fn top_menu_offer_draw() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("O", 0.0, top_menu_text_format_underline());
    job.append("ffer Draw", 0.0, top_menu_text_format());

    job
}

/// top_menu_resign()
/// "Resign"
pub fn top_menu_resign() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("R", 0.0, top_menu_text_format_underline());
    job.append("esign", 0.0, top_menu_text_format());

    job
}

/// top_menu_goto_move()
/// "Goto Move..."
pub fn top_menu_goto_move() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("G", 0.0, top_menu_text_format_underline());
    job.append("oto Move...", 0.0, top_menu_text_format());

    job
}

/// top_menu_replay_game()
/// "Replay Game"
pub fn top_menu_replay_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Repla", 0.0, top_menu_text_format());
    job.append("y", 0.0, top_menu_text_format_underline());
    job.append(" Game", 0.0, top_menu_text_format());

    job
}

/// top_menu_show_main_line()
/// "Show Main Line"
pub fn top_menu_show_main_line() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Show ", 0.0, top_menu_text_format());
    job.append("M", 0.0, top_menu_text_format_underline());
    job.append("ain Line", 0.0, top_menu_text_format());

    job
}

/// top_menu_insert_main_line()
/// "Insert Main Line"
pub fn top_menu_insert_main_line() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("I", 0.0, top_menu_text_format_underline());
    job.append("nsert Main Line", 0.0, top_menu_text_format());

    job
}

/// top_menu_insert_best_move()
/// "Insert Best Move"
pub fn top_menu_insert_best_move() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Insert ", 0.0, top_menu_text_format());
    job.append("B", 0.0, top_menu_text_format_underline());
    job.append("est Move", 0.0, top_menu_text_format());

    job
}

/// top_menu_game_details()
/// "Game Details..."
pub fn top_menu_game_details() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("G", 0.0, top_menu_text_format_underline());
    job.append("ame Details...", 0.0, top_menu_text_format());

    job
}

/// top_menu_flip_board()
/// "Flip Board"
pub fn top_menu_flip_board() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("F", 0.0, top_menu_text_format_underline());
    job.append("lip Board", 0.0, top_menu_text_format());

    job
}
