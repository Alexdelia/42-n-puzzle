use super::board::Board;
use super::r#move::{AllowedMove, Move};
use super::{Puz, Size, Token};

impl Puz {
    pub fn solve(&self) -> Vec<Move> {
        let mut open: Vec<Board> = Vec::new();
        let mut closed: Vec<Vec<Token>> = Vec::new();
        let mut found_solution: Vec<Move> = Vec::new();
        let mut sol_len: usize = 0;
        let mut current: usize;
        let mut allowed_move: [AllowedMove; 4] = AllowedMove::new_array();

        open.push(Board {
            board: self._board.clone(),
            blank: self._blank,
            distance: 0,
            solution: Vec::new(),
        });

        while open.len() > 0 {
            current = Self::_min_distance_index(&open);
            if open[current].board == self._target {
                if sol_len == 0 || open[current].solution.len() < sol_len {
                    found_solution = open[current].solution.clone();
                    sol_len = found_solution.len();
                    println!("found solution: {} {:?}", sol_len, found_solution);
                }
                open.remove(current);
                continue;
            }
            if sol_len > 0 && open[current].solution.len() >= sol_len {
                closed.push(open[current].board.clone());
                open.remove(current);
                continue;
            }
            if closed.contains(&open[current].board) {
                open.remove(current);
                continue;
            }

            Move::update_allowed_move(&mut allowed_move, open[current].blank, self._size);
            for m in allowed_move.iter() {
                if m.a {
                    let mut new_board = open[current].play_move(self._size, m.m);
                    if !closed.contains(&new_board.board) {
                        new_board.distance =
                            Self::manathan_distance(&new_board.board, self._size, &self._target);
                        open.push(new_board);
                    }
                }
            }

            closed.push(open[current].board.clone());
            open.remove(current);
        }

        return found_solution;
    }

    fn _min_distance_index(open: &Vec<Board>) -> usize {
        let mut min_distance = open[0].distance;
        let mut min_index = 0;
        for i in 1..open.len() {
            if open[i].distance < min_distance {
                min_distance = open[i].distance;
                min_index = i;
            }
        }
        return min_index;
    }

    pub fn manathan_distance(board: &[Token], size: Size, target: &[Token]) -> u32 {
        let size: Token = size as Token;
        let mut distance = 0;

        for i in 0..size.pow(2) {
            let mut x = 0;
            let mut y = 0;
            let mut target_x = 0;
            let mut target_y = 0;

            for f in 0..size.pow(2) {
                if board[f as usize] == i.into() {
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
