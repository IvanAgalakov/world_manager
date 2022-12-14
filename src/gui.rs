use std::{alloc::System, collections::VecDeque, ops::Deref, path::PathBuf, time::Instant};

use egui::{epaint::tessellator::path, Context, TextBuffer, TextureHandle, Vec2};
use glium::Display;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;

use crate::{
    info::{GUIInfo, InputInfo, WorldInfo},
    texture_manager, geometry, utils,
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
                let path_to_texture = tinyfiledialogs::open_file_dialog(
                    "open the base image for your world",
                    &document_dir,
                    Some((&["*.png"; 1], ".png")),
                );
                if path_to_texture.is_some() {
                    let path_to_texture = path_to_texture.unwrap();
                    let dyn_tex = texture_manager::get_dynamic_image(&path_to_texture);
                    let mut dyn_tex_copy = DynamicImage::clone(&dyn_tex);

                    //calculating image width in world units
                    let width = dyn_tex.width();
                    let height = dyn_tex.height();
                    let aspect = (width as f32)/(height as f32);
                    let x: f32 = aspect;
                    let y: f32 = -1.0;
                    world_info.bottom_right = (x*aspect,y);
                    world_info.top_left = (-1.0*aspect,1.0);

                    
                    let lines = geometry::generate_mesh_from_image(&mut dyn_tex_copy);
                    world_info.lines = lines;
                    println!("length of: {}", world_info.lines.len());
                    let tri = utils::vertices_from_lines(0.01,&world_info.lines);
                    world_info.triangles = tri;
                    //println!("vertice representation {:?}", world_info.triangles);
                    //println!("{:?}", &world_info.lines);

                    let world_tex = texture_manager::get_texture_data(dis, egui_ctx, &dyn_tex);
                    world_info.world_texture = Some(world_tex);
                }
            }

            if world_info.world_texture.is_some() {
                let tex_han: &TextureHandle =
                    &world_info.world_texture.as_ref().unwrap().gui_texture;
                let ratio = tex_han.aspect_ratio();
                let s = Vec2::new(100.0 * ratio, 100.0);
                ui.image(tex_han, s);

                let slider_ocean = egui::Slider::new(&mut world_info.ocean_line_num, 1..=20).text("Ocean Line #");
                ui.add(slider_ocean);
                
            }
        });
    }

    (quit, gui_info)
}

