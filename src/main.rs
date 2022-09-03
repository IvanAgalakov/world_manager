#[macro_use]
extern crate glium;

use crate::geometry::vertex::Vertex;

use glium::{glutin, Surface, Display, Frame, Program};

pub mod geometry;

fn main() {

    

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();


    
    

    let vertex_shader_src = r#"
    #version 140
    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140
    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();

        // clear color
        target.clear_color(1.0, 1.0, 1.0, 1.0);


        let target = draw_things(&display, target, &program);


        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}

fn draw_things(dis:&Display, mut target:Frame, pro:&Program) -> Frame {
    let point1 = Vertex {position: [0.0, 1.0]};
    let point2 = Vertex {position: [-1.0, -1.0]};
    let point3 = Vertex {position: [1.0, -1.0]};

    let shape = vec![point1, point2, point3];

    let vertex_buffer = glium::VertexBuffer::new(dis, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    target.draw(&vertex_buffer, &indices, &pro, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

    return target;
}
