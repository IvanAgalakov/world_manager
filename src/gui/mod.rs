use egui::Context;

pub fn run(egui_ctx: &Context) -> bool{
    let mut test = egui::Window::new("test window");
    test = test.resizable(false);

    let mut quit = false;

    egui::Window::show(test, egui_ctx, |ui| {
        ui.heading("I love testing window stuff");
        if ui.button("Quit").clicked() {
            quit = true;
        }
    });

    quit
}
