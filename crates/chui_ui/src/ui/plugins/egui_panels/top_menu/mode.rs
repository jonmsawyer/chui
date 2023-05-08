//! Mode module.

use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

/// Mode menu.
pub fn mode(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_mode(), |ui_egui| {
        // Mode > Analysis
        if ui_egui.button(layout_jobs::top_menu_analysis()).clicked() {
            println!("Analysis was clicked");
        }

        ui_egui.separator();

        // Mode > Play Against Computer...
        if ui_egui
            .button(layout_jobs::top_menu_play_against_computer())
            .clicked()
        {
            println!("Play Against Computer was clicked");
        }

        // Mode > Enter Moves
        if ui_egui
            .button(layout_jobs::top_menu_enter_moves())
            .clicked()
        {
            println!("Enter Moves was clicked");
        }

        ui_egui.separator();

        // Mode > Triple Brain...
        if ui_egui
            .button(layout_jobs::top_menu_triple_brain())
            .clicked()
        {
            println!("Triple Brain was clicked");
        }

        ui_egui.separator();

        // Mode > Ananlyse Game...
        if ui_egui
            .button(layout_jobs::top_menu_analyse_game())
            .clicked()
        {
            println!("Analyse Game was clicked");
        }

        // Mode > Analyse Positions...
        if ui_egui
            .button(layout_jobs::top_menu_analyse_positions())
            .clicked()
        {
            println!("Analyse Positions was clicked");
        }

        ui_egui.separator();

        // Mode > DGT Board
        if ui_egui.button(layout_jobs::top_menu_dgt_board()).clicked() {
            println!("DGT Board was clicked");
        }
    })
}
