//! Help module.

use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

/// Help menu.
pub fn help(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, "Help", |ui_egui| {
        // File > New Game...
        if ui_egui.button(layout_jobs::top_menu_new_game()).clicked() {
            println!("New Game was clicked");
        }

        // File > New Chess960 Game...
        if ui_egui
            .button(layout_jobs::top_menu_new_chess960_game())
            .clicked()
        {
            println!("New Chess960 Game was clicked");
        }

        ui_egui.separator();

        // File > Load Game...
        if ui_egui.button(layout_jobs::top_menu_load_game()).clicked() {
            println!("Load Game was clicked");
        }

        // File > Save Game...
        if ui_egui.button(layout_jobs::top_menu_save_game()).clicked() {
            println!("Save Game was clicked");
        }

        // File > Delete Game...
        if ui_egui
            .button(layout_jobs::top_menu_delete_game())
            .clicked()
        {
            println!("Delete Game was clicked");
        }

        ui_egui.separator();

        // File > Load Position...
        if ui_egui
            .button(layout_jobs::top_menu_load_position())
            .clicked()
        {
            println!("Load Position was clicked");
        }

        // File > Save Position...
        if ui_egui
            .button(layout_jobs::top_menu_save_position())
            .clicked()
        {
            println!("Save Position was clicked");
        }

        ui_egui.separator();

        // File > Recent Files...
        if ui_egui
            .button(layout_jobs::top_menu_recent_files())
            .clicked()
        {
            println!("Recent Files was clicked");
        }

        // File > Edit Position...
        if ui_egui
            .button(layout_jobs::top_menu_edit_position())
            .clicked()
        {
            println!("Edit Position was clicked");
        }

        // File > Print Game...
        if ui_egui.button(layout_jobs::top_menu_print_game()).clicked() {
            println!("Print Game was clicked");
        }

        ui_egui.separator();

        // File > Exit
        if ui_egui.button(layout_jobs::top_menu_exit()).clicked() {
            std::process::exit(0);
        }
    })
}
