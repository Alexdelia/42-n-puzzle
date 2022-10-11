/*use rand::seq::SliceRandom;
use rand::thread_rng;*/

mod graphic;
mod r#move;
mod solve;
mod target_type;

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
    pub fn new(size: Size) -> Puz {
        Puz {
            _size: size,
            // will need to generate a random permutation of 1..size^2
            _board: (0..size.pow(2) as Token).collect::<Vec<Token>>(), //.shuffle(&mut thread_rng()),
            _target: target_type::get_target_snake(size),
            _solvable: true, // will need to decide if use is_solvable()
            _solution: Vec::new(),
        }
    }

    pub fn set_target(&mut self, target: Vec<Token>) {
        self._target = target;
    }
}
