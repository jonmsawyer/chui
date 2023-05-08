/*!
Layout jobs for the Levels menu

A layout job is the custom way to format [`RichText`].
*/

use super::{top_menu_text_format, top_menu_text_format_underline};
use bevy_egui::egui::text::LayoutJob;

/// "Mode"
pub fn top_menu_mode() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("M", 0.0, top_menu_text_format_underline());
    job.append("ode", 0.0, top_menu_text_format());

    job
}

/// "Analysis"
pub fn top_menu_analysis() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("A", 0.0, top_menu_text_format_underline());
    job.append("nalysis", 0.0, top_menu_text_format());

    job
}

/// "Play Against Computer"
pub fn top_menu_play_against_computer() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("P", 0.0, top_menu_text_format_underline());
    job.append("lay Against Computer", 0.0, top_menu_text_format());

    job
}

/// "Enter Moves"
pub fn top_menu_enter_moves() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("nter Moves", 0.0, top_menu_text_format());

    job
}

/// "Training"
pub fn top_menu_training() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("T", 0.0, top_menu_text_format_underline());
    job.append("raining", 0.0, top_menu_text_format());

    job
}

/// "Openings Training"
pub fn top_menu_openings_training() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("O", 0.0, top_menu_text_format_underline());
    job.append("penings Training", 0.0, top_menu_text_format());

    job
}

/// "Endgame Training"
pub fn top_menu_endgame_training() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("ndgame Training", 0.0, top_menu_text_format());

    job
}

/// "Handicap Games"
pub fn top_menu_handicap_games() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("H", 0.0, top_menu_text_format_underline());
    job.append("andicap Games", 0.0, top_menu_text_format());

    job
}

/// "Chess Puzzles"
pub fn top_menu_chess_puzzles() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("C", 0.0, top_menu_text_format_underline());
    job.append("hess Puzzles", 0.0, top_menu_text_format());

    job
}

/// "Daily Puzzle - Easy"
pub fn top_menu_daily_puzzle_easy() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Daily Puzzle - ", 0.0, top_menu_text_format());
    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("asy", 0.0, top_menu_text_format());

    job
}

/// "Daily Puzzle - Medium"
pub fn top_menu_daily_puzzle_medium() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Daily Puzzle - ", 0.0, top_menu_text_format());
    job.append("M", 0.0, top_menu_text_format_underline());
    job.append("edium", 0.0, top_menu_text_format());

    job
}

/// "Daily Puzzle - Hard"
pub fn top_menu_daily_puzzle_hard() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Daily Puzzle - ", 0.0, top_menu_text_format());
    job.append("H", 0.0, top_menu_text_format_underline());
    job.append("ard", 0.0, top_menu_text_format());

    job
}

/// "Triple Brain..."
pub fn top_menu_triple_brain() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Triple ", 0.0, top_menu_text_format());
    job.append("B", 0.0, top_menu_text_format_underline());
    job.append("rain...", 0.0, top_menu_text_format());

    job
}

/// "Analyse Game..."
pub fn top_menu_analyse_game() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Analyse ", 0.0, top_menu_text_format());
    job.append("G", 0.0, top_menu_text_format_underline());
    job.append("ame...", 0.0, top_menu_text_format());

    job
}

/// "Analyse Positions..."
pub fn top_menu_analyse_positions() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Analyse ", 0.0, top_menu_text_format());
    job.append("P", 0.0, top_menu_text_format_underline());
    job.append("ositions...", 0.0, top_menu_text_format());

    job
}

/// "Computer vs Computer"
pub fn top_menu_computer_vs_computer() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Computer ", 0.0, top_menu_text_format());
    job.append("v", 0.0, top_menu_text_format_underline());
    job.append("s Computer", 0.0, top_menu_text_format());

    job
}

/// "Shootout..."
pub fn top_menu_shootout() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("S", 0.0, top_menu_text_format_underline());
    job.append("hootout...", 0.0, top_menu_text_format());

    job
}

/// "Autoplayer..."
pub fn top_menu_autoplayer() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("A", 0.0, top_menu_text_format_underline());
    job.append("utoplayer...", 0.0, top_menu_text_format());

    job
}

/// "Engine Match..."
pub fn top_menu_engine_match() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("E", 0.0, top_menu_text_format_underline());
    job.append("ngine Match...", 0.0, top_menu_text_format());

    job
}

/// "Engine Tournament"
pub fn top_menu_engine_tournament() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("Engine ", 0.0, top_menu_text_format());
    job.append("T", 0.0, top_menu_text_format_underline());
    job.append("ournament", 0.0, top_menu_text_format());

    job
}

/// "DGT Board"
pub fn top_menu_dgt_board() -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("D", 0.0, top_menu_text_format_underline());
    job.append("GT Board", 0.0, top_menu_text_format());

    job
}
