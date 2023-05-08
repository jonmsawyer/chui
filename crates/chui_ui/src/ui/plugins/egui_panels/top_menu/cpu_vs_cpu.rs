//! CPU vs CPU module.

use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

/// CPU vs CPU menu.
pub fn cpu_vs_cpu(ui: &mut Ui) -> InnerResponse<Option<()>> {
    // Mode > Computer vs Computer
    egui::menu::menu_button(
        ui,
        layout_jobs::top_menu_computer_vs_computer(),
        |ui_egui| {
            // Mode > Computer vs Computer > Shootout
            if ui_egui.button(layout_jobs::top_menu_shootout()).clicked() {
                println!("Shootout was clicked");
            }

            ui_egui.separator();

            // Mode > Computer vs Computer > Autoplayer...
            if ui_egui.button(layout_jobs::top_menu_autoplayer()).clicked() {
                println!("Autoplayer was clicked");
            }

            // Mode > Computer vs Computer > Engine Match...
            if ui_egui
                .button(layout_jobs::top_menu_engine_match())
                .clicked()
            {
                println!("Engine Match was clicked");
            }

            // Mode > Computer vs Computer > Engine Tournament
            if ui_egui
                .button(layout_jobs::top_menu_engine_tournament())
                .clicked()
            {
                println!("Engine Tournament was clicked");
            }
        },
    )
}
