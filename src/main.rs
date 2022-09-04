#[macro_use]
extern crate glium;

extern crate image;

use std::{u8, path::Path};

use crate::geometry::vertex::Vertex;

use glium::{glutin, Display, Frame, Program, Surface, texture::RawImage2d};
use image::DynamicImage;

pub mod geometry;
pub mod gui;

fn draw_things(dis: &Display, mut target: Frame, pro: &Program, img: RawImage2d<u8>) -> Frame {
    let point1 = Vertex {
        position: [-1.0, 1.0],
        tex_coords: [-1.0, 1.0],
    };
    let point2 = Vertex {
        position: [-1.0, -1.0],
        tex_coords: [-1.0, -1.0],
    };
    let point3 = Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, -1.0],
    };
    let point4 = Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, -1.0],
    };
    let point5 = Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    };
    let point6 = Vertex {
        position: [-1.0, 1.0],
        tex_coords: [-1.0, 1.0],
    };

    let shape = vec![point1, point2, point3, point4, point5, point6];

    let vertex_buffer = glium::VertexBuffer::new(dis, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    
    let texture = glium::texture::SrgbTexture2d::new(dis, img).unwrap();

    let uniforms = uniform! {tex: &texture};

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

fn get_texture() -> RawImage2d<'static, u8> {
    let image = image::open(&Path::new("C:/Users/Ivan/Documents/test.png")).unwrap();
    let image = image.to_rgba8();
    let image_dimensions = image.dimensions();

    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    
    return image;
}


fn main() {


    

    let event_loop = glutin::event_loop::EventLoopBuilder::with_user_event().build();
    let display = create_display(&event_loop);

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);

    let vertex_shader_src = r#"
    #version 140
    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;

    void main() {
        v_tex_coords = tex_coords;
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140
    in vec2 v_tex_coords;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        //color = vec4(1.0, 0.0, 0.0, 1.0);
        color = texture(tex, v_tex_coords);
    }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            let mut quit = false;

            let repaint_after = egui_glium.run(&display, |egui_ctx| {
                quit = gui::run(egui_ctx);
            });

            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if repaint_after.is_zero() {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else if let Some(repaint_after_instant) =
                std::time::Instant::now().checked_add(repaint_after)
            {
                glutin::event_loop::ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                use glium::Surface as _;
                let mut target = display.draw();

                let color = egui::Rgba::from_rgb(0.1, 0.3, 0.2);
                target.clear_color(color[0], color[1], color[2], color[3]);

                // draw things behind egui here
                let image = get_texture();
                let mut target = draw_things(&display, target, &program, image);

                egui_glium.paint(&display, &mut target);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                let event_response = egui_glium.on_event(&event);

                if event_response {
                    display.gl_window().window().request_redraw();
                }
            }
            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached {
                ..
            }) => {
                display.gl_window().window().request_redraw();
            }
            _ => (),
        }
    });
}

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title("World Builder");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}
