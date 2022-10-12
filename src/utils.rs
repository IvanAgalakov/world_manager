use egui::Vec2;

use crate::geometry::{Line, Vertex};

pub fn normal_point_to_point(p: Vec2, rise: f32, run: f32, deviate: f32) -> Vec2{
    let hypo = ((rise * rise) + (run * run)).sqrt();
    // System.out.println(p.toString() + " | " + -run + " | " + rise + " | " + hypo + " | " + deviate);

    return get_point_along_line(p, -run, rise, hypo, deviate);
}

pub fn get_point_along_line(start: Vec2, rise: f32, run: f32, hypo: f32, deviate: f32) -> Vec2{
    let run_sq = run.powf(2.0);
    let rise_sq = rise.powf(2.0);
    let formula = deviate.signum() * (deviate.powf(2.0) / (run_sq + rise_sq)).sqrt();

    return Vec2 {x: start.x + run * formula, y: start.y + rise * formula};
}

pub fn vertices_from_lines(thickness: f32, lines: &Vec<Line>) -> Vec<Vertex>{

    let mut points = Vec::new();
    for i in 0..lines.len() {
        let cur_line = lines.get(i).unwrap();
        let top_left = normal_point_to_point(cur_line.get_start().as_vector(), cur_line.get_rise(), cur_line.get_run(), thickness / 2.0);
        let bot_left = normal_point_to_point(cur_line.get_start().as_vector(), cur_line.get_rise(), cur_line.get_run(), -thickness / 2.0);
        let top_right = normal_point_to_point(cur_line.get_end().as_vector(), cur_line.get_rise(), cur_line.get_run(), thickness / 2.0);
        let bot_right = normal_point_to_point(cur_line.get_end().as_vector(), cur_line.get_rise(), cur_line.get_run(), -thickness / 2.0);
        //lines.insert(visualPoints, topLeft, botLeft, botRight, botRight, topR, topLeft);
        points.push(Vertex::from_vector(top_left));
        points.push(Vertex::from_vector(bot_left));
        points.push(Vertex::from_vector(bot_right));
        points.push(Vertex::from_vector(bot_right));
        points.push(Vertex::from_vector(top_right));
        points.push(Vertex::from_vector(top_left));
    }
    points
}

pub fn vertices_from_line_points(lines: &Vec<Line>) -> Vec<Vertex>{
    let mut points = Vec::new();
    for line in lines {
        points.push(line.get_start());
        points.push(line.get_end());
    }
    points
}



pub fn screen_point_to_world_point(screen: Vertex, width: u32, height: u32, zoom: f32, normx: f32, normy: f32, aspect: f32) -> Vertex {
    let screen = screen.as_pos();
    let x = -(-1.0 + 1.0 / zoom) + (((screen.x / width as f32) * 2.0) / zoom - (normx * 2.0));
    let y = ((-1.0 + 1.0 / zoom) / aspect - (((screen.y / height as f32) * 2.0) / aspect / zoom + (normy * 2.0 - 2.0) / aspect));
    Vertex{position: [x,y], tex_coords: [x,y]}
}

pub fn world_point_to_screen_point(world: Vertex, width: u32, height: u32, zoom: f32, normx: f32, normy: f32, aspect: f32) -> Vertex {
    let world = world.as_pos();
    let x = (((world.x + (-1.0 + 1.0 / zoom) + (normx * 2.0)) * zoom) / 2.0) * width as f32;
    let y = -((((world.y - (-1.0 + 1.0 / zoom) / aspect + (normy * 2.0 - 2.0) / aspect) * zoom) * aspect / 2.0) * height as f32);
    Vertex{position: [x,y], tex_coords: [x,y]}
}