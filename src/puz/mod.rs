/*use rand::seq::SliceRandom;
use rand::thread_rng;*/

mod graphic;
mod r#move;
mod read;
mod solve;
pub mod target_type;

type Token = u16;
type Size = u8;

struct Coord {
    x: Size,
    y: Size,
}

pub struct Puz {
    _size: Size,
    _board: Vec<Token>,
    _target: Vec<Token>,
    _solution: Vec<r#move::Move>,
    _blank: Coord,
}

impl Puz {
    pub fn new() -> Puz {
        Puz {
            _size: 0,
            _board: Vec::new(),
            _target: Vec::new(),
            _solution: Vec::new(),
            _blank: Coord { x: 0, y: 0 },
        }
    }

    pub fn from(size: Size) -> Puz {
        let mut p = Puz {
            _size: size,
            // will need to generate a random permutation of 1..size^2
            _board: (0..(size as Token).pow(2)).collect::<Vec<Token>>(), //.shuffle(&mut thread_rng()),
            _target: target_type::get_target_snake(size),
            _solution: Vec::new(),
            _blank: Coord { x: 0, y: 0 },
        };
        p._update_blank();
        return p;
    }

    fn _update_blank(&mut self) {
        let size = self._size as Token;
        for i in 0..size.pow(2) {
            if self._board[i as usize] == 0 {
                self._blank.x = (i % size) as Size;
                self._blank.y = (i / size) as Size;
                break;
            }
        }
    }

    pub fn set_target(&mut self, target: Vec<Token>) {
        self._target = target;
    }

    pub fn get_size(&self) -> Size {
        self._size
    }
}
