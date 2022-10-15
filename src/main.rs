mod color;
mod parse;
mod puz;
mod usage;
mod utils;

use parse::parse;
use puz::target_type::get_target_snail;
use puz::Puz;
use std::{env::args, process::ExitCode, time::SystemTime};
use usage::usage;

fn init_board() -> Result<Puz, bool> {
    let mut p: Puz;
    let av = args().collect::<Vec<String>>();

    if av.len() > 1 {
        p = match parse() {
            Ok(p) => p,
            Err(_) => return Err(false),
        };
    } else {
        usage();
        p = Puz::from(3);
        p.set_target(&get_target_snail(p.get_size()));
    }

    return Ok(p);
}

fn main() -> ExitCode {
    let mut p = match init_board() {
        Ok(p) => p,
        Err(_) => return ExitCode::FAILURE,
    };

    println!(
        "\n{B}# {G}initial state{C} {B}#{C}",
        C = color::CLEAR,
        B = color::BOLD,
        G = color::GRE,
    );
    p.print();
    println!(
        "\n{B}#  {CY}goal state{C}   {B}#{C}",
        C = color::CLEAR,
        B = color::BOLD,
        CY = color::CYA,
    );
    p.print_target();
    println!();

    if !p.is_solvable() {
        println!(
            "{B}initial state is {Y}not{C} {B}solvable{C}",
            C = color::CLEAR,
            B = color::BOLD,
            Y = color::YEL
        );
        if p.get_stop() {
            return ExitCode::FAILURE;
        }
    }

    p.start_time = SystemTime::now();
    let solution = p.solve();
    p.end_time = SystemTime::now();
    p.print_solution(solution);
    println!();

    return ExitCode::SUCCESS;
}
