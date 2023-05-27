//! Extras module.

use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

/// Extras menu.
pub fn extras(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_extras(), |ui_egui| {
        // Extras > Query Online Database
        if ui_egui
            .button(layout_jobs::top_menu_query_online_database())
            .clicked()
        {
            println!("Query Online Database was clicked");
        }

        // Extras > Publish Game...
        if ui_egui
            .button(layout_jobs::top_menu_publish_game())
            .clicked()
        {
            println!("Publish Game was clicked");
        }

        // Extras > Endgame Oracle...
        if ui_egui
            .button(layout_jobs::top_menu_endgame_oracle())
            .clicked()
        {
            println!("Endgame Oracle was clicked");
        }

        ui_egui.separator();

        // Extras > My Results...
        if ui_egui.button(layout_jobs::top_menu_my_results()).clicked() {
            println!("My Results was clicked");
        }

        // Extras > Options...
        if ui_egui.button(layout_jobs::top_menu_options()).clicked() {
            println!("Options was clicked");
        }
    })
}
