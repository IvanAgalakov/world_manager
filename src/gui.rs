use std::{alloc::System, collections::VecDeque, ops::Deref, path::PathBuf, time::Instant};

use egui::{epaint::tessellator::path, Context, TextBuffer, TextureHandle, Vec2};
use glium::Display;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;

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
                let path_to_texture = tinyfiledialogs::open_file_dialog(
                    "open the base image for your world",
                    &document_dir,
                    Some((&["*.png"; 1], ".png")),
                );
                if path_to_texture.is_some() {
                    let path_to_texture = path_to_texture.unwrap();
                    let mut dyn_tex = texture_manager::get_dynamic_image(&path_to_texture);

                    

                    let mut start_x: i32 = -1;
                    let mut start_y: i32 = -1;
                    for x in 0..dyn_tex.width() {
                        for y in 0..dyn_tex.height() {
                            let pix = dyn_tex.get_pixel(x, y);
                            if pix.0[3] > 50 {
                                dyn_tex.put_pixel(
                                    x,
                                    y,
                                    Rgba {
                                        0: [255, 255, 255, 255],
                                    },
                                );
                                start_x = x as i32;
                                start_y = y as i32;
                            }
                        }
                    }

                    if start_x != -1 {

                        fill(
                            start_x as u32,
                            start_y as u32,
                            &mut dyn_tex,
                            Rgba {
                                0: [250, 0, 0, 255],
                            },
                        );

                        let white = Rgba {
                            0: [255,255,255,255],
                        };

                        let mut rng = rand::thread_rng();

                        for x in 0..dyn_tex.width() {
                            for y in 0..dyn_tex.height() {
                                if dyn_tex.get_pixel(x, y).eq(&white) {
                                    let r = rng.gen_range(1..255);
                                    let g = rng.gen_range(1..255);
                                    let b = rng.gen_range(1..255);
                                    fill(x, y, &mut dyn_tex, Rgba {
                                        0: [r, g, b, 255],
                                    });
                                }
                            }
                        }
                    }

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
            }
        });
    }

    (quit, gui_info)
}

pub fn fill(x: u32, y: u32, img: &mut DynamicImage, color: Rgba<u8>) {
    let new_color = color;
    let initial_color = img.get_pixel(x, y);

    if new_color.eq(&initial_color) {
        println!("returned");
        return;
    }

    let height = img.height();
    let width = img.width();

    let mut cells: VecDeque<(u32, u32)> = VecDeque::new();

    cells.push_back((x, y));

    while let Some((x, y)) = cells.pop_front() {
        let cell = &mut img.get_pixel(x as u32, y as u32);

        if (*cell).eq(&new_color) {
            //println!("done");
            continue;
        }

        if (*cell).eq(&initial_color) {
            //println!("{:?}", new_color);
            img.put_pixel(x as u32, y as u32, new_color);

            let offsets: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (delta_x, delta_y) in offsets {
                let new_x = (x as i32).wrapping_add(delta_x) as u32;
                let new_y = (y as i32).wrapping_add(delta_y) as u32;

                if new_y < height && new_x < width {
                    cells.push_back((new_x, new_y));
                }
            }
        }
    }
}
