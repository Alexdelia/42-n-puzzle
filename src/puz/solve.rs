use super::r#move::{AllowedMove, Move};
use super::{Board, Puz, Size, Token};

impl Puz {
    pub fn solve(&self) {
        let mut open: Vec<Board> = Vec::new();
        let mut closed: Vec<Board> = Vec::new();
    }

    pub fn manathan_distance(&self, target: &[Token]) -> u32 {
        let size = self._size as Token;
        let mut distance = 0;

        for i in 0..size.pow(2) {
            let mut x = 0;
            let mut y = 0;
            let mut target_x = 0;
            let mut target_y = 0;

            for f in 0..size.pow(2) {
                if self._board[f as usize] == i.into() {
                    x = f % size;
                    y = f / size;
                }
                if target[f as usize] == i.into() {
                    target_x = f % size;
                    target_y = f / size;
                }
            }
            distance += (x as i32 - target_x as i32).abs() as u32
                + (y as i32 - target_y as i32).abs() as u32;
        }
        return distance;
    }

    pub fn is_solvable(&self) -> bool {
        let size = self._size as Token;
        let mut inversions = 0;
        let mut blank_row = 0;

        for x in 0..size.pow(2) {
            if self._board[x as usize] == 0 {
                blank_row = x / size;
                continue;
            }
            for y in x..size.pow(2) {
                if self._board[y as usize] == 0 {
                    continue;
                }
                if self._board[x as usize] > self._board[y as usize] {
                    inversions += 1;
                }
            }
        }

        if size % 2 == 1 {
            return inversions % 2 == 1;
        } else {
            return (inversions + blank_row) % 2 == 0;
        }
    }
}
