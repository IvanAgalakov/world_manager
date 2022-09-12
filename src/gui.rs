use std::{alloc::System, ops::Deref, path::PathBuf};

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

                    let mut rng = rand::thread_rng();

                    let mut start_x: i32 = -1;
                    let mut start_y: i32 = -1;
                    for x in 0..dyn_tex.width() {
                        for y in 0..dyn_tex.height() {
                            let pix = dyn_tex.get_pixel(x, y);
                            if pix.0[3] != 0 {
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
                        // let mut done = false;
                        // let mut x = start_x as u32;
                        // let mut y = start_y as u32;
                        // while !done {}
                        fill(
                            start_x as u32,
                            start_y as u32,
                            &mut dyn_tex,
                            Rgba {
                                0: [250, 0, 0, 255],
                            },
                            1,
                        );
                        // dyn_tex.put_pixel(3, 3, Rgba {
                        //             0: [250, 0, 0, 255],
                        //         });
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

pub fn fill(x: u32, y: u32, img: &mut DynamicImage, color: Rgba<u8>, num: u32) {
    let mut pixels_to_go: Vec<[u32; 2]> = Vec::new();
    pixels_to_go.push([x,y]);

    let mut num = 0;

    let mut pix_check: Vec<String> = Vec::new();
    pix_check.push(format!("({},{})", x, y));

    //println!("{} image width {} image height", img.width(), img.height());
    while num < pixels_to_go.len() {
        let x = pixels_to_go.get(num).unwrap()[0];
        let y = pixels_to_go.get(num).unwrap()[1];
        img.put_pixel(x, y, color);
        //println!("painted {},{}", x, y);
        for mx in -1..2 {
            for my in -1..2 {
                if !(mx == 0 && my == 0) {
                    let int_x = (x as i32) + mx;
                    let int_y = (y as i32) + my;

                    if int_x >= 0
                        && int_y >= 0
                        && int_x < img.width() as i32
                        && int_y < img.height() as i32
                    {
                        if img.get_pixel(int_x as u32, int_y as u32).0[3] != 0 {
                            if !pix_check.contains(&format!("({},{})", int_x, int_y)) {
                                pixels_to_go.push([int_x as u32, int_y as u32]);
                                pix_check.push(format!("({},{})", int_x, int_y));
                            }
                        }
                    }
                }
            }
        }
        num += 1;
        
    }
    println!("{}", num);
}
