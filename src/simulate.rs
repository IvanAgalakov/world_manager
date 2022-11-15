use crate::geometry::{Line, Vertex};

pub fn simulate_ocean_flows(horiz: Line, collision: Vec<Line>) -> Vec<Line> {
    let mut flow: Vec<Line> = Vec::new();
    let steps = 1000;
    let distance = 0.01;

    let mut fl = horiz.clone();

    let points :Vec<Vertex> = Vec::new();
    for i in 0..steps {
        let mut intersect = None;
        for line in &collision {
            if line.get_intersection(fl).is_some() {
                intersect = Some((line,line.get_intersection(fl).unwrap()));
                break;
            }
        }
        if intersect.is_some() {
            let intersect = intersect.unwrap();
            let line = intersect.0;
            let point = intersect.1;

            let angle_of_line = line.start.get_angle_to(&line.end);
            let my_angle = fl.start.get_angle_to(&fl.end);

            if my_angle < angle_of_line {
                fl.end.rotate_around(&fl.start, distance);
            } else if my_angle > angle_of_line {
                fl.end.rotate_around(&fl.start, -distance);
            }
            
        }
    } 



    return flow;
}