use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

pub fn engines(ui: &mut Ui) -> InnerResponse<Option<()>> {
    // Engines
    egui::menu::menu_button(ui, layout_jobs::top_menu_engines(), |ui| {
        // Engines > Engine Options...
        if ui.button(layout_jobs::top_menu_engine_options()).clicked() {
            println!("Engine Options was clicked");
        }

        // Engines > Hash Tables...
        if ui.button(layout_jobs::top_menu_hash_tables()).clicked() {
            println!("Hash Tables was clicked");
        }
        // Engines > Permanent Brain
        if ui.button(layout_jobs::top_menu_permanent_brain()).clicked() {
            println!("Permanent Brain was clicked");
        }

        ui.separator();

        // Engines > Install Engine...
        if ui.button(layout_jobs::top_menu_install_engine()).clicked() {
            println!("Install Engine was clicked");
        }

        // Engines > Uninstall Engine...
        if ui
            .button(layout_jobs::top_menu_uninstall_engine())
            .clicked()
        {
            println!("Uninstall Engine was clicked");
        }

        // Engines > Edit Engine File...
        if ui
            .button(layout_jobs::top_menu_edit_engine_file())
            .clicked()
        {
            println!("Edit Engine File was clicked");
        }

        ui.separator();

        // Engines > Endgame Databases...
        if ui
            .button(layout_jobs::top_menu_endgame_databases())
            .clicked()
        {
            println!("Endgame Databases was clicked");
        }
    })
}
