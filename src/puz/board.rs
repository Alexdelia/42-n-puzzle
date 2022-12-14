use super::r#move::Move;
use super::Token;

#[derive(Eq, PartialEq)]
pub struct Board {
    pub board: Vec<Token>,
    pub blank: Token,
    pub score: u32,
    pub solution: Vec<Move>,
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Board {
    pub fn get_blank_index(board: &[Token]) -> Token {
        for (i, t) in board.iter().enumerate() {
            if *t == 0 {
                return i as Token;
            }
        }
        Token::MAX
    }
}
