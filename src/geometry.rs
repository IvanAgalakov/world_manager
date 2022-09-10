#[derive(Copy, Clone,Debug)]
pub struct Vertex {
    pub(crate) position: [f32; 2],
    pub(crate) tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

#[derive(Clone,Debug)]
pub struct Shape {
    pub(crate) vertices: Vec<Vertex>,
}

impl Shape {
    pub fn new_rectangle(aspect_ratio: f32) -> Self {
        let mut vertices = Vec::new();
        let mut sign_x = -1.0;
        let mut sign_y = 1.0;
        for numbers in 1..6 {
            vertices.push(
                Vertex { position: [sign_x*aspect_ratio, sign_y], tex_coords: [sign_x/2.0+0.5, sign_y/2.0+0.5] }
            );
            if numbers == 3 {
                vertices.push(
                    Vertex { position: [sign_x*aspect_ratio, sign_y], tex_coords: [sign_x/2.0+0.5, sign_y/2.0+0.5] }
                );
            }
            if numbers%2==1 {
                sign_y *= -1.0;
            } else {
                sign_x *= -1.0;
            }
        }
        
        Shape {
            vertices
        }
    }
}