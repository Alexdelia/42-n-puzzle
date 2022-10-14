mod color;
mod puz;
mod utils;

use puz::target_type::get_target_snake;
use puz::Puz;
use std::{env::args, process::ExitCode};

fn main() -> ExitCode {
    let mut p: Puz;
    let av = args().collect::<Vec<String>>();
    dbg!(av.clone());

    if av.len() > 1 {
        p = Puz::new();
        if !p.read(&av[1]) {
            return ExitCode::FAILURE;
        }
        p.set_target(get_target_snake(p.get_size()));
    } else {
        p = Puz::from(3);
    }

    p.print();

    println!("Solving...");
    p.solve();
    println!("Done!");

    p.print();

    return ExitCode::SUCCESS;
}
