/*use rand::seq::SliceRandom;
use rand::thread_rng;*/

pub struct Puz {
    _size: u8,
    _board: Vec<u8>,
}

impl Puz {
    pub fn new(size: u8) -> Puz {
        Puz {
            _size: size,
            // will need to generate a random permutation of 1..size^2
            _board: (0..size.pow(2)).collect::<Vec<u8>>(), //.shuffle(&mut thread_rng()),
        }
    }

    pub fn print(&self) {
        for x in 0..self._size {
            for y in 0..self._size {
                print!("{} ", self._board[(x * self._size + y) as usize]);
            }
            println!("");
        }
    }
}
