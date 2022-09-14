use std::collections::VecDeque;

use egui::{Pos2, Vec2};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;


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

#[derive(Copy, Clone, Debug)]
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
            return Some(Vertex {
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

struct PixelIsland {
    pixel_coordinates: Vec<(u32, u32)>,
    top_right: (u32, u32),
    bottom_left: (u32, u32),
    my_color: Rgba<u8>,
}

pub fn generate_mesh_from_image(dyn_tex: &mut DynamicImage) -> Vec<Line> {
    let mut start_x: i32 = -1;
    let mut start_y: i32 = -1;
    for x in 0..dyn_tex.width() {
        for y in 0..dyn_tex.height() {
            let pix = dyn_tex.get_pixel(x, y);
            if pix.0[3] > 0 {
                dyn_tex.put_pixel(
                    x,
                    y,
                    Rgba {
                        0: [255, 255, 255, 255],
                    },
                );
                start_x = x as i32;
                start_y = y as i32;
            }
        }
    }

    let mut islands: Vec<PixelIsland> = Vec::new();

    if start_x != -1 {
        let mut used_colors = Vec::new();

        let red = Rgba {
            0: [255, 0, 0, 255],
        };

        let info = fill(start_x as u32, start_y as u32, dyn_tex, red);
        islands.push(PixelIsland {
            pixel_coordinates: info.0,
            top_right: info.1,
            bottom_left: info.2,
            my_color: red,
        });
        used_colors.push(red);

        let white = Rgba {
            0: [255, 255, 255, 255],
        };

        let mut rng = rand::thread_rng();

        for x in 0..dyn_tex.width() {
            for y in 0..dyn_tex.height() {
                if dyn_tex.get_pixel(x, y).eq(&white) {
                    let mut r = rng.gen_range(1..255);
                    let mut g = rng.gen_range(1..255);
                    let mut b = rng.gen_range(1..255);
                    let mut color = Rgba { 0: [r, g, b, 255] };
                    while used_colors.contains(&color) {
                        r = rng.gen_range(1..255);
                        g = rng.gen_range(1..255);
                        b = rng.gen_range(1..255);
                        color = Rgba { 0: [r, g, b, 255] };
                    }
                    used_colors.push(color);
                    let info = fill(x, y, dyn_tex, color);
                    islands.push(PixelIsland {
                        pixel_coordinates: info.0,
                        top_right: info.1,
                        bottom_left: info.2,
                        my_color: color,
                    });
                }
            }
        }
    }

    let width = dyn_tex.width();
    let height = dyn_tex.height();
    let mut lines = Vec::new();
    //let mut all_arranged = Vec::new();
    //println!("{}", islands.len());
    for mut island in islands {
        let mut start = (0, 0);
        for x in island.bottom_left.0..island.top_right.0 {
            for y in island.bottom_left.1..island.top_right.1 {
                if dyn_tex.get_pixel(x, y).eq(&island.my_color) {
                    start = (x, y);
                }
            }
        }
        let mut done = false;
        let mut arranged_pixels: Vec<(u32, u32)> = Vec::new();
        let mut x = start.0;
        let mut y = start.1;
        while !done {
            arranged_pixels.push(start);
            let offsets: Vec<(i32, i32)> = vec![
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
            ];
            let mut found_path = false;
            for (delta_x, delta_y) in offsets {
                let new_x = x as i32 + delta_x;
                let new_y = y as i32 + delta_y;
                if new_x < 0 || new_x >= width as i32 {
                    continue;
                }
                if new_y < 0 || new_y >= height as i32 {
                    continue;
                }
                let new_x = new_x as u32;
                let new_y = new_y as u32;
                if dyn_tex.get_pixel(new_x, new_y).0[3] > 0
                    && !arranged_pixels.contains(&(new_x, new_y))
                    && island.pixel_coordinates.contains(&(new_x, new_y))
                {
                    x = new_x;
                    y = new_y;
                    arranged_pixels.push((x, y));
                    found_path = true;
                    break;
                }
            }
            if !found_path {
                done = true;
            }
        }

        // //let mut points = Vec::new();
        // let mut triang: DelaunayTriangulation<Point2<f64>> = DelaunayTriangulation::new();
        // for pixel in &arranged_pixels {
        //     triang.insert(Point2 { x: (pixel.0 as f64) / (width as f64), y: (pixel.1 as f64) / (height as f64),});
        // }
        // let mut points = Vec::new();
        // for face in triang.inner_faces() {
        //     for vert in face.vertices() {
        //         let x = vert.position().x as f32;
        //         let y = 1.0 - vert.position().y as f32;
        //         points.push(Vertex {
        //             position: [x, y],
        //             tex_coords: [x, y],
        //         });
        //     }
        // }

        // triangles.append(&mut points);
        if arranged_pixels.len() > 0 {
            let first_coord = arranged_pixels.get(0);
            for i in 0..arranged_pixels.len() {
                let x = (arranged_pixels.get(i).unwrap().0/width) as f32;
                let y = (arranged_pixels.get(i).unwrap().1/height) as f32;
                if i+1 < arranged_pixels.len() {
                    let x2 = (arranged_pixels.get(i+1).unwrap().0/width) as f32;
                    let y2 = (arranged_pixels.get(i+1).unwrap().1/height) as f32;
                    lines.push(Line {
                        start: Vertex { position: [x,y], tex_coords: [x,y] },
                        end: Vertex { position: [x2,y2], tex_coords: [x2,y2] }
                    });
                } else {
                    let x2 = (arranged_pixels.get(0).unwrap().0/width) as f32;
                    let y2 = (arranged_pixels.get(0).unwrap().1/height) as f32;
                    lines.push(Line {
                        start: Vertex { position: [x,y], tex_coords: [x,y] },
                        end: Vertex { position: [x2,y2], tex_coords: [x2,y2] }
                    });
                }
            }
        }


        island.pixel_coordinates = arranged_pixels;
    }

    lines
}

pub fn fill(
    x: u32,
    y: u32,
    img: &mut DynamicImage,
    color: Rgba<u8>,
) -> (Vec<(u32, u32)>, (u32, u32), (u32, u32)) {
    let new_color = color;
    let initial_color = img.get_pixel(x, y);

    if new_color.eq(&initial_color) {
        //println!("returned");
        return (Vec::new(), (0, 0), (0, 0));
    }

    let height = img.height();
    let width = img.width();

    let mut cells: VecDeque<(u32, u32)> = VecDeque::new();

    let mut pixels = Vec::new();

    cells.push_back((x, y));
    pixels.push((x, y));
    let mut top_right = (x, y);
    let mut bottom_left = (x, y);
    while let Some((x, y)) = cells.pop_front() {
        let cell = &mut img.get_pixel(x as u32, y as u32);

        if (*cell).eq(&new_color) {
            //println!("done");
            continue;
        }

        if (*cell).eq(&initial_color) {
            //println!("{:?}", new_color);
            img.put_pixel(x as u32, y as u32, new_color);
            if top_right.0 < x {
                top_right.0 = x;
            }
            if top_right.1 < y {
                top_right.1 = y;
            }
            if bottom_left.0 > x {
                bottom_left.0 = x;
            }
            if bottom_left.1 > y {
                bottom_left.1 = y;
            }

            let offsets: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (delta_x, delta_y) in offsets {
                let new_x = (x as i32).wrapping_add(delta_x) as u32;
                let new_y = (y as i32).wrapping_add(delta_y) as u32;

                if new_y < height && new_x < width {
                    cells.push_back((new_x, new_y));
                    if img.get_pixel(new_x, new_y).0[3] == 0 {
                        pixels.push((x, y));
                    }
                } else {
                    pixels.push((x, y));
                }
            }
        }
    }

    (pixels, top_right, bottom_left)
}
