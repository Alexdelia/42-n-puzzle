mod color;
mod puz;
mod utils;

use puz::target_type::get_target_snail;
use puz::Puz;
use std::{env::args, process::ExitCode, time::SystemTime};

fn init_board() -> Result<Puz, bool> {
    let mut p: Puz;
    let av = args().collect::<Vec<String>>();
    dbg!(av.clone());

    if av.len() > 1 {
        p = Puz::new();
        if !p.read(&av[1], false) {
            return Err(false);
        }
    } else {
        Puz::usage();
        p = Puz::from(3);
    }
    p.set_target(&get_target_snail(p.get_size()));

    return Ok(p);
}

fn main() -> ExitCode {
    let mut p = match init_board() {
        Ok(p) => p,
        Err(_) => return ExitCode::FAILURE,
    };

    p.print();

    if !p.is_solvable() {
        println!(
            "{B}initial state is {Y}not{C} {B}solvable{C}",
            C = color::CLEAR,
            B = color::BOLD,
            Y = color::YEL
        );
        return ExitCode::FAILURE;
    }

    println!("Solving...");
    p.start_time = SystemTime::now();
    let solution = p.solve(true);
    p.end_time = SystemTime::now();
    println!("\nDone!");
    p.print_solution();
    println!("solution: {}", solution);

    return ExitCode::SUCCESS;
}
