use super::{Puz, Token};

impl Puz {
    pub fn is_solvable(&self) -> bool {
        let mut board_inversions = Self::_inversions(&self._board);
        let mut target_inversions = Self::_inversions(&self._target);

        if self._size % 2 == 0 {
            board_inversions +=
                self._board.iter().position(|&x| x == 0).unwrap() as usize / self._size as usize;
            target_inversions +=
                self._target.iter().position(|&x| x == 0).unwrap() as usize / self._size as usize;
        }

        return board_inversions % 2 == target_inversions % 2;
    }

    fn _inversions(board: &[Token]) -> usize {
        let mut i = 0;
        for x in 0..board.len() - 1 {
            for y in x + 1..board.len() {
                if board[x] == 0 || board[y] == 0 {
                    continue;
                }
                if board[x] > board[y] {
                    i += 1;
                }
            }
        }
        return i;
    }
}
