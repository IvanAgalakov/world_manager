use std::ops::Deref;

use egui::epaint::TextureManager;
use glium::{texture::SrgbTexture2d, Display, Frame, Program, Surface};

use crate::{
    geometry::{Shape, Vertex},
    info::{self, WorldInfo},
    utils,
};

pub fn draw_things(
    dis: &Display,
    mut target: Frame,
    pro: &Program,
    vertex_info: &info::VertexShaderInfo,
    world_info: &WorldInfo,
) -> Frame {
    //let texture = glium::texture::SrgbTexture2d::new(dis, img).unwrap();
    //world_info.world_texture;
    if !world_info.world_texture.is_none() {
        let shape = Shape::new_rectangle(
            world_info
                .world_texture
                .as_ref()
                .unwrap()
                .gui_texture
                .aspect_ratio(),
        );

        let shape = shape.vertices;
        //let shape = &world_info.triangles;

        let vertex_buffer = glium::VertexBuffer::new(dis, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let texture = &world_info.world_texture.as_ref().unwrap().vertex_texture;

        let uniforms = uniform! {tex: texture, aspect: vertex_info.aspect, zoom: vertex_info.zoom, offset: vertex_info.offset, useTexture: true};

        // &glium::uniforms::EmptyUniforms

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            //point_size: Some(1.0),
            //polygon_mode: glium::PolygonMode::Fill,
            ..Default::default()
        };

        target
            .draw(&vertex_buffer, &indices, &pro, &uniforms, &params)
            .unwrap();
    }
    if !world_info.triangles.is_empty() {
        target = draw_triangles(dis, target, pro, vertex_info, &world_info.triangles);
    }

    return target;
}

fn draw_triangles(
    dis: &Display,
    mut target: Frame,
    pro: &Program,
    vertex_info: &info::VertexShaderInfo,
    triangles: &Vec<Vertex>
) -> Frame {

    
    let shape = triangles;

    let vertex_buffer = glium::VertexBuffer::new(dis, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //let texture = &world_info.world_texture.as_ref().unwrap().vertex_texture;

    let uniforms = uniform! {aspect: vertex_info.aspect, zoom: vertex_info.zoom, offset: vertex_info.offset, useTexture: false};

    // &glium::uniforms::EmptyUniforms

    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        //point_size: Some(1.0),
        //polygon_mode: glium::PolygonMode::Fill,
        ..Default::default()
    };

    target
        .draw(&vertex_buffer, &indices, &pro, &uniforms, &params)
        .unwrap();
    
    target
}
