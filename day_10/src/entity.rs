pub type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
pub enum Pipe {
    Vertical,
    Horizontal,
    CornerBottomRight,
    CornerBottomLeft,
    CornerTopRight,
    CornerTopLeft,
}

#[derive(Debug)]
pub enum LandscapeField {
    Pipe(Pipe),
    Start,
    Empty,
}

pub struct Landscape {
    pub fields: Vec<Vec<LandscapeField>>,
    pub start: (usize, usize),
}

impl Pipe {
    pub fn enter(&self, from: &Direction) -> Direction {
        match self {
            Pipe::Vertical => from.clone(),
            Pipe::Horizontal => from.clone(),
            Pipe::CornerBottomRight => match from {
                Direction::South => from.turn_right(),
                Direction::East => from.turn_left(),
                _ => panic!("Invalid direction for CornerBottomRight: {:?}", from),
            },
            Pipe::CornerBottomLeft => match from {
                Direction::South => from.turn_left(),
                Direction::West => from.turn_right(),
                _ => panic!("Invalid direction for CornerBottomLeft: {:?}", from),
            },
            Pipe::CornerTopRight => match from {
                Direction::North => from.turn_left(),
                Direction::East => from.turn_right(),
                _ => panic!("Invalid direction for CornerTopRight: {:?}", from),
            },
            Pipe::CornerTopLeft => match from {
                Direction::North => from.turn_right(),
                Direction::West => from.turn_left(),
                _ => panic!("Invalid direction for CornerTopLeft: {:?}", from),
            },
        }
    }

    pub fn enter_checked(&self, from: &Direction) -> Option<Direction> {
        match self {
            Pipe::Vertical => Some(from.clone()),
            Pipe::Horizontal => Some(from.clone()),
            Pipe::CornerBottomRight => match from {
                Direction::South => Some(from.turn_right()),
                Direction::East => Some(from.turn_left()),
                _ => None
            },
            Pipe::CornerBottomLeft => match from {
                Direction::South => Some(from.turn_left()),
                Direction::West => Some(from.turn_right()),
                _ => None,
            },
            Pipe::CornerTopRight => match from {
                Direction::North => Some(from.turn_left()),
                Direction::East => Some(from.turn_right()),
                _ => None,
            },
            Pipe::CornerTopLeft => match from {
                Direction::North => Some(from.turn_right()),
                Direction::West => Some(from.turn_left()),
                _ => None,
            },
        }
    }
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn move_point(&self, point: &Point) -> Point {
        match self {
            Direction::North => (point.0, point.1 - 1),
            Direction::South => (point.0, point.1 + 1),
            Direction::East => (point.0 + 1, point.1),
            Direction::West => (point.0 - 1, point.1),
        }
    }
}

impl Landscape {
    pub fn get_field(&self, point: &Point) -> &LandscapeField {
        &self.fields[point.1][point.0]
    }

    pub fn get_field_checked(&self, point: &Point) -> Option<&LandscapeField> {
        if point.0 >= self.fields[0].len() || point.1 >= self.fields.len() {
            return None;
        }

        Some(&self.fields[point.1][point.0])
    }

    pub fn hash_point(&self, point: &Point) -> usize {
        point.0 + point.1 * self.fields[0].len()
    }

    pub fn new(fields: Vec<Vec<LandscapeField>>) -> Landscape {
        let start = fields.iter()
            .flatten()
            .enumerate()
            .find_map(|(i, field)| match field {
                LandscapeField::Start => Some((i % fields[0].len(), i / fields[0].len())),
                _ => None,
            })
            .expect("Start field should be present");
        Landscape { fields, start }
    }
}
