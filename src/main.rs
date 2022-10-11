mod puz;
use puz::Puz;

fn main() {
    println!("Hello, world!");

    let p = Puz::new(3);

    p.print();
}
