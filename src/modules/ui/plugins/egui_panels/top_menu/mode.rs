use super::layout_jobs;

use bevy_egui::egui::{self, InnerResponse, Ui};

pub fn mode(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_mode(), |ui| {
        // Mode > Analysis
        if ui.button(layout_jobs::top_menu_analysis()).clicked() {
            println!("Analysis was clicked");
        }

        ui.separator();

        // Mode > Play Against Computer...
        if ui
            .button(layout_jobs::top_menu_play_against_computer())
            .clicked()
        {
            println!("Play Against Computer was clicked");
        }

        // Mode > Enter Moves
        if ui.button(layout_jobs::top_menu_enter_moves()).clicked() {
            println!("Enter Moves was clicked");
        }

        ui.separator();

        // Mode > Triple Brain...
        if ui.button(layout_jobs::top_menu_triple_brain()).clicked() {
            println!("Triple Brain was clicked");
        }

        ui.separator();

        // Mode > Ananlyse Game...
        if ui.button(layout_jobs::top_menu_analyse_game()).clicked() {
            println!("Analyse Game was clicked");
        }

        // Mode > Analyse Positions...
        if ui
            .button(layout_jobs::top_menu_analyse_positions())
            .clicked()
        {
            println!("Analyse Positions was clicked");
        }

        ui.separator();

        // Mode > DGT Board
        if ui.button(layout_jobs::top_menu_dgt_board()).clicked() {
            println!("DGT Board was clicked");
        }
    })
}
