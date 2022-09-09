#[macro_use]
extern crate glium;

extern crate image;

use crate::geometry::vertex::Vertex;

use egui_winit::winit::event::{ElementState, MouseButton, MouseScrollDelta, KeyboardInput, ScanCode, ModifiersState};
use glium::{glutin, texture::SrgbTexture2d, Display, Frame, Program, Surface};

pub mod geometry;
pub mod gui;
pub mod info;
pub mod texture_manager;
pub mod data_displayer;

fn main() {
    let mut vertex_info = info::VertexShaderInfo {
        aspect: 0.0,
        zoom: 1.0,
        offset: [0.0, 0.0],
        init_camera: [0.0, 0.0],
        camera: [0.0, 0.0],
    };

    let mut input_info = info::InputInfo {
        scroll_delta: 0.0,
        left_mouse: false,
        control: false,
        drag_start: (0.0, 0.0),
        mouse_pos: (0.0, 0.0),

        zoom_modifier: 0.05,
    };

    let mut gui_info = info::GUIInfo {
        new_menu_opened: false,
    };

    let event_loop = glutin::event_loop::EventLoopBuilder::with_user_event().build();
    let display = create_display(&event_loop);

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);

    let vertex_shader_src = r#"
    #version 140
    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;

    uniform float aspect;
    uniform float zoom;
    uniform vec2 offset;

    void main() {
        v_tex_coords = tex_coords;
        gl_Position = vec4((position.x+offset.x)*zoom, (position.y*aspect+offset.y)*zoom, 0.0, 1.0);
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

    let image = texture_manager::get_texture(&display, &egui_glium.egui_ctx);

    let mut scroll = false;
    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            let mut quit = false;

            let repaint_after = egui_glium.run(&display, |egui_ctx| {
                let run_results = gui::run(egui_ctx, &input_info, gui_info);
                quit = run_results.0;
                gui_info = run_results.1;
                //egui_ctx.load_texture(name, image, filter);
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
                //get shader info
                if !scroll {
                    input_info.scroll_delta = 0.0;
                }
                
                vertex_info = info::collect_vertex_shader_info(vertex_info, &input_info, &display, &egui_glium);
                if scroll {
                    scroll = false
                }

                use glium::Surface as _;
                let mut target = display.draw();

                let color = egui::Rgba::from_rgb(0.1, 0.3, 0.2);
                target.clear_color(color[0], color[1], color[2], color[3]);

                // draw things behind egui here
                //let mut target = draw_things(&display, target, &program, &image, &vertex_info);
                let mut target = data_displayer::draw_things(&display, target, &program, &image.vertex_texture, &vertex_info);

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
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        println!("Received termination signal.");
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    /* The code to get the mouse position (And print it to the console) */
                    glutin::event::WindowEvent::CursorMoved { position, .. } => {
                        //println!("Mouse position: {:?}x{:?}", position.x as u16, position.y as u16);
                        input_info.mouse_pos.0 = position.x as f32;
                        input_info.mouse_pos.1 = position.y as f32;
                    }
                    // _ => return,
                    glutin::event::WindowEvent::MouseWheel { delta, .. } => {
                        if let MouseScrollDelta::LineDelta(_, y) = delta {
                            scroll = true;
                            input_info.scroll_delta = y;
                        }
                    }

                    glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                        if let MouseButton::Left = button {
                            if let ElementState::Pressed = state {
                                if input_info.left_mouse == false {
                                    input_info.drag_start = input_info.mouse_pos;
                                    vertex_info.init_camera[0] = vertex_info.camera[0];
                                    vertex_info.init_camera[1] = vertex_info.camera[1];
                                }

                                // vertex_info.init_offset[0] = vertex_info.offset[0];
                                // vertex_info.init_offset[1] = vertex_info.offset[1];

                                input_info.left_mouse = true;
                            } else {
                                input_info.left_mouse = false;
                            }
                        }
                    }

                    glutin::event::WindowEvent::ModifiersChanged(state) => {
                        if state.ctrl() {
                            input_info.control = true;
                        } else {
                            input_info.control = false;
                        }
                    }

                    _ => {
                        input_info.scroll_delta = 0.0;
                        return;
                    }
                }

                // important piece of egui code that allows egui to update
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
