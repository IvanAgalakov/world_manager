use egui::epaint::TextureManager;
use glium::{texture::SrgbTexture2d, Display, Frame, Program, Surface};

use crate::{
    geometry::{Vertex, Shape},
    info::{self, WorldInfo},
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
        let shape = Shape::new_rectangle(world_info.world_texture.as_ref().unwrap().gui_texture.aspect_ratio());
        
        //let shape = shape.vertices;
        let shape = &world_info.triangles;

        let vertex_buffer = glium::VertexBuffer::new(dis, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let texture = &world_info.world_texture.as_ref().unwrap().vertex_texture;

        let uniforms = uniform! {tex: texture, aspect: vertex_info.aspect, zoom: vertex_info.zoom, offset: vertex_info.offset};

        // &glium::uniforms::EmptyUniforms

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &pro,
                &uniforms,
                &params,
            )
            .unwrap();
    }

    return target;
}
