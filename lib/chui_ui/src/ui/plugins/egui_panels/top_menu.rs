//! Top Menu module.

use bevy::prelude::{EventWriter, ResMut};
use bevy_egui::{egui, EguiContexts};

use crate::ui::events::ResizeBoardEvent;
use crate::ui::resources::UiResource;

pub mod file;
pub use file::file;

pub mod commands;
pub use commands::commands;

pub mod copy;
pub use copy::copy;

pub mod levels;
pub use levels::levels;

pub mod mode;
pub use mode::mode;

pub mod training;
pub use training::training;

pub mod cpu_vs_cpu;
pub use cpu_vs_cpu::cpu_vs_cpu;

pub mod extras;
pub use extras::extras;

pub mod engines;
pub use engines::engines;

pub mod opening_book;
pub use opening_book::opening_book;

pub mod windows;
pub use windows::windows;

pub mod design;
pub use design::design;

pub mod help;
pub use help::help;

pub use super::layout_jobs;

/// Generate the top menu using egui.
pub fn top_menu(
    egui_ctx: &mut EguiContexts,
    ui_state: &mut ResMut<UiResource>,
    resize_board_event: &mut EventWriter<ResizeBoardEvent>,
) {
    egui::TopBottomPanel::top("menu").show(egui_ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui_egui| {
            file(ui_egui);
            commands(ui_egui, ui_state, resize_board_event);
            copy(ui_egui);
            levels(ui_egui);
            mode(ui_egui);
            training(ui_egui);
            cpu_vs_cpu(ui_egui);
            engines(ui_egui);
            opening_book(ui_egui);
            extras(ui_egui);
            windows(ui_egui);
            design(ui_egui);
            help(ui_egui);
        });
    });
}
