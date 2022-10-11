pub struct Puz {
    _size: u8,
    _board: Vec<u8>,
}

impl Puz {
    fn new(size: u8) -> Puz {
        Puz {
            _size: size,
            // will need to generate a random permutation of 1..size^2
            _board: vec![0; size.pow(2) as usize],
        }
    }

    pub fn print(&self) {
        println!("Puzzle:");
        for x in 0..self._size {
            for y in 0..self._size {
                print!("{} ", self._board[(x * self._size + y) as usize]);
            }
            println!("");
        }
    }
}
