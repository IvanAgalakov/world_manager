use egui_glium::EguiGlium;
use glium::Display;
#[derive(Copy, Clone)]
pub struct VertexShaderInfo {
    pub(crate) aspect: f32,
    pub(crate) zoom: f32,
    pub(crate) offset: [f32; 2],
    pub(crate) init_camera: [f32; 2],
    pub(crate) camera: [f32; 2],
}

#[derive(Copy, Clone)]
pub struct InputInfo {
    pub(crate) scroll_delta: f32,
    pub(crate) left_mouse: bool,
    pub(crate) control: bool,
    pub(crate) drag_start: (f32, f32),
    pub(crate) mouse_pos: (f32, f32),

    pub(crate) zoom_modifier: f32,
}

pub fn collect_vertex_shader_info(mut vert: VertexShaderInfo, input: &InputInfo, display: &Display, egui_glium: &EguiGlium) -> VertexShaderInfo {
    let dimensions = display.get_framebuffer_dimensions();
    let dimensions = (dimensions.0 as f32, dimensions.1 as f32);
    if dimensions.1 > 0.0 {
        vert.aspect = dimensions.0 / dimensions.1;
    }

    //println!("the mouse pos is are {:?}", input.mouse_pos);

    vert.zoom += input.scroll_delta * input.zoom_modifier;
    if vert.zoom < 0.01 {
        vert.zoom = 0.01;
    }

    if input.left_mouse && !egui_glium.egui_ctx.wants_pointer_input() && input.control {
        vert.camera[0] = vert.init_camera[0]+(input.mouse_pos.0-input.drag_start.0)/vert.zoom;
        vert.camera[1] = vert.init_camera[1]+(input.mouse_pos.1-input.drag_start.1)/vert.zoom;
    }

    //println!("{}", input.scroll_delta);

    vert.offset[0] = vert.camera[0]/dimensions.0 * 2.0 - 1.0;
    vert.offset[1] = (1.0-vert.camera[1]/dimensions.1) * 2.0 - 1.0;

    return vert;
}
