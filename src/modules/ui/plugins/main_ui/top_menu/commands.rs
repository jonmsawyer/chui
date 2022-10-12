use super::layout_jobs;

use bevy_egui::egui::{self, Ui, InnerResponse};

pub fn commands(ui: &mut Ui) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_commands(), |ui| {
        // Commands > Compute / Switch Sides
        if ui.button(layout_jobs::top_menu_compute())
            .clicked()
        {
            println!("Compute / Switch Sides was clicked");
        }

        // Commands > Interrupt
        if ui.button(layout_jobs::top_menu_interrupt())
            .clicked()
        {
            println!("Interrupt was clicked");
        }

        ui.separator();

        // Commands > Paste
        if ui.button(layout_jobs::top_menu_paste())
            .clicked()
        {
            println!("Paste was clicked");
        }

        ui.separator();

        // Commands > Offer Draw
        if ui.button(layout_jobs::top_menu_offer_draw())
            .clicked()
        {
            println!("Offer Draw was clicked");
        }

        // Commands > Resign
        if ui.button(layout_jobs::top_menu_resign())
            .clicked()
        {
            println!("Resign was clicked");
        }

        ui.separator();

        // Commands > Goto Move...
        if ui.button(layout_jobs::top_menu_goto_move())
            .clicked()
        {
            println!("Goto Move was clicked");
        }

        // Commands > Replay Game
        if ui.button(layout_jobs::top_menu_replay_game())
            .clicked()
        {
            println!("Replay Game was clicked");
        }

        ui.separator();

        // Commands > Show Main Line
        if ui.button(layout_jobs::top_menu_show_main_line())
            .clicked()
        {
            println!("Show Main Line was clicked");
        }

        // Commands > Insert Main Line
        if ui.button(layout_jobs::top_menu_insert_main_line())
            .clicked()
        {
            println!("Insert Main Line was clicked");
        }

        // Commands > Insert Best Move
        if ui.button(layout_jobs::top_menu_insert_best_move())
            .clicked()
        {
            println!("Insert Best Move was clicked");
        }

        // Commands > Game Details...
        if ui.button(layout_jobs::top_menu_game_details())
            .clicked()
        {
            println!("Game Details was clicked");
        }
    })
}
