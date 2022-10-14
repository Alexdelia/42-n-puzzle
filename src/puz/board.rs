use super::r#move::Move;
use super::Token;

pub struct Board {
    pub board: Vec<Token>,
    pub blank: Token,
    pub distance: u32,
    pub solution: Vec<Move>,
}
