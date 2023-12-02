pub struct Game {
    pub id: u8,
    pub plays: Vec<Play>,
}

pub struct Play {
    pub cubes: Vec<Cube>,
}

pub struct Cube {
    pub color: Color,
    pub amount: u8,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    RED,
    GREEN,
    BLUE,
}

impl Game {
    pub fn get_cube_amount(self: &Game, color: Color) -> u8 {
        self.plays.iter()
            .map(|play| {
                play.cubes.iter()
                    .filter(|cube| cube.color == color)
                    .map(|cube| cube.amount)
                    .sum::<u8>()
            })
            .max()
            .unwrap()
    }
}

