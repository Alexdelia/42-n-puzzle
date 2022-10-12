mod puz;
mod utils;

use puz::Puz;
use std::env::args;

fn main() {
    let av = args().collect::<Vec<String>>();
    dbg!(av.clone());

    if av.len() > 1 {
        let mut p = Puz::from(av[1].clone());
        return ();
    }

    let mut p = Puz::new(3);

    p.print();

    println!("Solving...");
    p.solve();
    println!("Done!");

    p.print();
}
