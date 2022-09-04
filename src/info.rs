use glium::Display;
#[derive(Copy, Clone)]
pub struct VertexShaderInfo {
    pub(crate) aspect: f32,
    pub(crate) zoom: f32,
    pub(crate) offset: [f32; 2],
}

pub fn collect_vertex_shader_info(mut vert: VertexShaderInfo, display: &Display) -> VertexShaderInfo {
    let dimensions = display.get_framebuffer_dimensions();
    if dimensions.1 > 0 {
        vert.aspect = (dimensions.0 as f32) / (dimensions.1 as f32);
    }

    

    return vert;
}
