use super::board::Board;
use super::{Puz, Size, Token};
use crate::color;
use byte_unit::Byte;
use std::mem::size_of;

impl Puz {
    pub fn usage() {
        println!(
            "{B}usage:   {G}./{file}{C} {B}{CY}<heuristic>{C} {B}{BL}<strategy>{C} {B}{M}<initial_state>{C} {B}{M}{D}<goal_state>{C} {B}{R}{D}<stop_at_first_solution>{C}

	{B}{CY}<heuristic>:{C} {B}{M}manhattan{C} {B}| {M}hamming{C} {B}| {M}linear_conflict{C} {B}| {M}euclidean{C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}manhattan{C} {I}| {Y}first letter is enough{C}{I}{B}){C}

	{B}{BL}<strategy>:{C} {B}{M}astar{C} {B}| {M}greedy{C} {B}| {M}uniform{C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}astar{C}     {I}| {Y}first letter is enough{C}{I}{B}){C}

	{B}{M}<initial_state>:{C} {B}{M}{U}file{C} {B}| {M}{U}number{C}{B}:{C} {I}(random n-puzzle of size {M}{B}{U}n{C}{I}){C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}3{C}){C}

	{B}{M}{D}<goal_state>:{C} {B}{M}{U}file{C} {B}| {M}snail{C} {B}| {M}classic{C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}snail{C}     {I}| {Y}first letter is enough{C}{I}{B}){C}

	{B}{R}{D}<stop_at_first_solution>:{C} {B}{M}true{C} {B}| {M}false{C}
		{I}{B}({Y}optional{C} {I}| {M}default: {B}true{C}      {I}| {Y}first letter is enough{C}{I}{B}){C}

{B}default: {G}./{file}{C} {B}{CY}manhattan{C} {B}{BL}astar{C} {B}{M}3{C} {B}{M}{D}snail{C} {B}{R}{D}true{C}",
            file = std::env::args().next().unwrap(),
            C = color::CLEAR,
            B = color::BOLD,
            I = color::ITALIC,
            D = color::DIM,
            U = color::UNDERLINE,
			R = color::RED,
            G = color::GRE,
            Y = color::YEL,
            BL = color::BLU,
            M = color::MAG,
            CY = color::CYA,
        );
    }

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
