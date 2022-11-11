//! Design module.

use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

/// Design menu.
pub fn design(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_design(), |ui_egui| {
        // Design > Default
        if ui_egui.button(layout_jobs::top_menu_default()).clicked() {
            println!("Default was clicked");
        }

        ui_egui.separator();

        // Design > Change Design
        if ui_egui
            .button(layout_jobs::top_menu_change_design())
            .clicked()
        {
            println!("Change Design was clicked");
        }

        ui_egui.separator();

        // Design > Load Design
        if ui_egui
            .button(layout_jobs::top_menu_load_design())
            .clicked()
        {
            println!("Load Design was clicked");
        }

        // Design > Save Design
        if ui_egui
            .button(layout_jobs::top_menu_save_design())
            .clicked()
        {
            println!("Save Design was clicked");
        }

        ui_egui.separator();

        // Design > Background...
        if ui_egui.button(layout_jobs::top_menu_background()).clicked() {
            println!("Background was clicked");
        }

        // Design > Colors...
        if ui_egui.button(layout_jobs::top_menu_colors()).clicked() {
            println!("Colors was clicked");
        }

        // Design > Window Title Bars
        if ui_egui
            .button(layout_jobs::top_menu_window_title_bars())
            .clicked()
        {
            println!("Window Title Bars was clicked");
        }
    })
}
