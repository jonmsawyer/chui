use super::layout_jobs;

use bevy_egui::egui::{self, Ui, InnerResponse};

pub fn opening_book(ui: &mut Ui) -> InnerResponse<Option<()>> {
    // Opening Book
    egui::menu::menu_button(ui, layout_jobs::top_menu_opening_book(), |ui| {
        // Opening Book > New Opening Book...
        if ui.button(layout_jobs::top_menu_new_opening_book())
            .clicked()
        {
            println!("New Opening Book was clicked");
        }

        // Opening Book > Load Opening Book...
        if ui.button(layout_jobs::top_menu_load_opening_book())
            .clicked()
        {
            println!("Load Opening Book was clicked");
        }

        ui.separator();

        // Opening Book > Import Opening Book...
        if ui.button(layout_jobs::top_menu_import_opening_book())
            .clicked()
        {
            println!("Import Opening Book was clicked");
        }

        // Opening Book > Import Games...
        if ui.button(layout_jobs::top_menu_import_games())
            .clicked()
        {
            println!("Import Games was clicked");
        }

        // Opening Book > Import Current Game...
        if ui.button(layout_jobs::top_menu_import_current_game())
            .clicked()
        {
            println!("Import Current Game was clicked");
        }

        // Opening Book > Export Opening Book...
        if ui.button(layout_jobs::top_menu_export_opening_book())
            .clicked()
        {
            println!("Export Opening Book was clicked");
        }

        ui.separator();

        // Opening Book > Count Book Moves
        if ui.button(layout_jobs::top_menu_count_book_moves())
            .clicked()
        {
            println!("Count Book Moves was clicked");
        }

        // Opening Book > Weed Opening Book...
        if ui.button(layout_jobs::top_menu_weed_opening_book())
            .clicked()
        {
            println!("Weed Opening Book was clicked");
        }
    })
}
