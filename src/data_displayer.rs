use std::ops::Deref;

use egui::epaint::TextureManager;
use glium::{texture::SrgbTexture2d, Display, Frame, Program, Surface};

use crate::{
    geometry::{Shape, Vertex, Line},
    info::{self, WorldInfo, InputInfo},
    utils, constants,
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
    // if !world_info.lines.is_empty() {
    //     let fill = [world_info.bottom_right.0, vertex_info.mouse_pos.position[1]];
    //     let end = Vertex{position: fill, tex_coords: fill};
    //     let fill = [world_info.top_left.0, vertex_info.mouse_pos.position[1]];
    //     let start = Vertex{position: fill, tex_coords: fill};
    //     let horiz_line = Line{start: start, end: end};
    //     let mut lines = Vec::new();
    //     let mut intersections = Vec::new();

    //     //lines.push(horiz_line);
    //     for l in &world_info.lines {
    //         if l.get_intersection(horiz_line).is_some() {
    //             //lines.push(l.clone());
    //             intersections.push(l.get_intersection(horiz_line).unwrap());
    //         }
    //     }
    //     intersections.sort_by(|a, b| a.get_x().total_cmp(&b.get_x()));
    //     println!("Length before dedup {}", intersections.len());
    //     intersections.dedup_by(|a, b| a.get_x()==b.get_x());
    //     println!("Length after dedup {}", intersections.len());
    //     // for i in 0..intersections.len() {
    //     //     if i < intersections.len()-1 {
    //     //         // if intersections[i].position[0] - intersections[i+1].position[0] < constants::PRECISION {
    //     //         //     intersections.remove(i+1);
    //     //         // }
    //     //     }
    //     // }
    //     for i in 0..intersections.len() {
    //         if i%2 == 0 && i < intersections.len()-1{
    //             lines.push(Line::new(intersections[i], intersections[i+1]));
    //         }
    //     }
        
    //     let triangles = utils::vertices_from_lines(0.01, &lines);
    //     target = draw_triangles(dis, target, pro, vertex_info, &triangles);
    // }

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
