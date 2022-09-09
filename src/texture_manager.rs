use std::path::Path;

use egui::{Context, ColorImage, TextureHandle};
use glium::{Display, texture::SrgbTexture2d};

pub struct TextureData {
    pub(crate) vertex_texture: SrgbTexture2d,
    pub(crate) gui_texture: TextureHandle,
}


pub fn get_texture(dis: &Display, egui_ctx: &Context) -> TextureData {
    let image = image::open(&Path::new("C:/Users/Ivan/Documents/test.png")).unwrap();
    let size = [image.width() as _, image.height() as _];
    let image = image.to_rgba8();
    let pixels = image.as_flat_samples();
    let eg_texture = egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    );

    let image_dimensions = image.dimensions();

    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::SrgbTexture2d::new(dis, image).unwrap();

    //let eg_texture = ColorImage::from_rgba_unmultiplied(image_dimensions, rgba);

    let handle = egui_ctx.load_texture("test", eg_texture, egui::TextureFilter::Linear);
    
   // return texture;
   TextureData { vertex_texture: texture, gui_texture: handle }
}