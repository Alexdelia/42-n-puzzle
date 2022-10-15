use super::board::Board;
use super::{Puz, Size, Token};
use byte_unit::Byte;
use std::mem::size_of;

impl Puz {
    pub fn print(&self) {
        let size = self._size as usize;
        for x in 0..size {
            for y in 0..size {
                print!("{} ", self._board[x * size + y]);
            }
            println!("");
        }
    }

    pub fn print_other(board: &Vec<Token>, size: Size) {
        let size = size as usize;
        for x in 0..size {
            for y in 0..size {
                print!("{} ", board[x * size + y]);
            }
            println!("");
        }
    }

    pub fn print_solution(&self) {
        let b_open_now = (self._max_open * size_of::<Board>()) as u128;
        let b_closed_now = (self._closed_at_end * size_of::<Vec<Token>>()) as u128;
        let b_open_max = (self._open_at_end * size_of::<Board>()) as u128;
        let difference = self
            .end_time
            .duration_since(self.start_time)
            .expect("clock may have gone backwards");

        println!("steps:\t{}", self._solution.len());
        println!("solution:\t{:?}", self._solution);
        println!(
            "open   now:\t{}\t({})",
            self._open_at_end,
            Byte::from_bytes(b_open_now)
                .get_appropriate_unit(true)
                .format(1)
        );
        println!(
            "closed now:\t{}\t({})",
            self._closed_at_end,
            Byte::from_bytes(b_closed_now)
                .get_appropriate_unit(true)
                .format(1)
        );
        println!(
            "open   max:\t{}\t({})",
            self._max_open,
            Byte::from_bytes(b_open_max)
                .get_appropriate_unit(true)
                .format(1)
        );
        println!(
            "open   total:\t{}\t({})",
            self._open_at_end + self._closed_at_end,
            Byte::from_bytes(b_open_now + b_closed_now)
                .get_appropriate_unit(true)
                .format(1)
        );
        println!("time:\t{:?}", difference);
    }
}
