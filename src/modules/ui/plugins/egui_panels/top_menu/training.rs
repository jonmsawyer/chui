//! Training module.

use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

/// Training menu.
pub fn training(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_training(), |ui_egui| {
        // Mode > Training > Openings Training
        if ui_egui
            .button(layout_jobs::top_menu_openings_training())
            .clicked()
        {
            println!("Openings Training was clicked");
        }

        // Mode > Training > Endgame Training
        if ui_egui
            .button(layout_jobs::top_menu_endgame_training())
            .clicked()
        {
            println!("Endgame Training was clicked");
        }

        // Mode > Training > Handicap Games
        if ui_egui
            .button(layout_jobs::top_menu_handicap_games())
            .clicked()
        {
            println!("Handicap Games was clicked");
        }

        ui_egui.separator();

        // Mode > Training > Chess Puzzles
        if ui_egui
            .button(layout_jobs::top_menu_chess_puzzles())
            .clicked()
        {
            println!("Chess Puzzles was clicked");
        }

        ui_egui.separator();

        // Mode > Training > Daily Puzzle - Easy
        if ui_egui
            .button(layout_jobs::top_menu_daily_puzzle_easy())
            .clicked()
        {
            println!("Daily Puzzle Easy was clicked");
        }

        // Mode > Training > Daily Puzzle - Medium
        if ui_egui
            .button(layout_jobs::top_menu_daily_puzzle_medium())
            .clicked()
        {
            println!("Daily Puzzle Medium was clicked");
        }

        // Mode > Training > Daily Puzzle - Hard
        if ui_egui
            .button(layout_jobs::top_menu_daily_puzzle_hard())
            .clicked()
        {
            println!("Daily Puzzle Hard was clicked");
        }
    })
}
