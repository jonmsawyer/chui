//! Levels module.

use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

/// Levels menu.
pub fn levels(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_levels(), |ui_egui| {
        // Levels > Playing Strength...
        if ui_egui
            .button(layout_jobs::top_menu_playing_strength())
            .clicked()
        {
            println!("Playing Strength was clicked");
        }

        ui_egui.separator();

        // Levels > Blitz...
        if ui_egui.button(layout_jobs::top_menu_blitz()).clicked() {
            println!("Blitz was clicked");
        }

        // Levels > Time Per Move...
        if ui_egui
            .button(layout_jobs::top_menu_time_per_move())
            .clicked()
        {
            println!("Time Per Move was clicked");
        }

        // Levels > Save Game...
        if ui_egui.button(layout_jobs::top_menu_save_game()).clicked() {
            println!("Save Game was clicked");
        }

        // Levels > Time Controls...
        if ui_egui
            .button(layout_jobs::top_menu_time_controls())
            .clicked()
        {
            println!("Time Controls was clicked");
        }

        ui_egui.separator();

        // Levels > Fixed Search Depth...
        if ui_egui
            .button(layout_jobs::top_menu_fixed_search_depth())
            .clicked()
        {
            println!("Fixed Search Depth was clicked");
        }
    })
}
