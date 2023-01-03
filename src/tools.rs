use std::collections::HashMap;

use super::grid::Grid;
use super::location::Location;

pub fn euclidean(a: &Location, b: &Location) -> f64 {
    f64::sqrt(((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)) as f64)
}

pub fn manhattan(a: &Location, b: &Location) -> f64 {
    (i32::abs(a.x - b.x) + i32::abs(a.y - b.y)) as f64
}

pub fn reconstruct_path(
    start: &Location,
    goal: &Location,
    came_from: &HashMap<Location, Location>,
) -> Vec<Location> {
    let mut path: Vec<Location> = Vec::new();
    let mut current = goal;
    while current != start {
        path.push(*current);
        match came_from.get(current) {
            Some(next) => {
                current = next;
            }
            _ => return Vec::new(), // No solution, return empty path
        }
    }
    path.reverse();
    path
}

pub fn draw_grid(
    grid: &Grid,
    distances: Option<&HashMap<Location, f64>>,
    point_to: Option<&HashMap<Location, Location>>,
    path: &Vec<Location>,
    came_from: &HashMap<Location, Location>,
    start: Option<&Location>,
    goal: Option<&Location>,
) {
    let field_width: usize = 3;
    println!("{}", "_".repeat(field_width * grid.get_width() as usize));
    for y in 0..grid.get_height() {
        for x in 0..grid.get_width() {
            let id = Location {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            if grid.walls.contains(&id) {
                print!("{}", "#".repeat(field_width));
            } else if start.is_some() && &id == start.unwrap() {
                print!("{:^field_width$}", "A");
            } else if goal.is_some() && &id == goal.unwrap() {
                print!("{:^field_width$}", "Z");
            } else if path.contains(&id) {
                print!("{:^field_width$}", "@");
            } else if came_from.contains_key(&id) {
                print!("{:^field_width$}", "J");
            } else if point_to.is_some() && point_to.unwrap().contains_key(&id) {
                let next = point_to.unwrap().get(&id).unwrap();
                if next.x == (x + 1) as i32 {
                    print!("{:^field_width$}", ">")
                } else if next.x == (x - 1) as i32 {
                    print!("{:^field_width$}", "<")
                } else if next.y == (y + 1) as i32 {
                    print!("{:^field_width$}", "v")
                } else if next.y == (y - 1) as i32 {
                    print!("{:^field_width$}", "^")
                } else {
                    print!("{:^field_width$}", "*")
                }
            } else if distances.is_some() && distances.unwrap().contains_key(&id) {
                print!("{:<field_width$}", " ");
            } else {
                print!("{:^field_width$}", ".");
            }
        }
        println!();
    }
    println!("{}", "~".repeat(field_width * grid.get_width() as usize));
}

//              else if(distances.count(id)){
//                  cout << ' ' << left << setw(field_width - 1) << distances.at(id);
//              }
