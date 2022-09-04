#[derive(Copy, Clone)]
pub struct Vertex {
    pub(crate) position: [f32; 2],
    pub(crate) tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);