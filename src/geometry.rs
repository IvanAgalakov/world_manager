use std::cmp::Ordering;
use std::collections::VecDeque;

use egui::{Pos2, Vec2};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;

use crate::constants;
use crate::utils;

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
    pub fn from_vector(vec: Vec2) -> Vertex {
        Vertex {
            position: [vec.x, vec.y],
            tex_coords: [vec.x, vec.y],
        }
    }
    pub fn get_x(&self) -> f32 {
        return self.position[0];
    }
    pub fn get_y(&self) -> f32 {
        return self.position[0];
    }

    pub fn eq(&self, other: Vertex) -> bool {
        (self.get_x() - other.get_x()).abs() <= constants::PRECISION
            && (self.get_y() - other.get_y()).abs() <= constants::PRECISION
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub(crate) start: Vertex,
    pub(crate) end: Vertex,
}

impl Line {
    pub fn get_intersection(&self, line: Line) -> Option<Vertex> {
        if self.get_slope() == line.get_slope() {
            return  None;
        }

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
            < constants::PRECISION;
    }

    pub fn get_length(&self) -> f32 {
        return self.start.as_pos().distance(self.end.as_pos());
    }

    pub fn get_rise(&self) -> f32 {
        return self.end.as_vector().y - self.start.as_vector().y;
    }

    pub fn get_run(&self) -> f32 {
        return self.end.as_vector().x - self.start.as_vector().x;
    }

    pub fn get_rise_and_run(&self) -> (f32, f32) {
        (self.get_rise(), self.get_run())
    }

    pub fn get_slope(&self) -> f32 {
        return self.get_rise()/self.get_run();
    }

    pub fn get_start(&self) -> Vertex {
        return self.start;
    }

    pub fn get_end(&self) -> Vertex {
        return self.end;
    }

    pub fn new(start: Vertex, end: Vertex) -> Self {
        Line {
            start: start,
            end: end,
        }
    }

