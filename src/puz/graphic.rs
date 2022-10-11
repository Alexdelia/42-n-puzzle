use super::Puz;
use crate::puz::{Size, Token};

impl Puz {
    pub fn print(&self) {
        for x in 0..self._size {
            for y in 0..self._size {
                print!("{} ", self._board[(x * self._size + y) as usize]);
            }
            println!("");
        }
    }

    pub fn print_other(board: &Vec<Token>, size: Size) {
        for x in 0..size {
            for y in 0..size {
                print!("{} ", board[(x * size + y) as usize]);
            }
            println!("");
        }
    }
}
