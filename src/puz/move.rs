use super::{Board, Puz, Size, Token};

#[derive(Copy, Clone)]
pub enum Move {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Move {
    pub fn get_opposite(m: Move) -> Move {
        match m {
            Move::Up => Move::Down,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
            Move::Right => Move::Left,
        }
    }
}

pub struct AllowedMove {
    pub m: Move,
    pub a: bool,
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

impl Puz {
    // pub fn play_move(&mut self, m: Move) {
    //     let (x, y) = self.get_blank_xy();
    //     let mut new_x = x;
    //     let mut new_y = y;
    //     match m {
    //         Move::Up => new_y -= 1,
    //         Move::Down => new_y += 1,
    //         Move::Left => new_x -= 1,
    //         Move::Right => new_x += 1,
    //     }
    //     self._board.swap(
    //         (x + y * self._size) as usize,
    //         (new_x + new_y * self._size) as usize,
    //     );
    //     self._blank = new_x as Token + new_y as Token * self._size as Token;
    // }

    pub fn update_allowed_move(&self, m: &mut [AllowedMove; 4]) {
        let (x, y) = self.get_blank_xy();
        m[Move::Up as usize].a = y > 0;
        m[Move::Down as usize].a = y < self._size - 1;
        m[Move::Left as usize].a = x > 0;
        m[Move::Right as usize].a = x < self._size - 1;
    }
}

impl Board {
    pub fn play_move(board: &Board, size: Size, m: Move) -> Board {
        let (x, y) = Puz::get_xy(board._blank, size);
        let mut new_x = x;
        let mut new_y = y;
        match m {
            Move::Up => new_y -= 1,
            Move::Down => new_y += 1,
            Move::Left => new_x -= 1,
            Move::Right => new_x += 1,
        }
        let mut new_board = board._board.clone();
        new_board.swap((x + y * size) as usize, (new_x + new_y * size) as usize);
        return Board {
            _board: new_board,
            _blank: new_x as Token + new_y as Token * size as Token,
        };
    }
}
