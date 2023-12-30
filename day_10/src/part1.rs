use std::fs;
use std::path::Path;

use crate::entity::{Direction, Landscape, LandscapeField, Point};
use crate::parser;

fn check_start_direction(landscape: &Landscape, direction: Direction) -> bool {
    let point = direction.move_point(&landscape.start);
    let field = landscape.get_field_checked(&point);
    if field.is_none() {
        return false;
    }
    let field = field.unwrap();
    match field {
        LandscapeField::Pipe(pipe) => pipe.enter_checked(&direction).is_some(),
        _ => false,
    }
}

fn get_start_directions(landscape: &Landscape) -> (Direction, Direction) {
    let mut directions = [Direction::North, Direction::South, Direction::East, Direction::West]
        .iter()
        .filter(|direction| check_start_direction(landscape, **direction));

    (*directions.next().unwrap(), *directions.next().unwrap())
}

fn make_step(landscape: &Landscape, direction: Direction, point: Point) -> (Direction, Point) {
    let point = direction.move_point(&point);
    let field = landscape.get_field(&point);
    match field {
        LandscapeField::Pipe(pipe) => {
            let new_direction = pipe.enter(&direction);
            (new_direction, point)
        }
        _ => panic!("Invalid field: {:?}", field),
    }
}


pub fn execute<P: AsRef<Path>>(path: P, name: &str, print: bool) -> u32 {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let landscape = parser::parse(&contents)
        .expect("Should have been able to parse the input");


    let (mut direction_a, mut direction_b) = get_start_directions(&landscape);
    let (mut point_a, mut point_b) = (landscape.start, landscape.start);
    let mut visited: Vec<u16> = vec![0; landscape.fields.len() * landscape.fields[0].len()];
    let mut steps = 0;
    loop {
        steps += 1;
        (direction_a, point_a) = make_step(&landscape, direction_a, point_a);
        (direction_b, point_b) = make_step(&landscape, direction_b, point_b);
        if visited[landscape.hash_point(&point_a)] != 0 {
            break;
        }

        visited[landscape.hash_point(&point_a)] = steps;
        visited[landscape.hash_point(&point_b)] = steps;
    }

    let result = steps - 1;
    if print {
        println!("{} Result: {}", name, result);
    }

    result as u32
}
