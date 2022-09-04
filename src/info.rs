use glium::Display;
#[derive(Copy, Clone)]
pub struct VertexShaderInfo {
    pub(crate) aspect: f32,
    pub(crate) zoom: f32,
    pub(crate) offset: [f32; 2],
    pub(crate) init_offset: [f32; 2],
}

#[derive(Copy, Clone)]
pub struct InputInfo {
    pub(crate) scroll_delta: f32,
    pub(crate) middle_mouse: bool,
    pub(crate) drag_start: (f32, f32),
    pub(crate) mouse_pos: (f32, f32),
}

pub fn collect_vertex_shader_info(mut vert: VertexShaderInfo, input: &InputInfo, display: &Display) -> VertexShaderInfo {
    let dimensions = display.get_framebuffer_dimensions();
    if dimensions.1 > 0 {
        vert.aspect = (dimensions.0 as f32) / (dimensions.1 as f32);
    }

    vert.zoom += input.scroll_delta * 0.05;

    if input.middle_mouse {
        vert.offset[0] = vert.init_offset[0] + (input.mouse_pos.0 - input.drag_start.0)/100.0;
        vert.offset[1] = vert.init_offset[1] + -(input.mouse_pos.1 - input.drag_start.1)/100.0;
    }

    //println!("{}", input.scroll_delta);

    return vert;
}
