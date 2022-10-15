use super::board::Board;
use super::{Puz, Size, Token};
use byte_unit::Byte;
use std::mem::size_of;

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

    pub fn print_solution(&self) {
        println!("steps:\t{}", self._solution.len());
        println!("solution:\t{:?}", self._solution);
        println!("open   now:\t{}", self._open_at_end);
        println!("closed now:\t{}", self._closed_at_end);
        println!(
            "open   max:\t{}\t({})",
            self._max_open,
            Byte::from_bytes((size_of::<Board>() * self._max_open) as u128)
                .get_appropriate_unit(true)
        );
        println!("open   total:\t{}", self._open_at_end + self._closed_at_end);
    }
}