    pub fn new_from_rise_run(start: Vertex, rise: f32, run: f32) -> Self {
        let fill = [start.position[0] + run, start.position[1] + rise];
        let end = Vertex {
            position: fill,
            tex_coords: fill,
        };
        Line {
            start: start,
            end: end,
        }
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
}

pub fn generate_mesh_from_image(dyn_tex: &mut DynamicImage) -> Vec<Line> {
    let mut start_x: i32 = -1;
    let mut start_y: i32 = -1;
    for x in 0..dyn_tex.width() {
        for y in 0..dyn_tex.height() {
            let pix = dyn_tex.get_pixel(x, y);
            if pix.0[3] != 0 {
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
                    });
                }
            }
        }
    }

    let width = dyn_tex.width();
    let height = dyn_tex.height();
    let aspect = (width as f32) / (height as f32);
    println!("{}", aspect);
    let mut lines: Vec<Line> = Vec::new();
    //let mut all_arranged = Vec::new();
    //println!("{}", islands.len());
    for island in islands {
        //println!("pix coord: {:?}", island.pixel_coordinates.len());
        let mut island_lines = Vec::new();
        for pixel in island.pixel_coordinates {
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
            for (delta_x, delta_y) in offsets {
                let new_x = pixel.0 as i32 + delta_x;
                let new_y = pixel.1 as i32 + delta_y;
                if new_x < 0 || new_x >= width as i32 {
                    continue;
                }
                if new_y < 0 || new_y >= height as i32 {
                    continue;
                }
                let new_x = new_x as u32;
                let new_y = new_y as u32;
                if dyn_tex.get_pixel(new_x, new_y).0[3] > 0 {
                    let x = (pixel.0 as f32 / width as f32) * 2.0 - 1.0;
                    let y = -(pixel.1 as f32 / height as f32) * 2.0 + 1.0;
                    let new_x = (new_x as f32 / width as f32) * 2.0 - 1.0;
                    let new_y = -(new_y as f32 / height as f32) * 2.0 + 1.0;
                    let line = Line {
                        start: Vertex {
                            position: [x * aspect, y],
                            tex_coords: [x, y],
                        },
                        end: Vertex {
                            position: [new_x * aspect, new_y],
                            tex_coords: [new_x, new_y],
                        },
                    };

                    island_lines.push(line);
                    //lines.push();
                }
            }
        }



        // let mut resolution: usize = 3;
        // let mut comp_lines = Vec::new();
        // let mut start: Option<Vertex> = None;

        // if resolution as f32 > (island_lines.len() as f32 / 3.0) {
        //     resolution = (island_lines.len() as f32 / 3.0).floor() as usize;
        //     if resolution <= 0 {
        //         resolution = 1;
        //     }
        // }
        // for i in 0..island_lines.len() {
        //     if start.is_none() {
        //         start = Some(island_lines[i].start.clone());
        //     }
        //     if (i + 1) % resolution == 0 || i == (island_lines.len() - 1) {
        //         let line = Line::new(start.unwrap().clone(), island_lines[i].end.clone());
        //         start = Some(line.end.clone());
        //         comp_lines.push(line);
        //     }
        // }

        lines.append(&mut island_lines);
    }
    //println!("{:?}", arranged_pixels);
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
    //println!("arr length: {:?} height: {} width: {}",arranged_pixels, height, width);
    // if arranged_pixels.len() > 0 {
    //     for i in 0..arranged_pixels.len() {
    //         let x = (arranged_pixels.get(i).unwrap().0 as f32/width as f32);
    //         let y = -(arranged_pixels.get(i).unwrap().1 as f32/height as f32);
    //         if i+1 < arranged_pixels.len() {
    //             let x2 = (arranged_pixels.get(i+1).unwrap().0 as f32/width as f32);
    //             let y2 = -(arranged_pixels.get(i+1).unwrap().1 as f32/height as f32);
    //             lines.push(Line {
    //                 start: Vertex { position: [x,y], tex_coords: [x,y] },
    //                 end: Vertex { position: [x2,y2], tex_coords: [x2,y2] }
    //             });
    //         } else {
    //             let x2 = (arranged_pixels.get(0).unwrap().0 as f32/width as f32);
    //             let y2 = -(arranged_pixels.get(0).unwrap().1 as f32/height as f32);
    //             lines.push(Line {
    //                 start: Vertex { position: [x,y], tex_coords: [x,y] },
    //                 end: Vertex { position: [x2,y2], tex_coords: [x2,y2] }
    //             });
    //         }
    //     }
    // }

    // println!("{} lines before optimization", lines.len());
    // //optimize lines
    // for i in 0..lines.len() {
    //     for mut x in 0..lines.len() {
    //         if lines.get(i).is_some() && lines.get(x).is_some() {
    //             let mut my_line = lines[i];
    //             let mut check_line = lines[x];
    //             if my_line.is_point_on_line(check_line.get_start().as_vector()) && my_line.is_point_on_line(check_line.get_end().as_vector()) {
    //                 let dis_my = my_line.start.as_pos().distance(my_line.end.as_pos());
    //                 let dis_their = my_line.start.as_pos().distance(check_line.end.as_pos());
    //                 let dis_between = my_line.end.as_pos().distance(check_line.end.as_pos());
    //                 if dis_between - (dis_my+dis_their) < constants::PRECISION {
    //                     if dis_my < dis_their {
    //                         my_line.end = check_line.end;
    //                         lines.remove(x);
    //                         if (x > 0) {
    //                             x -= 1;
    //                         }
    //                     }
    //                 }

    //                 let dis_my = my_line.end.as_pos().distance(my_line.start.as_pos());
    //                 let dis_their = my_line.end.as_pos().distance(check_line.start.as_pos());
    //                 let dis_between = my_line.start.as_pos().distance(check_line.start.as_pos());
    //                 if dis_between - (dis_my+dis_their) < constants::PRECISION {
    //                     if dis_my < dis_their {
    //                         my_line.start = check_line.start;
    //                         lines.remove(x);
    //                         if (x > 0) {
    //                             x -= 1;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // println!("{} lines after optimization", lines.len());

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
    //pixels.push((x, y));
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
                    if img.get_pixel(new_x, new_y).0[3] == 0 && !pixels.contains(&(x, y)) {
                        pixels.push((x, y));
                    }
                } else {
                    if !pixels.contains(&(x, y)) {
                        pixels.push((x, y));
                    }
                }
            }
        }
    }

    (pixels, top_right, bottom_left)
}
