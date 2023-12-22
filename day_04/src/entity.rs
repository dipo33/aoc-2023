#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winning: Vec<u8>,
    pub guesses: Vec<u8>,
}
