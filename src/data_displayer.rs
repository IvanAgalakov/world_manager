use glium::{Display, Frame, Program, texture::SrgbTexture2d, Surface};

use crate::{info, geometry::vertex::Vertex};

pub fn draw_things(
    dis: &Display,
    mut target: Frame,
    pro: &Program,
    texture: &SrgbTexture2d,
    vertex_info: &info::VertexShaderInfo,
) -> Frame {
    let point1 = Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    };
    let point2 = Vertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    };
    let point3 = Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    };
    let point4 = Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    };
    let point5 = Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    };
    let point6 = Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    };

    let shape = vec![point1, point2, point3, point4, point5, point6];

    let vertex_buffer = glium::VertexBuffer::new(dis, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //let texture = glium::texture::SrgbTexture2d::new(dis, img).unwrap();

    let uniforms = uniform! {tex: texture, aspect: vertex_info.aspect, zoom: vertex_info.zoom, offset: vertex_info.offset};

    // &glium::uniforms::EmptyUniforms
    target
        .draw(
            &vertex_buffer,
            &indices,
            &pro,
            &uniforms,
            &Default::default(),
        )
        .unwrap();

    return target;
}