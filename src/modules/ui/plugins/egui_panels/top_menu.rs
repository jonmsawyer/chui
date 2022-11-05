use bevy::prelude::{EventWriter, ResMut};
use bevy_egui::{egui, EguiContext};

use crate::modules::ui::events::ResizeBoardEvent;
use crate::modules::ui::resources::UiResource;

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

pub fn top_menu(
    egui_ctx: &mut ResMut<EguiContext>,
    ui_state: &mut ResMut<UiResource>,
    resize_board_event: &mut EventWriter<ResizeBoardEvent>,
) {
    egui::TopBottomPanel::top("menu").show(egui_ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            file(ui);
            commands(ui, ui_state, resize_board_event);
            copy(ui);
            levels(ui);
            mode(ui);
            training(ui);
            cpu_vs_cpu(ui);
            engines(ui);
            opening_book(ui);
            extras(ui);
            windows(ui);
            design(ui);
            help(ui);
        });
    });
}
