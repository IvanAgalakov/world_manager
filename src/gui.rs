use egui::Context;

use crate::info::InputInfo;


pub fn run(egui_ctx: &Context, mut input: InputInfo) -> (bool, InputInfo) {
    let mut test = egui::SidePanel::left("actions");
    test = test.resizable(false);

    let mut quit = false;

    egui::SidePanel::show(test, egui_ctx, |ui| {
        ui.heading("Actions");
        if ui.button("Quit").clicked() {
            quit = true;
        }
        ui.add(egui::Slider::new(&mut input.zoom_modifier, 0.01..=0.05).text("Zoom Speed"));
    });

    (quit, input)
}
