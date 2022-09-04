use std::path::Path;

use glium::{Display, texture::SrgbTexture2d};

pub fn get_texture(dis: &Display) -> SrgbTexture2d {
    let image = image::open(&Path::new("C:/Users/Ivan/Documents/map.png")).unwrap();
    let image = image.to_rgba8();
    let image_dimensions = image.dimensions();

    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::SrgbTexture2d::new(dis, image).unwrap();
    
    return texture;
}