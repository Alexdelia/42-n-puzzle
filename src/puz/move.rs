use super::Puz;

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn get_opposite(&self) -> Move {
        match self {
            Move::Up => Move::Down,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
            Move::Right => Move::Left,
        }
    }
}

impl Puz {
    fn _move(&mut self, m: Move) {
        let size = self._size as usize;
        let blank_index = self._blank.y as usize * size + self._blank.x as usize;
        match m {
            Move::Up => {
                if self._blank.y == 0 {
                    return;
                }
                self._board.swap(blank_index, blank_index - size);
                self._blank.y -= 1;
            }
            Move::Down => {
                if self._blank.y == self._size - 1 {
                    return;
                }
                self._board.swap(blank_index, blank_index + size);
                self._blank.y += 1;
            }
            Move::Left => {
                if self._blank.x == 0 {
                    return;
                }
                self._board.swap(blank_index, blank_index - 1);
                self._blank.x -= 1;
            }
            Move::Right => {
                if self._blank.x == self._size - 1 {
                    return;
                }
                self._board.swap(blank_index, blank_index + 1);
                self._blank.x += 1;
            }
        }
    }
}
