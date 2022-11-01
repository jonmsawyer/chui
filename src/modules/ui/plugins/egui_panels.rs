//! MainUi plugin

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::modules::ui::events::ResizeBoardEvent;

use super::{ui_state::UiState, MainCamera, Fps, debug_panel};

pub mod layout_jobs;

pub mod top_menu;
use top_menu::top_menu;


pub const INFO_PANEL_WIDTH: f32 = 300.0;
pub const ANNOTATION_PANEL_WIDTH: f32 = 300.0;

fn egui_panels(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut resize_board_event: EventWriter<ResizeBoardEvent>,
    windows: Res<Windows>,
    fps: Local<Fps<25>>,
    time: Res<Time>,
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    top_menu(&mut egui_ctx, &mut ui_state, &mut resize_board_event);

    egui::TopBottomPanel::bottom("status")
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label(ui_state.status.as_str());
        });

    egui::SidePanel::left("info")
        .default_width(INFO_PANEL_WIDTH)
        .min_width(INFO_PANEL_WIDTH)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut ui_state.label);
            // });

            // ui.add(egui::widgets::Image::new(
            //     egui_texture_handle.id(),
            //     egui_texture_handle.size_vec2(),
            // ));

            // ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     ui_state.value += 1.0;
            // }

            // ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            // ui.horizontal(|ui| {
            //     load = ui.button("Load").clicked();
            //     invert = ui.button("Invert").clicked();
            //     remove = ui.button("Remove").clicked();
            // });

            // ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            // ui.checkbox(&mut ui_state.is_window_open, "Window Is Open");

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            //     ui.add(egui::Hyperlink::from_label_and_url(
            //         "powered by egui",
            //         "https://github.com/emilk/egui/",
            //     ));
            // });
            debug_panel(ui_state, windows, fps, time, ui, query);

            // ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.heading("Info Panel");
            ui.label("...");
    });

    egui::SidePanel::right("annotation")
        .min_width(ANNOTATION_PANEL_WIDTH)
        .default_width(ANNOTATION_PANEL_WIDTH)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Annotation");
        });

    // egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
    //     ui.heading("Game Board");
    //     // ui.hyperlink("https://github.com/emilk/egui_template");
    //     // ui.add(egui::github_link_file_line!(
    //     //     "https://github.com/mvlabat/bevy_egui/blob/main/",
    //     //     "Direct link to source code."
    //     // ));
    //     // egui::warn_if_debug_build(ui);

    //     // ui.separator();

    //     // ui.heading("Central Panel");
    //     // ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
    //     // ui.label("It is often a great place for big things, like drawings:");
    //     ui.label("Add game board here from assets.");
    //     //commands.spawn_bundle(Camera2dBundle::default());
    // });

    // egui::Window::new("Window")
    //     .vscroll(true)
    //     .open(&mut ui_state.is_window_open)
    //     .show(egui_ctx.ctx_mut(), |ui| {
    //         ui.label("Windows can be moved by dragging them.");
    //         ui.label("They are automatically sized based on contents.");
    //         ui.label("You can turn on resizing and scrolling if you like.");
    //         ui.label("You would normally chose either panels OR windows.");
    //     });
}

pub struct EguiPanelsPlugin;

impl Plugin for EguiPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(egui_panels)
            // Set multi-sample anti-aliasing (WGPU only supports 1 or 4)
            .insert_resource(Msaa { samples: 4 });
    }
}