use super::layout_jobs;

use bevy_egui::egui::{self, Ui, InnerResponse};

pub fn cpu_vs_cpu(ui: &mut Ui) -> InnerResponse<Option<()>> {
    // Mode > Computer vs Computer
    egui::menu::menu_button(ui, layout_jobs::top_menu_computer_vs_computer(), |ui| {
        // Mode > Computer vs Computer > Shootout
        if ui.button(layout_jobs::top_menu_shootout())
            .clicked()
        {
            println!("Shootout was clicked");
        }

        ui.separator();

        // Mode > Computer vs Computer > Autoplayer...
        if ui.button(layout_jobs::top_menu_autoplayer())
            .clicked()
        {
            println!("Autoplayer was clicked");
        }

        // Mode > Computer vs Computer > Engine Match...
        if ui.button(layout_jobs::top_menu_engine_match())
            .clicked()
        {
            println!("Engine Match was clicked");
        }

        // Mode > Computer vs Computer > Engine Tournament
        if ui.button(layout_jobs::top_menu_engine_tournament())
            .clicked()
        {
            println!("Engine Tournament was clicked");
        }
    })
}
