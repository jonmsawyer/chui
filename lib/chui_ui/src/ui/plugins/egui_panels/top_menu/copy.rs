//! Copy module.

use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

/// Copy menu.
pub fn copy(ui: &mut Ui) -> InnerResponse<Option<()>> {
    // Commands > Copy
    egui::menu::menu_button(ui, layout_jobs::top_menu_copy(), |ui_egui| {
        // Commands > Copy > Game (PGN)
        if ui_egui.button(layout_jobs::top_menu_copy_pgn()).clicked() {
            println!("Copy Game (PGN) was clicked");
        }

        // Commands > Copy > Position (EPD)
        if ui_egui.button(layout_jobs::top_menu_copy_epd()).clicked() {
            println!("Copy Position (EPD) was clicked");
        }

        // Commands > Copy > Analysis Window
        if ui_egui
            .button(layout_jobs::top_menu_copy_analysis_window())
            .clicked()
        {
            println!("Copy Analysis Window was clicked");
        }

        // Commands > Copy > Last Analysis
        if ui_egui
            .button(layout_jobs::top_menu_copy_last_analysis())
            .clicked()
        {
            println!("Copy Last Analysis was clicked");
        }

        // Commands > Copy > Notation
        if ui_egui
            .button(layout_jobs::top_menu_copy_notation())
            .clicked()
        {
            println!("Copy Notation was clicked");
        }
    })
}
