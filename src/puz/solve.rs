use std::collections::{BinaryHeap, HashSet};

use super::board::Board;
use super::heuristic::manathan_distance;
use super::r#move::{AllowedMove, Move};
use super::{Puz, Token};

impl Puz {
    pub fn solve(&mut self, stop: bool) -> bool {
        let mut open = BinaryHeap::<Board>::new();
        let mut closed = HashSet::<Vec<Token>>::new();
        let mut allowed_move: [AllowedMove; 4] = AllowedMove::new_array();

        open.push(Board {
            board: self._board.clone(),
            blank: Board::get_blank_index(&self._board),
            score: 0,
            solution: Vec::new(),
        });

        while open.is_empty() == false {
            let cur = open.pop().unwrap();
            if cur.board == self._target
                && (self._solution.is_empty() || cur.solution.len() < self._solution.len())
            {
                self._solution = cur.solution;
                self._open_at_end = open.len();
                self._closed_at_end = closed.len();
                if stop {
                    return true;
                }
                self.print_solution();
                closed.insert(cur.board.clone());
                continue;
            }
            if !self._solution.is_empty() && cur.solution.len() >= self._solution.len() {
                closed.insert(cur.board.clone());
                continue;
            }
            if closed.contains(&cur.board) {
                continue;
            }

            Move::update_allowed_move(&mut allowed_move, cur.blank, self._size);
            for m in allowed_move.iter() {
                if m.a {
                    let mut new_board = cur.play_move(self._size, m.m);
                    if !closed.contains(&new_board.board) {
                        new_board.score = new_board.solution.len() as u32
                            + manathan_distance(&new_board.board, self._size, &self._target);
                        open.push(new_board);
                    }
                }
            }

            closed.insert(cur.board.clone());

            if open.len() > self._max_open {
                self._max_open = open.len();
            }
            print!("\ropen: {}  closed: {}", open.len(), closed.len());
        }

        self._open_at_end = open.len();
        self._closed_at_end = closed.len();
        return false;
    }

    fn _min_score_index(open: &Vec<Board>) -> usize {
        let mut min_score = open[0].score;
        let mut min_index = 0;
        for i in 1..open.len() {
            if open[i].score < min_score {
                min_score = open[i].score;
                min_index = i;
            }
        }
        return min_index;
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
