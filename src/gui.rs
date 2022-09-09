use egui::Context;

use crate::info::{InputInfo, GUIInfo};

pub fn run(egui_ctx: &Context, input: &InputInfo, mut gui_info: GUIInfo) -> (bool, GUIInfo) {
    let mut main_panel = egui::SidePanel::left("actions");
    main_panel = main_panel.resizable(false);

    let mut quit = false;

    let mut new_world_menu = egui::Window::new("New World");

    egui::SidePanel::show(main_panel, egui_ctx, |ui| {
        ui.heading("Actions");

        if ui.button("New").clicked() {
            gui_info.new_menu_opened = true;
        }

        if ui.button("Quit").clicked() {
            quit = true;
        }
        //ui.add(egui::Slider::new(&mut input.zoom_modifier, 0.01..=0.05).text("Zoom Speed"));
    });

    if gui_info.new_menu_opened {
        egui::Window::show(new_world_menu, egui_ctx, |ui| {
            ui.heading("New World Menu");
            //ui.image(texture_id, size)
        });
    }

    (quit, gui_info)
}
