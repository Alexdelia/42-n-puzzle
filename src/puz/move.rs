use super::board::Board;
use super::{Puz, Size, Token};

// might remove Debug
#[derive(Copy, Clone, Debug)]
pub enum Move {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}
pub struct AllowedMove {
    pub m: Move,
    pub a: bool,
}

impl Move {
    pub fn update_allowed_move(m: &mut [AllowedMove; 4], blank: Token, size: Size) {
        let (x, y) = Puz::get_xy(blank, size);
        m[Move::Up as usize].a = y > 0;
        m[Move::Down as usize].a = y < size - 1;
        m[Move::Left as usize].a = x > 0;
        m[Move::Right as usize].a = x < size - 1;
    }

    pub fn get_opposite(m: Move) -> Move {
        match m {
            Move::Up => Move::Down,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
            Move::Right => Move::Left,
        }
    }
}

impl AllowedMove {
    pub fn new_array() -> [AllowedMove; 4] {
        let mut m: [AllowedMove; 4] = [
            AllowedMove {
                m: Move::Up,
                a: true,
            },
            AllowedMove {
                m: Move::Down,
                a: true,
            },
            AllowedMove {
                m: Move::Left,
                a: true,
            },
            AllowedMove {
                m: Move::Right,
                a: true,
            },
        ];
        m[Move::Up as usize] = AllowedMove {
            m: Move::Up,
            a: true,
        };
        m[Move::Down as usize] = AllowedMove {
            m: Move::Down,
            a: true,
        };
        m[Move::Left as usize] = AllowedMove {
            m: Move::Left,
            a: true,
        };
        m[Move::Right as usize] = AllowedMove {
            m: Move::Right,
            a: true,
        };
        return m;
    }
}

impl Board {
    pub fn play_move(&self, size: Size, m: Move) -> Board {
        let (x, y) = Puz::get_xy(self.blank, size);
        let mut new_x = x;
        let mut new_y = y;
        match m {
            Move::Up => new_y -= 1,
            Move::Down => new_y += 1,
            Move::Left => new_x -= 1,
            Move::Right => new_x += 1,
        }
        let mut new_board = Board {
            board: self.board.clone(),
            blank: new_x as Token + new_y as Token * size as Token,
            score: 0,
            solution: self.solution.clone(),
            sol_len: self.sol_len + 1,
        };
        new_board
            .board
            .swap((x + y * size) as usize, (new_x + new_y * size) as usize);
        new_board.solution.push(m);
        return new_board;
    }
}
