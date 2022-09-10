use std::{path::PathBuf, ops::Deref, alloc::System};

use egui::{Context, TextureHandle, Vec2, TextBuffer, epaint::tessellator::path};
use glium::Display;

use crate::{
    info::{GUIInfo, InputInfo, WorldInfo},
    texture_manager,
};

pub fn run(
    egui_ctx: &Context,
    dis: &Display,
    input: &InputInfo,
    mut gui_info: GUIInfo,
    world_info: &mut WorldInfo,
) -> (bool, GUIInfo) {

    let mut main_panel = egui::SidePanel::left("actions");
    main_panel = main_panel.resizable(false);

    let mut quit = false;

    let mut new_world_menu = egui::Window::new("New World");

    egui::SidePanel::show(main_panel, egui_ctx, |ui| {
        ui.heading("Actions");

        if ui.button("New").clicked() {
            gui_info.new_menu_opened = true;
            // if !world_info.created {
            //     //world_info.world_texture = Some(texture_manager::get_texture(dis, egui_ctx));
            // }
        }

        if ui.button("Quit").clicked() {
            quit = true;
        }
        //ui.add(egui::Slider::new(&mut input.zoom_modifier, 0.01..=0.05).text("Zoom Speed"));
    });

    if gui_info.new_menu_opened {
        egui::Window::show(new_world_menu, egui_ctx, |ui| {
            ui.heading("New World Menu");
            if ui.button("open base image").clicked() {
                let document_dir = dirs_next::document_dir().unwrap();
                let document_dir = document_dir.into_os_string().into_string().unwrap();
                let path_to_texture = tinyfiledialogs::open_file_dialog("open the base image for your world", &document_dir, Some((&["*.png"; 1], ".png")));
                if path_to_texture.is_some() {
                    let path_to_texture = path_to_texture.unwrap();
                    let world_tex = texture_manager::get_texture(dis, egui_ctx, &path_to_texture);
                    world_info.world_texture = Some(world_tex);
                }
            }

            if world_info.world_texture.is_some() {
                let tex_han: &TextureHandle = &world_info.world_texture.as_ref().unwrap().gui_texture;
                let ratio = tex_han.aspect_ratio();
                let s = Vec2::new(100.0*ratio, 100.0);
                ui.image(tex_han, s);
            }
        });
    }

    (quit, gui_info)
}
