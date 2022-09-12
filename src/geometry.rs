use egui::{Pos2, Vec2};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub(crate) position: [f32; 2],
    pub(crate) tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);
impl Vertex {
    pub fn as_vector(&self) -> Vec2 {
        Vec2 {
            x: self.position[0],
            y: self.position[1],
        }
    }
    pub fn as_pos(&self) -> Pos2 {
        Pos2 {
            x: self.position[0],
            y: self.position[1],
        }
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    pub(crate) start: Vertex,
    pub(crate) end: Vertex,
}

impl Line {
    pub fn get_intersection(&self, line: Line) -> Option<Vertex> {
        let start = self.start.as_vector();
        let end = self.end.as_vector();

        let a1 = end.y - start.y;
        let b1 = start.x - end.x;
        let c1 = a1 * start.x + b1 * start.y;

        let start = line.start.as_vector();
        let end = line.end.as_vector();
        let a2 = end.y - start.y;
        let b2 = start.x - end.x;
        let c2 = a2 * start.x + b2 * start.y;

        let delta = a1 * b2 - a2 * b1;

        let inter = Vec2 {
            x: (b2 * c1 - b1 * c2) / delta,
            y: (a1 * c2 - a2 * c1) / delta,
        };

        let mut ret = Some(inter);

        if !self.is_point_on_line(inter) {
            ret = None;
        }

        if ret.is_some() {
            let a = ret.unwrap();
            return Some (Vertex {
                position: [a.x, a.y],
                tex_coords: [a.x, a.y],
            });
        } else {
            return None;
        }
    }

    pub fn is_point_on_line(&self, p: Vec2) -> bool {
        return (p.to_pos2().distance(self.start.as_pos())
            + p.to_pos2().distance(self.end.as_pos()).abs()
            - self.get_length())
            < 0.0001;
    }

    pub fn get_length(&self) -> f32 {
        return self.start.as_pos().distance(self.end.as_pos());
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub(crate) vertices: Vec<Vertex>,
}

impl Shape {
    pub fn new_rectangle(aspect_ratio: f32) -> Self {
        let mut vertices = Vec::new();
        let mut sign_x = -1.0;
        let mut sign_y = 1.0;
        for numbers in 1..6 {
            vertices.push(Vertex {
                position: [sign_x * aspect_ratio, sign_y],
                tex_coords: [sign_x / 2.0 + 0.5, sign_y / 2.0 + 0.5],
            });
            if numbers == 3 {
                vertices.push(Vertex {
                    position: [sign_x * aspect_ratio, sign_y],
                    tex_coords: [sign_x / 2.0 + 0.5, sign_y / 2.0 + 0.5],
                });
            }
            if numbers % 2 == 1 {
                sign_y *= -1.0;
            } else {
                sign_x *= -1.0;
            }
        }

        Shape { vertices }
    }
}
