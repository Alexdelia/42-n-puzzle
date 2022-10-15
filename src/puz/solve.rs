use std::collections::{BinaryHeap, HashSet};
use std::time::SystemTime;

use super::board::Board;
use super::r#move::{AllowedMove, Move};
use super::{Puz, Strategy, Token};

impl Puz {
    pub fn solve(&mut self) -> bool {
        let mut open = BinaryHeap::<Board>::new();
        let mut closed = HashSet::<Vec<Token>>::new();
        let mut allowed_move: [AllowedMove; 4] = AllowedMove::new_array();
        let heuristic = self._heuristic;

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
                self.end_time = SystemTime::now();
                if self._stop_at_first_solution {
                    return true;
                }
                self.print_solution(true);
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
                        new_board.score = self._score(
                            new_board.solution.len() as u32,
                            heuristic(&new_board.board, self._size, &self._target),
                        );
                        open.push(new_board);
                    }
                }
            }

            closed.insert(cur.board.clone());

            if open.len() > self._max_open {
                self._max_open = open.len();
            }
            print!(
                "open: {}   closed: {}                \r",
                open.len(),
                closed.len()
            );
        }

        self._open_at_end = open.len();
        self._closed_at_end = closed.len();
        return false;
    }

    fn _score(&self, g: u32, h: u32) -> u32 {
        match self._strategy {
            Strategy::AStar => g + h,
            Strategy::Greedy => h,
            Strategy::Uniform => g,
        }
    }
}
