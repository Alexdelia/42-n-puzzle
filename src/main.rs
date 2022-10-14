mod color;
mod puz;
mod utils;

use puz::target_type::get_target_snake;
use puz::Puz;
use std::{env::args, process::ExitCode};

fn init_board() -> Result<Puz, bool> {
    let mut p: Puz;
    let av = args().collect::<Vec<String>>();
    dbg!(av.clone());

    if av.len() > 1 {
        p = Puz::new();
        if !p.read(&av[1]) {
            return Err(false);
        }
        p.set_target(get_target_snake(p.get_size()));
    } else {
        p = Puz::from(3);
    }

    return Ok(p);
}

fn main() -> ExitCode {
    let mut p = match init_board() {
        Ok(p) => p,
        Err(_) => return ExitCode::FAILURE,
    };

    if !p.is_solvable() {
        p.print();
        println!(
            "{B}initial state is {Y}not{C} {B}solvable{C}",
            C = color::CLEAR,
            B = color::BOLD,
            Y = color::YEL
        );
        return ExitCode::FAILURE;
    }

    p.print();

    println!("Solving...");
    p.solve();
    println!("Done!");

    p.print();

    return ExitCode::SUCCESS;
}
