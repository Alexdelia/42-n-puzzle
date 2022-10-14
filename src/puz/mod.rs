/*use rand::seq::SliceRandom;
use rand::thread_rng;*/

mod graphic;
mod r#move;
mod read;
mod solve;
pub mod target_type;

type Token = u16;
type Size = u8;

pub struct Puz {
    _size: Size,
    _board: Vec<Token>,
    _target: Vec<Token>,
    _solvable: bool,
    _solution: Vec<r#move::Move>,
}

impl Puz {
    pub fn new() -> Puz {
        Puz {
            _size: 0,
            _board: Vec::new(),
            _target: Vec::new(),
            _solvable: true,
            _solution: Vec::new(),
        }
    }

    pub fn from(size: Size) -> Puz {
        Puz {
            _size: size,
            // will need to generate a random permutation of 1..size^2
            _board: (0..(size as Token).pow(2)).collect::<Vec<Token>>(), //.shuffle(&mut thread_rng()),
            _target: target_type::get_target_snake(size),
            _solvable: true, // will need to decide if use is_solvable()
            _solution: Vec::new(),
        }
    }

    pub fn set_target(&mut self, target: Vec<Token>) {
        self._target = target;
    }

    pub fn get_size(&self) -> Size {
        self._size
    }
}
