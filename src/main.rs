mod puz;
use puz::Puz;

fn main() {
    let mut p = Puz::new(3);

    p.print();

    println!("Solving...");
    p.solve();
    println!("Done!");

    p.print();
}
