use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

pub fn windows(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_windows(), |ui| {
        // Windows > Board
        if ui.button(layout_jobs::top_menu_board()).clicked() {
            println!("Board was clicked");
        }

        // Windows > Clock
        if ui.button(layout_jobs::top_menu_clock()).clicked() {
            println!("Clock was clicked");
        }

        // Windows > Notation
        if ui.button(layout_jobs::top_menu_notation()).clicked() {
            println!("Notation was clicked");
        }

        // Windows > Engine
        if ui.button(layout_jobs::top_menu_engine()).clicked() {
            println!("Engine was clicked");
        }

        // Windows > Moves
        if ui.button(layout_jobs::top_menu_moves()).clicked() {
            println!("Moves was clicked");
        }

        // Windows > Histogram
        if ui.button(layout_jobs::top_menu_histogram()).clicked() {
            println!("Histogram was clicked");
        }
    })
}
