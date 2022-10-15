use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{SystemTime, UNIX_EPOCH};

use self::target_type::get_target_snail;

mod board;
mod graphic;
pub mod heuristic;
mod is_solvable;
mod r#move;
mod read;
mod solve;
pub mod target_type;

type Token = u16;
pub type Size = u8;

pub enum Strategy {
    AStar,
    Greedy,
    Uniform,
}

pub struct Puz {
    _size: Size,
    _board: Vec<Token>,
    _target: Vec<Token>,
    _solution: Vec<r#move::Move>,
    _max_open: usize,
    _open_at_end: usize,
    _closed_at_end: usize,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    _heuristic: fn(&[Token], Size, &[Token]) -> u32,
    _strategy: Strategy,
    _stop_at_first_solution: bool,
}

impl Puz {
    pub fn new() -> Puz {
        Puz {
            _size: 0,
            _board: Vec::new(),
            _target: Vec::new(),
            _solution: Vec::new(),
            _max_open: 0,
            _open_at_end: 0,
            _closed_at_end: 0,
            start_time: UNIX_EPOCH,
            end_time: UNIX_EPOCH,
            _heuristic: heuristic::manathan_distance,
            _strategy: Strategy::AStar,
            _stop_at_first_solution: true,
        }
    }

    pub fn from(size: Size) -> Puz {
        let mut board = (0..(size as Token).pow(2)).collect::<Vec<Token>>();
        board.shuffle(&mut thread_rng());

        Puz {
            _size: size,
            _board: board,
            _target: Vec::new(),
            _solution: Vec::new(),
            _max_open: 0,
            _open_at_end: 0,
            _closed_at_end: 0,
            start_time: UNIX_EPOCH,
            end_time: UNIX_EPOCH,
            _heuristic: heuristic::manathan_distance,
            _strategy: Strategy::AStar,
            _stop_at_first_solution: true,
        }
    }

    pub fn set_target(&mut self, target: &[Token]) {
        self._target = target.to_vec();
    }

    pub fn set_heuristic(&mut self, heuristic: fn(&[Token], Size, &[Token]) -> u32) {
        self._heuristic = heuristic;
    }

    pub fn set_strategy(&mut self, strategy: Strategy) {
        self._strategy = strategy;
    }

    pub fn set_stop_at_first_solution(&mut self, stop_at_first_solution: bool) {
        self._stop_at_first_solution = stop_at_first_solution;
    }

    pub fn get_size(&self) -> Size {
        self._size
    }

    pub fn get_xy(index: Token, size: Size) -> (Size, Size) {
        return (
            (index % size as Token) as Size,
            (index / size as Token) as Size,
        );
    }
}
