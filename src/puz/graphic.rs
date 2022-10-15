use super::board::Board;
use super::r#move::Move;
use super::{Puz, Size, Token};
use crate::{color, err_no};
use byte_unit::Byte;
use std::collections::btree_set::Difference;
use std::mem::size_of;
use std::time::Duration;

impl Puz {
    pub fn print(&self) {
        Self::print_other(&self._board, self._size);
    }

    pub fn print_target(&self) {
        Self::print_other(&self._target, self._size);
    }

    pub fn print_other(board: &[Token], size: Size) {
        let size = size as usize;
        for x in 0..size {
            for y in 0..size {
                print!("{} ", board[x * size + y]);
            }
            println!("");
        }
    }

    pub fn print_solution(&self, found: bool) {
        let b_open_now = (self._open_at_end * size_of::<Board>()) as u128;
        let b_closed_now = (self._closed_at_end * size_of::<Vec<Token>>()) as u128;
        let b_open_max = (self._max_open * size_of::<Board>()) as u128;
        let difference: Duration;

        match self.end_time.duration_since(self.start_time) {
            Ok(d) => difference = d,
            Err(e) => {
                err_no!(
                    "time went backward!?\n({:?} -> {:?})\n\t{}",
                    self.start_time,
                    self.end_time,
                    e
                );
                difference = std::time::Duration::new(0, 0);
            }
        };

        println!("####################################",);
        if found {
            println!(
                "      {G}steps{C}{B}: {G}{s}{C}",
                s = self._solution.len(),
                C = color::CLEAR,
                B = color::BOLD,
                G = color::GRE
            );
            self.print_solution_arrow();
        } else if self._solution.is_empty() {
            println!(
                "\t{B}{R}no solution found{C}",
                C = color::CLEAR,
                B = color::BOLD,
                R = color::RED
            );
        } else {
            println!(
                "\t{B}{G}conclusion{C}",
                C = color::CLEAR,
                B = color::BOLD,
                G = color::GRE
            );
        }
        println!();
        println!(
            "   {G}{D}open{C} now{B}: {G}{D}{n}\t{C}{I}({B}{M}{b}{C}{I}){C}",
            n = self._open_at_end,
            b = Byte::from_bytes(b_open_now)
                .get_appropriate_unit(true)
                .format(1),
            C = color::CLEAR,
            B = color::BOLD,
            D = color::DIM,
            I = color::ITALIC,
            G = color::GRE,
            M = color::MAG,
        );
        println!(
            " {R}{D}closed{C} now{B}: {R}{D}{n}\t{C}{I}({B}{M}{b}{C}{I}){C}",
            n = self._closed_at_end,
            b = Byte::from_bytes(b_closed_now)
                .get_appropriate_unit(true)
                .format(1),
            C = color::CLEAR,
            B = color::BOLD,
            D = color::DIM,
            I = color::ITALIC,
            R = color::RED,
            M = color::MAG,
        );
        println!(
            "   {G}{D}open{C} max{B}: {G}{D}{n}\t{C}{I}({B}{M}{b}{C}{I}){C}",
            n = self._max_open,
            b = Byte::from_bytes(b_open_max)
                .get_appropriate_unit(true)
                .format(1),
            C = color::CLEAR,
            B = color::BOLD,
            D = color::DIM,
            I = color::ITALIC,
            G = color::GRE,
            M = color::MAG,
        );
        println!(
            " {G}{D}open{C} total{B}: {BL}{n}\t{C}{I}({B}{M}{b}{C}{I}){C}",
            n = self._open_at_end + self._closed_at_end,
            b = Byte::from_bytes(b_open_now + b_closed_now)
                .get_appropriate_unit(true)
                .format(1),
            C = color::CLEAR,
            B = color::BOLD,
            D = color::DIM,
            I = color::ITALIC,
            G = color::GRE,
            BL = color::BLU,
            M = color::MAG,
        );
        println!();
        println!(
            "       {CY}time{C}{B}: {CY}{n:?}{C}",
            n = difference,
            C = color::CLEAR,
            B = color::BOLD,
            CY = color::CYA,
        );
        print!("####################################\r");
    }

    fn print_solution_arrow(&self) {
        let up = "\x1b[38;2;167;84;134m↑\x1b[0m";
        let down = "\x1b[38;2;80;95;144m↓\x1b[0m";
        let left = "\x1b[38;2;122;184;92m←\x1b[0m";
        let right = "\x1b[38;2;212;180;106m→\x1b[0m";

        print!(
            "   {G}solution{C}{B}:{C} ",
            C = color::CLEAR,
            B = color::BOLD,
            G = color::GRE
        );
        for d in self._solution.iter() {
            match d {
                Move::Up => print!("{}", up),
                Move::Down => print!("{}", down),
                Move::Left => print!("{}", left),
                Move::Right => print!("{}", right),
            }
        }
        println!();
    }
}
