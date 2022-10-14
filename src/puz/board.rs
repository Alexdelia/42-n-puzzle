use super::r#move::Move;
use super::Token;

pub struct Board {
    pub board: Vec<Token>,
    pub blank: Token,
    pub score: u32,
    pub solution: Vec<Move>,
    pub sol_len: usize,
}

impl Board {
    pub fn get_blank_index(board: &[Token]) -> Token {
        for (i, t) in board.iter().enumerate() {
            if *t == 0 {
                return i as Token;
            }
        }
        return Token::MAX;
    }
}
