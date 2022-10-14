/*use rand::seq::SliceRandom;
use rand::thread_rng;*/

mod board;
mod graphic;
mod heuristic;
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
}

impl Puz {
    pub fn new() -> Puz {
        Puz {
            _size: 0,
            _board: Vec::new(),
            _target: Vec::new(),
        }
    }

    pub fn from(size: Size) -> Puz {
        let mut p = Puz {
            _size: size,
            // will need to generate a random permutation of 1..size^2
            _board: (0..(size as Token).pow(2)).collect::<Vec<Token>>(), //.shuffle(&mut thread_rng()),
            _target: Vec::new(),
        };
        return p;
    }

    pub fn set_target(&mut self, target: &[Token]) {
        self._target = target.to_vec();
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
