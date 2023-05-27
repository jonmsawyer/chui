//! Commands module.

use bevy::prelude::EventWriter;
use bevy::prelude::ResMut;
use bevy_egui::egui::{self, InnerResponse, Ui};

use super::layout_jobs;
use crate::ui::events;
use crate::ui::resources::UiResource;

/// Commands menu.
pub fn commands(
    ui: &mut Ui,
    ui_state: &mut ResMut<UiResource>,
    resize_board_event: &mut EventWriter<events::ResizeBoardEvent>,
) -> InnerResponse<Option<()>> {
    egui::menu::menu_button(ui, layout_jobs::top_menu_commands(), |ui_egui| {
        // Commands > Compute / Switch Sides
        if ui_egui.button(layout_jobs::top_menu_compute()).clicked() {
            println!("Compute / Switch Sides was clicked");
        }

        // Commands > Interrupt
        if ui_egui.button(layout_jobs::top_menu_interrupt()).clicked() {
            println!("Interrupt was clicked");
        }

        ui_egui.separator();

        // Commands > Paste
        if ui_egui.button(layout_jobs::top_menu_paste()).clicked() {
            println!("Paste was clicked");
        }

        ui_egui.separator();

        // Commands > Offer Draw
        if ui_egui.button(layout_jobs::top_menu_offer_draw()).clicked() {
            println!("Offer Draw was clicked");
        }

        // Commands > Resign
        if ui_egui.button(layout_jobs::top_menu_resign()).clicked() {
            println!("Resign was clicked");
        }

        // Commands > Flip Board
        if ui_egui.button(layout_jobs::top_menu_flip_board()).clicked() {
            ui_state.draw_for_white = !ui_state.draw_for_white;
            resize_board_event.send_default();
            println!("Flip Board was clicked");
        }

        ui_egui.separator();

        // Commands > Goto Move...
        if ui_egui.button(layout_jobs::top_menu_goto_move()).clicked() {
            println!("Goto Move was clicked");
        }

        // Commands > Replay Game
        if ui_egui
            .button(layout_jobs::top_menu_replay_game())
            .clicked()
        {
            println!("Replay Game was clicked");
        }

        ui_egui.separator();

        // Commands > Show Main Line
        if ui_egui
            .button(layout_jobs::top_menu_show_main_line())
            .clicked()
        {
            println!("Show Main Line was clicked");
        }

        // Commands > Insert Main Line
        if ui_egui
            .button(layout_jobs::top_menu_insert_main_line())
            .clicked()
        {
            println!("Insert Main Line was clicked");
        }

        // Commands > Insert Best Move
        if ui_egui
            .button(layout_jobs::top_menu_insert_best_move())
            .clicked()
        {
            println!("Insert Best Move was clicked");
        }

        // Commands > Game Details...
        if ui_egui
            .button(layout_jobs::top_menu_game_details())
            .clicked()
        {
            println!("Game Details was clicked");
        }
    })
}
